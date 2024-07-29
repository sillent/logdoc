use clap::Parser;

use crate::args;
use crate::files;
use crate::meta::Description;
use crate::meta::Level;
use crate::meta::Message;
use crate::meta::Meta;
use crate::meta::Pos;
use crate::meta::Subject;
use crate::meta::Typo;

pub struct Application;

impl Application {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let arg = args::Arg::parse();
        println!("info desc = {:?}", arg.info_desc);
        let mut parse = tree_sitter::Parser::new();
        let lang = crate::language::Language::from(&arg.language);
        parse.set_language(&lang.sitter_language()).or(Err(format!(
            "Failed to load {} tree-sitter language",
            &lang
        )))?;
        // println!("processed files = {:?}", arg.files_list());
        // let argclone = arg.clone();
        // let k = files::list_files(&arg.directories.unwrap(), false);
        // let k = files::list_files(&arg.directories_ref(), false);
        let files = files::form_list_files(&arg)?;
        println!("processed files = {:?}", files);

        let query = tree_sitter::Query::new(&lang.sitter_language(), &lang.query())?;
        // .unwrap();
        // let files = files::proceed(&arg);
        // let files = arg.files_list();
        for file in files {
            println!("path = {file}");
            let file_bytes = std::fs::read_to_string(file)?;
            let tree = parse
                .parse(&file_bytes.as_bytes(), None)
                .ok_or("Failed to parse data")?;
            let mut query_cursor = tree_sitter::QueryCursor::new();
            let query_matches =
                query_cursor.matches(&query, tree.root_node(), file_bytes.as_bytes());
            for query_match in query_matches {
                let mut positions = vec![];
                let mut m = Meta::default();
                for query_capture in query_match.captures {
                    let position = Pos::from(query_capture);
                    let query_bytes = files::search_in_file_dyn(&file_bytes.as_bytes(), &position);
                    if position.typo == Typo::Level {
                        let data = String::from_utf8_lossy(&query_bytes);
                        let level = Level::from(&data.to_string());
                        m.level = level;
                        m.message = Message::from((&data.to_string(), lang.comment()));
                    }
                    if position.typo == Typo::Subject {
                        let data = String::from_utf8_lossy(&query_bytes);
                        m.subject = Subject::from((&data.to_string(), lang.comment()));
                    }
                    if position.typo == Typo::Description {
                        let data = String::from_utf8_lossy(&query_bytes);
                        let desc = Description::from((&data.to_string(), lang.comment()));
                        let v = vec![m.description.0.clone(), desc.0];
                        let v = Description::from((&v.join("").to_string(), lang.comment()));
                        // let res
                        m.description = v;

                        // m.description.0.
                    }
                    // println!("meta = {:?}", m);
                    positions.push(position);
                    // let p1 = files::search_in_file(res.as_bytes(), &position);
                    // println!("p1 = {query_bytes:?}");
                }
                // let meta = form_meta(positions);
                // let meta = MetaPos::from(positions);
                // println!("metapos = {meta:?}");
                let t = files::write_to_file(m, &arg);
                if let Err(e) = t {
                    println!("error happened: {e:?}");
                }
                // println!("meta = {m:?}");
                println!("-----");
                // println!("{:?}", q.captures);
                // println!("------");

                // for qc in q.captures {}
            }
        }
        Ok(())
    }
}

use clap::Parser;
use tree_sitter::QueryMatches;

use crate::args;
use crate::files;
use crate::meta::form_meta;
use crate::meta::Pos;

pub struct Application;

impl Application {
    pub fn run() -> Result<(), Box<dyn std::error::Error>> {
        let arg = args::Arg::parse();
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
        println!("processed files = {:?}", files);

        let query = tree_sitter::Query::new(&lang.sitter_language(), &lang.query())?;
        // .unwrap();
        // let files = files::proceed(&arg);
        // let files = arg.files_list();
        for file in files {
            println!("path = {file}");
            let res = std::fs::read_to_string(file)?;
            let tree = parse
                .parse(&res.as_bytes(), None)
                .ok_or("Failed to parse data")?;
            let mut query_cursor = tree_sitter::QueryCursor::new();
            let query_matches = query_cursor.matches(&query, tree.root_node(), res.as_bytes());
            for query_match in query_matches {
                let mut positions = vec![];
                for query_capture in query_match.captures {
                    let position = Pos::from(query_capture);
                    positions.push(position.clone());
                    // let p1 = files::search_in_file(res.as_bytes(), &position);
                    let p1 = files::search_in_file_dyn(&res.as_bytes(), &position);
                    println!("p1 = {p1:?}");
                }
                let meta = form_meta(positions);
                println!("meta = {meta:?}");
                println!("-----");
                // println!("{:?}", q.captures);
                // println!("------");

                // for qc in q.captures {}
            }
        }
        Ok(())
    }
}

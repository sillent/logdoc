use clap::Parser;

use crate::args;
use crate::files;
use crate::meta::form_meta;
use crate::meta::Pos;
use crate::queries::*;
pub struct Application;

impl Application {
    pub fn run() {
        let arg = args::Arg::parse();
        let mut parse = tree_sitter::Parser::new();
        parse.set_language(&arg.language.sitter_language()).expect(
            format!(
                "Failed to load {} tree-sitter language",
                arg.language.to_string().as_str()
            )
            .as_str(),
        );
        println!("processed files = {:?}", files::proceed(&arg));

        let mut query =
            tree_sitter::Query::new(&arg.language.sitter_language(), &arg.language.query())
                .unwrap();
        let files = files::proceed(&arg);
        for file in files {
            println!("path = {file}");
            let res = std::fs::read_to_string(file).unwrap();
            let tree = parse.parse(&res.as_bytes(), None).unwrap();
            let mut query_cursor = tree_sitter::QueryCursor::new();
            let mut query_matches = query_cursor.matches(&query, tree.root_node(), res.as_bytes());
            for query_match in query_matches {
                let mut poses = vec![];
                for query_capture in query_match.captures {
                    let p = Pos::from(*query_capture);
                    poses.push(p.clone());
                    let p1 = files::walk_file(res.as_bytes(), p.clone());
                    println!("p1 = {p1:?}");
                }
                let meta = form_meta(poses);
                println!("meta = {meta:?}");
                println!("-----");
                // println!("{:?}", q.captures);
                // println!("------");

                // for qc in q.captures {}
            }
        }
    }
}

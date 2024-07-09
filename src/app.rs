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
            let mut qc = tree_sitter::QueryCursor::new();
            let mut qc_res = qc.matches(&query, tree.root_node(), res.as_bytes());
            for q in qc_res {
                // Pos::from(q.captures);
                let mut poses = vec![];
                for q1 in q.captures {
                    let p = Pos::from(*q1);
                    poses.push(p);
                    // form_meta()
                    // println!("{:?}", p);
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

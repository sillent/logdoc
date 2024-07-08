use clap::Parser;

use crate::args;
use crate::files;
use crate::queries::*;
pub struct Application;

impl Application {
    pub fn run() {
        let arg = args::Arg::parse();
        let mut parse = tree_sitter::Parser::new();
        parse.set_language(&arg.language.sitter_language()).expect(
            format!(
                "failed to load {} tree-sitter language",
                arg.language.to_string().as_str()
            )
            .as_str(),
        );
        println!(
            "processed files = {:?}",
            files::proceed(&arg.files, &arg.directories, true)
        );
        // unimplemented!("test");
        if let Some(files) = arg.files {
            println!("path = {files:?}");
            let res = std::fs::read_to_string(files.get(0).unwrap()).unwrap();
            let tree = parse.parse(res.as_bytes(), None).unwrap();

            #[allow(unused_mut)]
            let mut query =
                tree_sitter::Query::new(&tree_sitter_go::language(), &QUERY_LOG2).unwrap();
            let mut qc = tree_sitter::QueryCursor::new();
            let mut qc_res = qc.matches(&query, tree.root_node(), res.as_bytes());
            for q in qc_res {
                // println!("q = {:?}", q);
                println!("{:?}", q.captures);
                // q.captures.iter(|x| println!(""))
                println!("------");
            }
        }
    }
}

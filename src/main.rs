mod app;
mod args;
mod files;
use clap::Parser;
use tree_sitter;
use tree_sitter_go;

const QUERY: &'static str = r#"(
    (comment) @comment1
    (comment) @comment2
	(expression_statement
    	(call_expression
        	function:
            	((selector_expression
                  (_)
                  (field_identifier) @module
                )(#match? @module "^(Info|Infof|Print|Printf|Debug|Debugf|Error|Errorf|Trace|Tracef)"))
            (argument_list (interpreted_string_literal) @strliteral
            )
         )
    ) 

)"#;

fn main() {
    let arg = args::Arg::parse();
    println!("arg = {:?}", arg);
    // println!("Hello, world!");
    let mut parse = tree_sitter::Parser::new();
    parse
        .set_language(&tree_sitter_go::language())
        .expect("failed to load GO tree sitter language");
    // let arg = std::env::args().next();
    let mut args = std::env::args();
    args.next().unwrap();
    let file = args.next().unwrap();
    let res = std::fs::read_to_string(file).unwrap();
    let tree = parse.parse(res.as_bytes(), None).unwrap();
    println!("tree = {tree:?}");

    //    let pattern = "(
    //    (comment)* @comment
    // (expression_statement
    //    	(call_expression
    //        	(selector_expression
    //            	(identifier)
    //                (field_identifier)
    //            )
    //         )
    //    ) @expr)";
    //    let pattern2 = "(
    //    (comment)* @comment
    // (expression_statement
    //    	(call_expression
    //        	(selector_expression
    //            	(identifier)
    //                (field_identifier) @field
    //            )
    //            (argument_list (interpreted_string_literal) @strliteral
    //            ) @arglist
    //         )
    //    )
    //    )";
    //     let pattern3 = r#"(
    //     (comment) @comment1
    //     (comment) @comment2
    // 	(expression_statement
    //     	(call_expression
    //         	function:
    //             	((selector_expression
    //                   (_)
    //                   (field_identifier) @module
    //                 )(#match? @module "^(Info|Infof|Print|Printf|Debug|Debugf|Error|Errorf|Trace|Tracef)"))
    //             (argument_list (interpreted_string_literal) @strliteral
    //             )
    //          )
    //     )

    // )"#;
    #[allow(unused_mut)]
    let mut query = tree_sitter::Query::new(&tree_sitter_go::language(), &QUERY).unwrap();
    // println!("{:?}", query.unwrap().capture_names());
    let mut qc = tree_sitter::QueryCursor::new();
    let qc_res = qc.matches(&query, tree.root_node(), res.as_bytes());
    // println!("qc res = {:?}", qc_res.count());
    // qc_res.next()
    for q in qc_res {
        println!("q = {:?}", q);
        // c.node.named_children()
        println!(
            "cap for name = {:?}",
            query.capture_index_for_name("strliteral")
        );
        println!("----");
    }

    // println!("res = {res:?}");
}

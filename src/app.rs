use clap::Parser;

use crate::args;
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

const QUERY2: &'static str = r#"(
    (comment) @comment1 
    (comment)* @comment2
	(expression_statement
    	(call_expression
        	function:
            	((selector_expression
                  (field_identifier) @module
                )(#match? @module "^(Info|Infof|Print|Printf|Debug|Debugf|Error|Errorf|Trace|Tracef)"))
            (argument_list (interpreted_string_literal) @strliteral
            )
         )
    )*
)"#;
const QUERY3: &'static str = r#"(
    ((comment) @desc
    .
    (comment)? @action)
    .
	(expression_statement
    	(call_expression
        	function:
            	((selector_expression
                  (field_identifier) @module
                )(#match? @module "^(Info|Infof|Print|Printf|Debug|Debugf|Error|Errorf|Trace|Tracef)"))
            (argument_list (interpreted_string_literal) @strliteral
            )
         )
    )
)"#;

const QUERY4: &'static str = r#"
(source_file (package_clause (package_identifier) @package))
(
    ((comment) @comment1  
    .
    (comment)? @comment2)
    .
	(expression_statement
    	(call_expression
        	function:
            	((selector_expression
                  (field_identifier) @module
                )(#match? @module "^(Info|Print|Debug|Error|Trace)"))
            (argument_list 
            	[
                	(interpreted_string_literal) @strliteral
                    (identifier) @strvar
                ]
            )
         )
    )
)
"#;

const QUERY5: &'static str = r#"
(source_file (package_clause (package_identifier) @package))
(
    ([((comment) @comment1  
    .
    (comment) @comment2) @two_comments
    (
     (comment) @comment1
    ) @one_comment
    ])
	(expression_statement
    	(call_expression
        	function:
            	((selector_expression
                  (field_identifier) @module
                ) . (#match? @module "^(Info|Info|Print|Print|Debug|Debug|Error|Error|Trace|Trace)"))
            (argument_list 
            	[
                	(interpreted_string_literal) @strliteral
                    (identifier) @strvar
                ]
            )
         )
    )
)
"#;

pub struct Application;

impl Application {
    pub fn run() {
        let arg = args::Arg::parse();
        let mut parse = tree_sitter::Parser::new();
        parse
            .set_language(&tree_sitter_go::language())
            .expect("failed to load GO tree sitter language");
        if let Some(file) = arg.file {
            println!("path = {file:?}");
            let res = std::fs::read_to_string(file).unwrap();
            let tree = parse.parse(res.as_bytes(), None).unwrap();

            #[allow(unused_mut)]
            let mut query = tree_sitter::Query::new(&tree_sitter_go::language(), &QUERY4).unwrap();
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

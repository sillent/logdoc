pub const QUERY: &'static str = r#"(
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

pub const QUERY2: &'static str = r#"(
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
pub const QUERY3: &'static str = r#"(
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

pub const QUERY4: &'static str = r#"
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

pub const QUERY5: &'static str = r#"
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

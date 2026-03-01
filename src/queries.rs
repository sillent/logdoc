pub const QUERY_GOLANG: &str = r#"
(
  [
    (
      (comment)+ @comment
      .
      (expression_statement
        (call_expression
          (selector_expression
            (field_identifier) @level
            (#match? @level
              "^([Ii][Nn][Ff][Oo]*|[Dd][Ee][Bb][Uu][Gg]*|[Ww][Aa][Rr][Nn]*|[Ff][Aa][Tt][Aa][Ll]*|[Ee][Rr][Rr][Oo][Rr]*|[Tt][Rr][Aa][Cc][Ee]*)$")
          )
          (argument_list
            (
              [
                (interpreted_string_literal)
                (raw_string_literal)
              ]+
            ) @content
          )
        )
      )
    )
    (
      (expression_statement
        (call_expression
          (selector_expression
            (field_identifier) @level
            (#match? @level
              "^([Ii][Nn][Ff][Oo]*|[Dd][Ee][Bb][Uu][Gg]*|[Ww][Aa][Rr][Nn]*|[Ff][Aa][Tt][Aa][Ll]*|[Ee][Rr][Rr][Oo][Rr]*|[Tt][Rr][Aa][Cc][Ee]*)$")
          )
          (argument_list
            (
              [
                (interpreted_string_literal)
                (raw_string_literal)
              ]+
            ) @content
          )
        )
      )
    )
  ]
)

"#;
pub const QUERY_RUST: &str = r#"
(
  [
    (
      [
        (line_comment)
        (block_comment)
      ]+ @comment
      .
      (expression_statement
        (macro_invocation
          [
            (identifier) @level
            (scoped_identifier
              name: (identifier) @level)
          ]
          (#match? @level
            "^([Ii][Nn][Ff][Oo]*|[Dd][Ee][Bb][Uu][Gg]*|[Ww][Aa][Rr][Nn]*|[Ff][Aa][Tt][Aa][Ll]*|[Ee][Rr][Rr][Oo][Rr]*|[Tt][Rr][Aa][Cc][Ee]*)$")
          (token_tree
            (string_literal) @content)
        )
      )
    )
    (
      (expression_statement
        (macro_invocation
          [
            (identifier) @level
            (scoped_identifier
              name: (identifier) @level)
          ]
          (#match? @level
            "^([Ii][Nn][Ff][Oo]*|[Dd][Ee][Bb][Uu][Gg]*|[Ww][Aa][Rr][Nn]*|[Ff][Aa][Tt][Aa][Ll]*|[Ee][Rr][Rr][Oo][Rr]*|[Tt][Rr][Aa][Cc][Ee]*)$")
          (token_tree
            (string_literal) @content)
        )
      )
    )
  ]
)
"#;

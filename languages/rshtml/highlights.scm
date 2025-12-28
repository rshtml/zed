(start_symbol) @keyword
(hash_symbol) @punctuation.special

(open_paren) @punctuation.bracket
(close_paren) @punctuation.bracket
(open_brace) @punctuation.bracket
(close_brace) @punctuation.bracket

(fat_arrow) @operator
(semicolon) @punctuation.delimiter
(equals) @punctuation.delimiter

(string_line) @string

(comment_block) @comment
(open_comment) @operator
(close_comment) @operator

(continue_) @keyword
(break_) @keyword

(raw_) @keyword
(child_content_) @keyword

(as_) @keyword
(as_clause
  alias: (component_tag_identifier) @type)
(
  (start_symbol) @keyword
  .
  (use_directive (use_) @keyword)
)

(number) @number
(bool) @boolean

(tag_open) @punctuation.bracket
(tag_close) @punctuation.bracket
(tag_end_open) @punctuation.bracket
(tag_self_close) @punctuation.bracket

(component_tag
  name: (component_tag_identifier) @tag)

(component_tag
  name_close: (component_tag_identifier) @tag)

(component_tag_parameter
  name: (rust_identifier) @attribute)

(
  (start_symbol) @function
  .
  (rust_expr_simple)
)

(
  (start_symbol) @function
  .
  (rust_expr_paren)
)

(
  (start_symbol) @keyword
  .
  (rust_block)
)

(
  (start_symbol) @keyword
  .
  (if_stmt)
)
(
  (start_symbol) @keyword
  .
  (for_stmt)
)
(
  (start_symbol) @keyword
  .
  (while_stmt)
)
(
  (start_symbol) @keyword
  .
  (match_stmt)
)

(param_name) @variable

;this is for now extra
(else_clause
  head: (rust_text) @keyword)

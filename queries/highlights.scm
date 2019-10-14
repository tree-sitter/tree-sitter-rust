; Identifier conventions

; Assume all-caps names are constants
((identifier) @constant
 (match? @constant "^[A-Z][A-Z\\d_]+$'"))

; Assume that uppercase names in paths are types
((scoped_identifier
  path: (identifier) @type)
 (match? @type "^[A-Z]"))
((scoped_identifier
  path: (scoped_identifier
    name: (identifier) @type))
 (match? @type "^[A-Z]"))

; Assume other uppercase names are enum constructors
((identifier) @constructor
 (match? @constructor "^[A-Z]"))

; Function calls

(call_expression
  function: (identifier) @function)
(call_expression
  function: (field_expression
    field: (field_identifier) @function.method))
(call_expression
  function: (scoped_identifier
    "::"
    name: (identifier) @function))

(generic_function
  function: (identifier) @function)
(generic_function
  function: (scoped_identifier
    name: (identifier) @function))
(generic_function
  function: (field_expression
    field: (field_identifier) @function.method))

(macro_invocation
  macro: (identifier) @function.macro
  "!" @function.macro)

; Function definitions

(function_item (identifier) @function)
(function_signature_item (identifier) @function)

; Other identifiers

(type_identifier) @type
(primitive_type) @type.builtin
(field_identifier) @property

(line_comment) @comment
(block_comment) @comment

"(" @punctuation.bracket
")" @punctuation.bracket
"[" @punctuation.bracket
"]" @punctuation.bracket

(type_arguments
  "<" @punctuation.bracket
  ">" @punctuation.bracket)
(type_parameters
  "<" @punctuation.bracket
  ">" @punctuation.bracket)

"::" @punctuation.delimiter
"." @punctuation.delimiter
";" @punctuation.delimiter

(parameter (identifier) @variable.parameter)

(lifetime (identifier) @label)

; macro_definition > identifier,
; generic_function > identifier,
; generic_function > field_expression > field_identifier,
; generic_function > scoped_identifier > identifier,
; function_signature_item > identifier {
;   highlight: 'function';
; }
;

; use_wildcard > identifier:nth-child(0),
; use_wildcard > scoped_identifier > identifier:nth-child(2),
; scoped_type_identifier > identifier:nth-child(0),
; scoped_type_identifier > scoped_identifier:nth-child(0) > identifier,
; scoped_identifier > identifier:nth-child(0),
; scoped_identifier > scoped_identifier:nth-child(0) > identifier,
; use_declaration > identifier,
; use_declaration > scoped_identifier > identifier,
; use_list > identifier,
; use_list > scoped_identifier > identifier,
; meta_item > identifier {
;   &[text="^[A-Z]"] {
;     highlight: 'type';
;   }
; }
;
; lifetime > identifier {
;   highlight: 'label';
; }
;

"break" @keyword
"const" @keyword
"continue" @keyword
"default" @keyword
"dyn" @keyword
"else" @keyword
"enum" @keyword
"extern" @keyword
"fn" @keyword
"for" @keyword
"if" @keyword
"impl" @keyword
"in" @keyword
"let" @keyword
"let" @keyword
"loop" @keyword
"macro_rules!" @keyword
"match" @keyword
"mod" @keyword
"move" @keyword
"pub" @keyword
"ref" @keyword
"return" @keyword
"static" @keyword
"struct" @keyword
"trait" @keyword
"type" @keyword
"union" @keyword
"unsafe" @keyword
"use" @keyword
"where" @keyword
"while" @keyword
(mutable_specifier) @keyword
(use_list (self) @keyword)
(scoped_use_list (self) @keyword)
(scoped_identifier (self) @keyword)
(super) @keyword

(self) @variable.builtin

(char_literal) @string
(string_literal) @string
(raw_string_literal) @string

(boolean_literal) @constant.builtin
(integer_literal) @constant.builtin
(float_literal) @constant.builtin

(escape_sequence) @escape

(attribute_item) @attribute
(inner_attribute_item) @attribute

"as" @operator
"*" @operator
"&" @operator
"'" @operator

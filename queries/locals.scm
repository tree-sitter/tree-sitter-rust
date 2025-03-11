; Scopes

(block) @local.scope
(function_item) @local.scope
(closure_expression) @local.scope

; Definitions

(parameters
  (identifier) @local.definition)

(let_declaration
  name: (identifier) @local.definition)

(let_declaration
  name: (identifier) @local.definition)

; References

(identifier) @local.reference

# OpenAPI Gen (仮)

ver.2024.02.15

## 基本的な構文

```text
file-root:
  (import-statement)* (object-definition)*

object-definition:
  type-name object-name (with-attributes)? definition

shorthand-object-definition:
  type-name (with-attributes)? definition

type-name:
  "schema" | "enum" | "tag" | "response" | "path" | "requestBody"

with-attributes:
  "with" attributes

attributes:
  attribute ("," attribute)* (",")?
  
attribute:
  attribute-name "=" value

definition:
  type-specific-definition

import-statement:
  "import" relative-path ";"

value:
  literals | shorthand-object-definition

literals:
  string-literal | integer-literal | floating-number-literal | bool-literal | list-literal

string-literal:
  simple-string-literal | raw-string-literal

simple-string-literal:
  '"' (char)* '"'

raw-string-literal:
  'r#"' (char | "\n" | "\"")* '"#'

integer-literal:
  decimal-integer-literal | octal-integer-literal | hexadecimal-integer-literal | binary-integer-literal

decimal-integer-literal:
  ([0-9])+

octal-integer-literal:
  "0o" ([0-7])+

hexadecimal-integer-literal:
  "0x" ([a-fA-F0-9])+

binary-integer-literal:
  "0b" ([0-1])+

floating-number-literal:
  "." ([0-9])+ | ([0-9])+ "." ([0-9])*

bool-literal:
  "true" | "false"

list-literal:
  "[" (list-values)? "]"

list-values:
  value ("," value)* (",")?
```

## `schema`の構文

```text
schema-definition:
  brace-schema-definition | assignment-schema-definition

brace-schema-definition:
  "{" brace-schema-content "}"

assignment-schema-definition:
  "=" data-type ";"

brace-schema-content:
  data-content ("," data-content)* (",")?

data-content:
  object-name ":" ("required" | "optional")? data-type (with-attributes)?

data-type:
  "String" | "Object" | "Bool"
  | "Int32" | "Int64"
  | "Float"
  | "Number" | "Int"
  | "DateTime" | "Date" | "Time" | "Duration"
  | "Email" | "Uuid" | "Uri"
  | schema-name | list-type | shorthand-scheme-definition

list-type:
  "List" "<" data-type ">"
```

## `enum`の構文

```text
enum-definition:
  "{" enum-contents "}"
  
enum-contents:
  enum-content ("," enum-content)* (",")?

enum-content:
  identifier
```

## `tag`の構文

```text
tag-definition:
  ";"
```

## `response`の構文

```text
response-definition:
  "{" response-contents "}"

response-contents:
  (response-content)*

response-content:
  headers-assignment | content-assignment

headers-assignment:
  "headers" "=" headers-definition ";"

content-assignment:
  "content" "=" content-definition ";"

headers-definition:
  "headers" "{" headers-content "}"

headers-content:
  string-literal ":" TODO

content-definition:
  schema-name | shorthand-schema-definition
```

## `requestBody`の構文

```text
request-body-definition:
  "{" request-body-contents "}"

request-body-contents:
  (request-body-content)*

request-body-content:
  content-assignment
```

## `path`の構文

```text
path-definition:
  "{" path-content "}"

path-content:
  (method-operation | parameter-spec)

parameter-spec:
  (path-parameter-spec | query-parameter-spec | cookie-parameter-spec | header-parameter-spec)

path-parameter-spec:
  "pathParameters" "=" parameters-array

query-parameter-spec:
  "queries" "=" parameters-array

cookie-parameter-spec:
  "cookies" "=" parameters-array

header-parameter-spec:
  "headers" "=" parameters-array

parameters-array:
  "[" parameter-contents "]"

parameter-contents:
  parameter-content ("," parameter-content)* (",")?

parameter-content:
  "parameter" "{" (parameter-name | parameter-content | parameter-required)* "}"

parameter-name:
  "name" "=" string-literal

parameter-content:
  "content" "=" content-definition

parameter-required:
  "required" "=" bool-literal

method-operation:
  method operation-name "(" (arguments)? ")" "{" path-method-content "}"

method:
  "get" | "post" | "put" | "delete" | "options" | "head" | "trace"

arguments:
  argument ("," argument)* (",")?

argument:
  (parameter-spec | body-spec)

body-spec:
  "body" "=" content-definition

path-method-content:
  (return-statement)*

return-statement:
  "return" (http-status-code | "default") response-type ";"

response-type:
  response-name | shorthand-response-definition

http-status-code:
  R"\d{3}"
```

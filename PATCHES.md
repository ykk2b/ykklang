# Patches (beta, alpha)

### v0.0.1-alpha.13

- fix value printing
- remove docs

### v0.0.1-beta.2

- fixed maps
- added optimizer (removing unused functions)
- updated frontend tests
- added `--no-optimize` option

### v0.0.1-beta.1

- finish and optimize tokenizer tests
- fix this loop never actually loops [#267](https://github.com/ykk2b/ykklang/security/code-scanning/267)
- fix parameter is only used in recursion [#270](https://github.com/ykk2b/ykklang/security/code-scanning/270)
- fix using clone on type Option<&ValueType> which implements the Copy trait [#268,#269](https://github.com/ykk2b/ykklang/security/code-scanning/269)
- fix this boolean expression can be simplified [#266](https://github.com/ykk2b/ykklang/security/code-scanning/270)
- fix implementation of inherent method to_string(&self) -> String for type api::types::ValueType [#265](https://github.com/ykk2b/ykklang/security/code-scanning/265)
- fix writing &Vec instead of &[_] involves a new object where a slice will do [#256](https://github.com/ykk2b/ykklang/security/code-scanning/256)

### v0.0.1-alpha.12

- add tokenizer tests (in progress)
- fixed functions
- removed variables and anon functions

### v0.0.1-alpha.11

- added `print!` builting function
- minor fixes

### v0.0.1-alpha.10

- finished expression interpreter
- fixed writing &Vec instead of &[_] involves a new object where a slice will do [(#260)](https://github.com/ykk2b/ykklang/security/code-scanning/260)
- fixed using clone on type usize which implements the Copy trait [(#258)](https://github.com/ykk2b/ykklang/security/code-scanning/258)
- fixed minor bugs

### v0.0.1-alpha.9

- minor fixes
- progressed expression backened (in progress)
- fixed variable does not need to be mutable [(#4)](https://github.com/ykk2b/ykklang/security/code-scanning/4)
- fixed using clone on type bool which implements the Copy trait [(#252)](https://github.com/ykk2b/ykklang/security/code-scanning/252)
- fixed length comparison to zero [(#250,#251)](https://github.com/ykk2b/ykklang/security/code-scanning/251)
- fixed casting float literal to f32 is unnecessary [(#249)](https://github.com/ykk2b/ykklang/security/code-scanning/249)

### v0.0.1-alpha.8

- switched to mdbooks for documentation
- removed `UnaryRight`
- progressed expressions backend (in progress)
- finished statements interpreter
- assembled backend
- updated parser

### v0.0.1-alpha.7

- fixed name CLI contains a capitalized acronym [(#160)](https://github.com/ykk2b/ykklang/security/code-scanning/160)
- fixed use of char::is_digit with literal radix of 10 [(#155,#157,#158,#159)](https://github.com/ykk2b/ykklang/security/code-scanning/159)
- fixed used expect() on Ok value [(#151,#152,#153,#156)](https://github.com/ykk2b/ykklang/security/code-scanning/156)
- fixed this if-then-else expression returns a bool literal [(#154)](https://github.com/ykk2b/ykklang/security/code-scanning/154)
- fixed this expression creates a reference which is immediately dereferenced by the compiler ([#149,#150])(https://github.com/ykk2b/ykklang/security/code-scanning/150)
- fixed redundant pattern matching, consider using is_some() [(#148)](https://github.com/ykk2b/ykklang/security/code-scanning/148)
- fixed all variants have the same postfix: ValueType [(#137-#146)](https://github.com/ykk2b/ykklang/security/code-scanning/146)
- fixed very complex type used. Consider factoring parts into type definitions [#136](https://github.com/ykk2b/ykklang/security/code-scanning/136)
- fixed Vec<T> is already on the heap, the boxing is unnecessary [#123,#132-#135](https://github.com/ykk2b/ykklang/security/code-scanning/135)
- fixed all variants have the same postfix: Statement [#124-#131](https://github.com/ykk2b/ykklang/security/code-scanning/131)
- fixed all variants have the same postfix: Expression [#113-#122](https://github.com/ykk2b/ykklang/security/code-scanning/122)

### v0.0.1-alpha.6

- updated `return` parser:
  - now it must return an expression
- added modules environment backend (in progress)
- added resolver backend (in progress)
- added expressions backend (in progress)

### v0.0.1-alpha.5

- added `public` for `variable` and `function`
- updated `and` and `or` operators

### v0.0.1-alpha.4

- updated `variable` and `function` parsing logic
- updated and fixed `anon` function parsing logic
- added short functions

### v0.0.1-alpha.3

- fixed type parsing
- fixed multi-character token tokenization problem
- updated error messages

### v0.0.1-alpha.2

- added anonymous functions (frontend).
- updated `consume` function
- updated `run` command
- removed `array` left overs.
- fixed `map` parser.

### v0.0.1-alpha.1

- removed `while`, `break`, `assignment` and `array` from frontend.
- renamed `mod` to `module`.
- added new `->`(Arrow) token.
- updated `function` parsing logic.
- replaced `array` with `map`.

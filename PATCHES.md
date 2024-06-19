# Patches (alpha)

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

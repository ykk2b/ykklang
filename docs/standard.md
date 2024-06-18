# Language Standard

## placeholders

- `T`,`E`,`N`: type placehoders
- `NAME`: name placeholder
- `EXPRESSION`: expression placeholder
- `PARAM`: function parameter placeholder
- `ARG`: function argument placeholder
- `BLOCK`: block statement placeholder
- `STATEMENT`: statement placeholder
- `PATH`: path to the file placeholder

## variables

schema:

```
T NAME = EXPRESSION;
```

examples:

```ts
number x = 5;
string name = "john";
boolean isDir = false;
map user = [
    "name": "john",
    "age": 25
];
null x = null;
```

features:

- immutable
- shadowing isn't allowed
- unused and unnecessary variables will be destroyed
- type `void` isn't allowed

## functions

schema:

```
T NAME(E PARAM1, N PARAM2) {BLOCK | -> EXPRESSION}
T NAME(E PARAM1, ...N PARAM2) BLOCK
T NAME = $(E PARAM1, N PARAM2) {BLOCK | EXPRESSION}

NAME(ARG1, ARG2);
```

examples:

```ts
number add(number a, number b){
    return a + b;
}
add(2, 4);
```

```rs
number add(number a, number b) -> a + b;
add(2, 4);
```

```rs
number add = (number a, number b) -> a + b;
add(2, 4)
```

features:

- arity is limited to 32
- functions can't recurse more then 64 times
- functions with same name aren't allowed
- unused functions will be destroyed
- unused parameters will be destoryed

## if

schema:

```
if EXPRESSION BLOCK
if EXPRESSION BLOCK else BLOCK
if EXPRESSION BLOCK else if BLOCK else BLOCK
if EXPRESSION -> EXPRESSION;
if EXPRESSION -> EXPRESSION -> EXPRESSION;
```

example:

```rs
number x = 5;

if x < 10 {
    print(x);
} else if x > 10 {
    print(x - 10);
} else {
    print(10)
}
```

features:

- variables/functions used in if statement expressions willn't count as used variable/function

## public

schema:

```
public STATEMENT
```

example:

```rs
public number pi = 3.1415;
public number square(number a) -> a * a;
public as PI number pi = 3.1415;
```

## modules

schema:

```
module NAME from PATH;
module NAME1, NAME2 from PATH;
```

examples:

```ts
module pi from "./math.ykk";
module pi, square from "./math.ykk";
module pi as PI from "./math.ykk";
```

## operations

### expressions

- `+`
  - EXPRESSION + EXPRESSION
  - 2 + 4
  - 6
- `-`
  - EXPRESSION - EXPRESSION
  - 4 - 2
  - 2
- `*`
  - EXPRESSION \* EXPRESSION
  - 2 \* 2
  - 4
- `/`
  - EXPRESSION / EXPRESSION
  - 4 / 2
  - 2
- `%`
  - EXPRESSION % EXPRESSION
  - 5 % 2
  - 1
- `!`
  - ! EXPRESSION
  - !true
  - false
- `>`
  - EXPRESSION > EXPRESSION
  - 5 > 2
  - true
- `>=`
  - EXPRESSION >= EXPRESSION
  - 5 >= 2
  - true
- `<`
  - EXPRESSION < EXPRESSION
  - 5 < 2
  - false
- `<=`
  - EXPRESSION <= EXPRESSION
  - 5 <= 2
  - false
- `==`
- EXPRESSION == EXPRESSION
  - 5 == 2
  - false
- `!=`
- EXPRESSION == EXPRESSION
  - 5 !== 2
  - true
- `||`
  - EXPRESSION || EXPRESSION
  - false || true
  - true
- `&&`
  - EXPRESSION && EXPRESSION
  - true && false
  - false

### type expressions

- `|`
  - T | T
  - string | mytype
- `&`
   - T & T
  - string & mytype

# YKKLang

A minimalistic and purely functional programming language written in Rust.

## Installation

Download the latest stable (or rc) version from [releases](https://github.com/ykk2b/ykklang/releases), and if you are using Linux run:

```sh
sudo mv path/to/ykklang usr/bin
ykklang --version
```

And if you are using Windows or MacOS, switch to Linux!

## Hello World!

To write simple "Hello, world!" program, create `whatever.ykk` file and enter:

```
print!("Hello, world!");
```

and then run:

```sh
ykklang --run whatever.ykk
```

you should see the output:

```
"Hello, world!"
```

## Paradigms

[High-level](https://en.wikipedia.org/wiki/High-level_programming_language), [declarative](https://en.wikipedia.org/wiki/Declarative_programming) and [purely functional](https://en.wikipedia.org/wiki/Purely_functional_programming).

## Data Types

|  name   | description                   | example                     |
| :-----: | :---------------------------- | :-------------------------- |
| string  | just a string                 | "hi"                        |
| number  | 32 bit floating point number  | 23                          |
| boolean | either true or false          | true                        |
|  true   | just true                     | true                        |
|  false  | just false                    | false                       |
|   map   | collection of key-value pairs | ["name": "john", "age": 25] |
|  null   | no value type                 | null                        |
|  void   | function type with no returns | void                        |

## Functions

```rs
void printhi(){
    print!("hi");
}

number return2plus2(){
    return 2 + 2;
}

number samebutshorter() -> 2 + 2;

number add(number a, number b) -> a + b;

void loop10times(number i){
    print!(i);
    if i < 10 {
        loop10times(i + 1);
    }
}
loop10times(0);

void hof(number a, number b) -> a + b;
void square(number a) -> a * a;
hof(square(2), square(5));
```

## If statement

```rs
if true {
    print!(true);
}

if false {
    print!("this wont print");
} else {
    print!(true);
}

if false {
    print!("this wont print");
} else if true {
    print!(true);
}
```

## Comments

todo.

## Modules

todo.

## Type System

todo.

## API

todo.

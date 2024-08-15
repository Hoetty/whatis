# WhatIs

Quickly calculate small expressions or open a REPL to calculate multiple expressions

```sh
> whatis 1 + 2
3
> whatis
2 + 2
4
3 + 4
7
> whatis "sin(pi) * 2" # Expressions with functions and * and ** need to be enclosed in quotation marks due to how the shell works
```

Supports a number of operators
```rust
+ - * / % **
```
As well as functions
```rust
sin(3)
atan2(3,4)
```
And constants
```rust
ln(e)
```

These are all functions and constants currently availabe:
```
e
pi
tau

sin()
cos()
tan()
asin()
acos()
atan()
sinh()
cosh()
tanh()
asinh()
acosh()
atanh()

abs()
ceil()
floor()

sqrt()
ln()
lg()

log(,)
atan2(,)
root(,)
pow(,)
```

This small Calculator is written in Rust and consists of:
- A lexer
- A recursive descent parser
- A recurisive ast walking evaluator

In the future a bytecode vm might be implemented to increase performance
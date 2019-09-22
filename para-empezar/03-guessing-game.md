# Adivinando

```rust
use std::io
```

Brings input/output library to the scope.

> ## By the default Rust only includes a few types into the scope of every program in the `prelude`.
>
> `let` to declare a variable

```rust
let var = value;
```

Variables are immutable by default. To make a variable mutable use `mut`

```rust
let mut variable = mutableValue
```

```rust
String::new() // Creates a new empty string
```

String string type from std library. Growable, UTF-8 encode bit of text

`::new` Indicates that new is an `associated function` of the String type, rather than on an instance. "static method"

```rust
function(& variable)
function(&mut variable)
```

& Indicates we are passing a reference instead a variable. references allow multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. As variables, references are immutable by default. `&mut` to change that.

`enums` is a type that can have a fixed set of values.

`readline()` returns a `Result` type. `Result` types are enums. It's variants are `Ok` and `Err`.

An instance of the `io::Result` type has method `expect`

```rust
if (io::Result === Err ) {
  program crashes, and displays message from `expect`
} else () {
  // io::Result === Ok
  `expect` takes the value that Ok is holding and returns it for you to use.
}
```

```rust
let panda = 🐼;
let pig = 🐷;
printl!("Panda {}, Pig {}", panda, pig)
```

To print variables we use `{}` to indicate the position where the value will be inserted.
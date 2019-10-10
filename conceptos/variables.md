# Variables

Usamos `let` para declarar una variable, y esta es inmutable por defecto. 
```rust
let x = 5
```

Si tratamos de cambiar el valor de una variable el compilador capturará el error.

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

```bash
error[E0384]: cannot assign twice to immutable variable `x`
 --> src/main.rs:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: make this binding mutable: `mut x`
3 |     println!("The value of x is: {}", x);
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable
```

Si necesitamos cambiar el valor de una variable debemos declararla como mutable

```rust
let mut x = 5
```

## Constantes
Si queremos declarar un valor constante, que es _siempre_ inmutable lo hacemos con `const`. Por convención, las constantes son nombradas en mayúsculas.

```rust
 // Podemos utilizar _ para leer números más fácilmente.
const VELOCIDAD_MAX :u32 = 800_000; 
```

## Sombreado (Shadowing)
Podemos declrar una variable con el mismo nombre de una variable existente. La nueva variable le _hace sombra_ a la anterior. Por ejemplo:

```rust
fn main() {
  let x = 2;
  let x = x + 2;
  let x = x * 2;

  println!("x es {}", x); // x es 8
}
```

Es diferente a declarar una variable como `mut`:
- necesitamos let para declarar la nueva variable siempre.
- podemos hacer transformaciones pero la variable final es inmutable.
- como estamos declarando una variable nueva, podemos cambiar el tipo pero usar el mismo nombre. Si tratamos de hacer esto con una variable `mut` tendremos un error de compilación.

```rust 
// válido 
let numero = "1234567890";
let numero = numero.len();

//error
let mut numero = "1234567890";
numero = numero.len();
```


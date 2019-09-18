# Primero

Un archivo en Rust tiene la extensión `rs`.
```rust
miArchivo.rs
```

Rust es un lenguaje compilado y su compilador es `rustc`. 

Rust usa *snake_case* para nombrar funciones y variables. Todas las letras en minúsculas y guíon bajo para reemplazar espacios. 


Para declarar una función usamos `fn`. Y en Rust la función `main` es el punto de entrada de la mayoría de programas. Además de funciones Rust cuenta con `macros`, estos son llamados `macro!()`. 

```rust
fn main () {
  hola()

}
fn hola () {
    println!("hola 👋")
}
```
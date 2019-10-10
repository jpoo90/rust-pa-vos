# Funciones


_Siempre_ tenemos que declarar el tipo de cada parámetro cuando definimos una función.
```rust
fn la_funcion(x: u32, y: u32) {
  println("x es {}; y es {}", x, y)
}
```

En una función, podemos tener expresiones (expressions) y declaraciones (statements). 
Una _declaración_ es una instrucción que completa una acción pero no retorna ningún valor.
Una _expresión_ es evaluada y retorna un valor.

```rust

fn main() {
  let x = 5; // es una declaración. 

  // El bloque {} es una expresión que retorna un valor
  // y lo asigna a y.
  let y = {
    let z = 2;
    // Las expresiones no llevan ; 
    // si añadimos ; se convierte en una declaración 
    // y el valor no es retornado
    z + 2 
  }
}
```

Como en otros lenguajes las funciones pueden retornar valores.
Los valores retornados no necesitan ser nombrados pero si necesitamos declarar su tipo.
Normalmente el valor que se retorna es la última línea de un bloque. Si es necesario podemos usar `return` para retornar antes. 

```rust
// El tipo del valor retornado lo indicamos después del símbolo -> 
fn uno_mas(x: i32) -> i32 {
  x + 1
}
```

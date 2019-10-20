# Control de flujo

## if 
A diferencia de JS la condición en el condicional siempre debe ser un booleano.
Como en otros lenguajes podemos usar `else` y `else if`. 

```rust 
fn  main() {
  let x = 6 

  if number % 4 == 0 {
    println!("divisible por 4")
  } else if number % 3 == 0 {
    println!("divisible por 3")
  } else {
    println!("no se puede dividir por 3 o 4 ")
  }

  // Esto no funciona!! Necesitamos un booleano
  if x {
    println!("nunca pasa")
  }
}
```

En Rust `if` es una expresión. Por esto, podemos usar un `if` en el lado derecho de la declaracíon de una variable. 

```rust
fn main() {
  let flag = true;

  let letra = if flag {
    'A'
  } else {
    'B'
  }
  println!("La letra es {}", letra) // La letra es A

  // El tipo que retornamos debe ser el mismo en todos los casos.
  // Si los tipos no son iguales el compilador detecta un error.
  let problema = if flag {
    2
  } else {
    "dos"
  }
}
```
## loop
Este ciclo ejecuta un bloque de código por siempre ó hasta que lo detengamos explícitamente. Podemos usar `loop` para reintentar una operación que sabemos puede fallar, por ejemplo: verificar si un hilo ha completado una tarea.

```rust
fn main() {
  let mut cont = 0;
  let resultado = loop {
    cont += 1;

    if cont == 10 {
      // break interrumpe el ciclo.
      // Si necesitamos un valor lo podemos retornar.
      break cont;
    }
  } 

  println!("{}", resultado); // 10
}
```

## while
Nos permite evaluar una condición dentro del ciclo.
Siempre que la condición sea verdadera el ciclo será ejecutado, de lo contrario el programa llama `break` para salir del ciclo.

```rust 
fn main () {
  let mut contador = 3;
  
  while contador != 0 {
    println!("{}!", contador);
    contador = contador - 1; 
  }
  println!("🚀");
}
```

## for

Este el formato más común en Rust para crear ciclos. 
Nos brinda más seguridad y concisión al escribir nuestro código. 

Por ejemplo, podemos obtener el mismo resultado del ejemplo usando `while` de una manera más simple:

```rust 
fn main () {
  // (1..4) Range. HAce parte de la librería estándar.
  // rev() reordena los elementos en el rango. 
  for contador in (1..4).rev() {
    println!("{}!", contador);
  }
  
  println!("🚀");
}
```

Si queremos iterar sobre un arreglo: 

```rust
fn main() {
  let datos = [90, 07, 10];

  for d in datos.iter() {
    println!("{}!", d);
  }
}
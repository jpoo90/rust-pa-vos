Como Rust es un lenguaje tipado estaticamente, esto significa que debemos saber el tipo de todas las variables para compilar. 

El compilador puede inferir el tipo que queremos usar en una variable con base en su valor ó como es usada. 

En casos donde múltiples tipos son posibles tenemos que declarar el tipo que queremos.

```rust
// parse puede extraer el número en diferentes tipos.
let años : u32 = "29".parse().expect("no es un número");
```


## Tipos Escalares 

Representan un solo valor.

### Enteros
Números sin parte fraccionaria. 
Pueden ser signados `i` o no `u`. Cuando el número es signado significa que este puede ser positivo ó negativo. 
```rust 
// `i` desde -(2^n-1) hasta 2^n-1 -1
//i8 puede representar entre -128 y 127
let signado: i8 = -4;

// `u` desde 0 hasta 2^n-1
//u8 puede representar entre 0 y 255
let sin_signo: u8 = 4;
```

|Espacio | Signado | Sin signo |
|-------|--------|----------|
|8-bit | i8 | u8|
|16-bit | i16 | u16|
|32-bit | i32 | u32|
|64-bit | i64 | u64|
|128-bit | i128 | u128|
|arch | isize | usize|

> Por defecto Rust utiliza `i32`

### Punto flotante
Para representar números flotantes Rust tiene dos tipos `f32` y `f64`. El primero es de precision simple y el segundo de precision doble; de acuerdo con el estándar IEEE-754.
```rust
fn main() {
  let y = 2.0; //f64

  let z : f32 = 1.0; 
}
```
> Por defecto Rust utiliza `f64`

### Booleanos
Tiene dos valores posibles verdadero ó falso (`true` ó `false`) y ocupan un byte.

```rust
fn main() {
  let t = true

  let f : bool = false; 
}
```

### Cáracteres

Representados con comillas simples y utiliza 4 bytes en memoria. Esto nos permirte almacenar valores de unicode escalares (Unicode Scalar Values)

```rust
fn main() {
  let emoji = '👌';
  let letra = 'a';
}
```

## Tipos Compuestos
Permiten agrupar múltiples valores en un solo tipo.

### Tuplas (Tuple)
Agrupa múltiples valores con múltiples tipos. Tienen una longitud definida y no puede cambiar después de que es declarada.

```rust
fn main() {
  // Los tipos en este caso son opcionales
  let la_tupla: (i32, bool, u8) = (300, true, 1);

  // Podemos usar destructuring para acceder a los valores. Para valores que no necesitamos usamos _
  let (x, _, _) = la_tupla;

  println!("x es: {}", x); // x es 300

  //También podemos acceder elementos por su indice.
  let valido = la_tupla.1;
  let uno = la_tupla.2;
  println!("x es: {}, valido {}", uno, valido);
}

```

### Arreglos (Arrays)

Todos los elementos en un arreglo en Rust deben tener el mismo tipo. Como las tuplas, la longitud del arreglo es fija.

```rust
fn main() {
  let granja: [char; 3] = ['🐤', '🐷', '🐮'];

  // Para acceder a un elemento en el arreglo usamos indices. 
  let vaca = granja[2];
}
```
# Posesión (Ownership)

Es una característica que separa a Rust de otros lenguajes. Esta le permite a Rust garantizar seguridad en memoria sin necesidad de un recolector de basura (garbage collector)

Todos los lenguajes de programación deben manejar la memoria del computador mientras corren un programa. Hay diferentes estrategias para lograr esto: 

1. Recolector de basura: constantemente busca espacios en memoria que no están siendo usados. ej: __Javascript__
1. Asignar/liberar memoria: El desarrollador debe especificar cuando asignar o liberar memoria. ej: __C__
1. **Sistema de posesión**: La memoria es manejada por el sistema de posesión usando una serie de reglas que son validadas por el compilador. ej: __Rust__

## Stack and Heap

Ambos son partes en memoria que están disponibles en tiempo de ejecución para almacenar datos, pero están estructurados de manera diferente. 

### Stack 
- Almacena datos en el orden que los recibe, pero los remueve en el orden opuesto. Último en entrar primero en salir. 

- Todos los datos en esta estructura deben tener un tamaño fijo y lo debemos saber en tiempo de compilación. 

- Cuando llamamos una función los valores que esta usa son agregados (pushed) en el stack y cuando la función termina son removidos (popped off)

### Heap 
- Almacena datos de los cuales no sabemos su tamaño en tiempo de compilación. 

- Para almacenar datos en esta estructura, pedimos cierto espacio en memoria. El sistema operativo encuentra un punto que sea lo suficientemente grande y retorna un puntero a la dirección en memoria. 

- Acceder datos es más lento porque debemos encontrar los datos usando el puntero. 

> El sistema de posesión de Rust nos permite manejar eficientemente los datos almacenados en el heap.


## Reglas 
- Cada valor en Rust tiene una variable llamada _dueño_ (owner).
- Solo puede existir un dueño a la vez.
- Cuando el dueño sale del scope el valor es descartado.

## Contexto de una variable (Variable scope)
El contexto (scope) de una variable es el espacio de código en el que dicha variable es válida.

```rust
fn main() { // la_variable no es válida aquí. Aún no ha sido declarada.
  
  let la_variable = "Hola 👋"; // la_variable empieza a ser válida desde este punto.

  // Podemos usar la_variable 
} 
// El contexto termina. la_variable ya no es válida.
```


## Memoria y como la distribuimos

Para esta sección el libro usa el tipo `String` como base, pero los conceptos de posesión y manejo de memoria aplican para todos los tipos de datos. 

En Rust podemos definir texto de dos maneras diferentes: 

```rust
fn main() {
  /**
   * nombre: 
   * es inmutable
   * se guarda en el stack
   * es rápido y eficiente
   **/
  let nombre = "Juan";

  /**
   * mensaje: 
   * es guardado en el heap
   * puede guardar texto que no sabemos en tiempo de compilación.
   * se puede mutar 
   **/
  let mut mensaje = String.from("Hola");
  mensaje.push_str(" mundo 👋");
  println!("{}", s); //Hola mundo 👋
}
```

Para soportar texto que puede cambiar necesitamos usar el heap. Esto implica:

1. Pedir memoria al sistema operativo. `String::from`
1. Liberar la memoria cuando terminamos de usar nuestra variable. Esta acción es ejecutada por Rust automáticamente cuando al variable sale de contexto. 

    ```rust
    fn main() {
      let la_variable = "Hola 👋";
    } 
    /**
     * El contexto termina. 
    * la_variable ya no es válida.
    **/
    ```

    Una vez el contexto termina, en `}`, Rust llama automáticamente la función `drop`. 
    `drop` es donde el autor de `String` puede poner el código para liberar la memoria. (RAII en C++)
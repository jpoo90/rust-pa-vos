# Cargo

Cargo es instalado por defecto cuando instalamos Rust. 
Podemos verificar que fue instalado correctamente con 
```bash
cargo --version
```

Cargo es una herramienta dentro del ecosistema de Rust que tiene dos funcionalidades principales:

1. [Sistema de construcción](##Como-sistema-de-construcción) (build system)
1. [Manejar dependencias](##manejar-dependencias) (package manager)


## Como sistema de construcción

### Crear un proyecto nuevo 
```bash
cargo new mi-proyecto --bin
```

La bandera `--bin` es opcional y la agregamos si queremos crear un _ejecutable_. Por defecto cargo creara un proyecto tipo _librería_. 

Este comando genera una carpeta `mi-proyecto` con estos archivos:
```
├── Cargo.toml
└── src
    └── main.rs
```
En `src` deben ir todos los archivos con código. 

### Compilar un proyecto 
```bash
cargo build
```
La compilación crea un directorio nuevo `target/debug`, en este encontramos los binarios generados. 

Un archivo, `cargo.lock`, es generado para mantener las dependencias.

Si queremos compilar y correr el resultado al mismo tiempo podemos usar

```bash
cargo run
```

Si queremos solo validar que el código compila, sin necesidad de generar binarios podemos usar 
```bash
cargo check
```


### Crear un release para producción
```bash
cargo build --release
``` 

Este comando optimiza nuestro código para que sea más efectivo, por eso la compilación tomará más tiempo. Los archivos generados se encuentran `target/release/executable`


## Manejar dependencias

Los paquetes ó dependencias en Rust son llamados `crates`. 
Cada `crate` es un paquete de código en Rust con una funcionalidad especifica. 

Podemos encontrar y/o publicar paquetes en [crates.io](crates.io)

### Instalar una dependencia
Para agregar un crate a nuestro proyecto debemos actualizar la sección `[dependencies]` en el archivo .toml 

```rust
[dependencies]
crate_name = "sem.ver"
```

Después que un paquete es instalado y nuestro proyecto compilado el `cargo.lock` es actualizado.
Este archivo garantiza que podamos generar los mismos binarios en todo momento en cualquier máquina.

### Actualizar dependencias
```
cargo update
```

> Si un paquete necesita una versión _mayor_  debemos actualizar el `.toml` manualmente.

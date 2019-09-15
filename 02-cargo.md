Cargo is Rust's build system and package manager. 
It's a tool to facilitate building our code, and managing dependencies. Cargo comes installed by default with Rust verify with `cargo --version`

New cargo project 
```
cargo new project_name --bin
```

--bin if we want an executable. Otherwise it will be a library.

A new folder `project_name`  is created. It has one .toml file and a src folder with a main.rs file inside. Also git gets initialized.

All source files should go inside src, and the top level will hold readmes, licenses, configurations ...everything but code related. 

`cargo build` : compile our project. 

It genereates a target directory and the executable is inside a debug folder. `target/debug/executable`. ALso, to keep track of dependencies, a cargo.lock is ggenerated. 

`cargo run` : compile and then run in one step. 

To release use 

`cargo build --release` It will optimize for performance.The executable will be in `target/release/executable`. Compiling will take longer, that's why we have two options :) 
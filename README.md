# Rust Examples

To compile a .rs file use bellow command
> $ rustc hello.rs

To compile using cargo command, put your main function in src/main.rs
Create you Cargo.toml file with your info outside src folder
```
Cargo.toml 
[package]
name = "hello_world"
version = "0.0.1"
author = ["Álison Venâncio <alison.venancio@gmail.com>"]
```
Run cargo to compile it
> $ cargo build
To run this cargo project use the command
> $ cargo run

To create a Cargo tree project run:
> $ cargo new <project_name> --bin

To clean build binaries
> $ cargo clean

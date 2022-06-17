# Rust mandelbrot set

this is a simple cli for make images of the mandelbrot set with rust lang 

lib use:
- image
- clap
- num-complex


<img width="38%" src="https://user-images.githubusercontent.com/37389982/174195482-1e969cb4-7322-4aed-840a-493c271ee717.png">

## Use without build

```sh
cargo run <NAME_FILE> OR <NAME_FILE.png>
```

## build

```sh
cargo build --release
```
```sh
cp ./target/release/rust_mandelbrot .
```
### run

```sh
./rust_mandelbrot <NAME_FILE> OR <NAME_FILE.png>
```

## Maps
```
.
|-- Cargo.lock
|-- Cargo.toml
|-- mandelbroat.png
|-- run
|-- rust_mandelbrot
`-- src
    |-- controller
    |   `-- control.rs
    |-- images
    |   |-- fractal.rs
    |   `-- mandelbrot.rs
    |-- lib.rs
    |-- main.rs
    `-- views
        |-- default.rs
        |-- positional.rs
        |-- quick_derive.rs
        `-- settings_parser.rs
```

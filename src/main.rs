// #![allow(unused_imports)]
// #![allow(unused_imports)]

use rust_mandelbrot::views::{
    // select 1 views
    // default::cli_default,
    // positional::cli_positional,
    quick_derive::cli_quick,
    // settings_parser::cli_settings,
};

// use rust_mandelbrot::images::{
//     // select 1 images
//     // fractal::fractal,
//     mandelbrot::mandelbrot,
// };

// claping 
//  use rust_mandelbrot::claping;

fn main() {
    //Modules views
    // cli_default();
    // cli_positional();
    cli_quick();
    // cli_settings();

    // Modules images
    // fractal("fractal.png");
    // mandelbrot("mandelbrot.png");
}

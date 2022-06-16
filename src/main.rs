// use rust_mandelbrot::views::{
//     // select 1 views
//     cli_positional::cli_positional,
//     // cli_default::cli_default
// };

use rust_mandelbrot::images::{
    // select 1 images
    // fractal::fractal,
    mandelbrot::mandelbrot,
};

fn main() {
    // cli_default();
    // cli_positional()
    // fractal("fractal.png");
    mandelbrot("mandelbrot.png");
}

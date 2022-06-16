use crate::images::mandelbrot::mandelbrot;
use crate::views::quick_derive::cli_quick;

pub fn controller_fn() {
    // let cli = Cli::parse();
    let struct_quick = cli_quick();

    if let Some(name) = struct_quick.name {
        if name.ends_with(".jpg") || name.ends_with(".png") {
            // println!("{}", name);
            // fractal(&name)
            mandelbrot(&name);
        } else {
            mandelbrot(&vec![name, ".png".to_string()].concat());
        }
    }

}

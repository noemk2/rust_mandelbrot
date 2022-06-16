pub fn mandelbrot(outfile: &str) {
    let x_m_min = -2.0;
    let y_m_min = -1.2;

    let x_m_max = 1.0;
    let y_m_max = 1.2;

    let step = 0.003;
    let iter_max = 30;

    let width = get_range(y_m_min, y_m_max, step);
    let height = get_range(x_m_min, x_m_max, step);

    // let mut imgbuf = image::ImageBuffer::new(width, height);
    let mut img = image::RgbaImage::new(width, height);
    // let black =  Rgba([0, 0, 0, 255]);
    let black = image::Rgba([0, 0, 0, 255]);
    // let new_color =
    // 5 is max_iteration
    let colours = rainbow(iter_max);

    // let scale_x = 3.0 / width as f32;
    // let scale_y = 3.0 / height as f32;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let depth = mandelbrot_fn(
            transpose_coordinate(x, x_m_min, step),
            transpose_coordinate(y, y_m_min, step),
            iter_max,
        );
        //set the image colour according to depth.
        *pixel = if depth < iter_max {
            colours[depth as usize]
        } else {
            black
        };
        // Actually set the pixel. red, green, and blue are u8 values!
        // *pixel = image::Rgb([red, green, blue]);
    }

    img.save(outfile.to_string()).unwrap();
}

fn rainbow(c: u32) -> Vec<image::Rgba<u8>> {
    (0..c)
        .map(|i| {
            image::Rgba([
                sin_to_dec(c, i, 0.0 * std::f64::consts::PI * 2.0 / 3.0),
                sin_to_dec(c, i, 2.0 * std::f64::consts::PI * 2.0 / 3.0),
                sin_to_dec(c, i, 1.0 * std::f64::consts::PI * 2.0 / 3.0),
                255,
            ])
        })
        .collect()
}

fn sin_to_dec(c: u32, i: u32, phase: f64) -> u8 {
    let s = (std::f64::consts::PI / (c as f64) * 2.0 * (i as f64) + phase).sin();
    (((s * 127.0) + 128.0).floor()) as u8
}

// -- this is pre

fn transpose_coordinate(i: u32, min: f64, step: f64) -> f64 {
    min + (step * (i as f64))
}

fn get_range(min: f64, max: f64, step: f64) -> u32 {
    (((max - min) / step).floor()) as u32
}

fn mandelbrot_fn(x: f64, y: f64, imax: u32) -> u32 {
    // let a = Complex64::new(x, y);
    let a = num_complex::Complex::new(x, y);
    let mut i: u32 = 0;
    let mut z = a.clone();
    //while abs(z) < 2.0 && i < imax {
    while z.norm() < 4.0 && i < imax {
        i += 1;
        z = z * z + a;
    }
    i
}

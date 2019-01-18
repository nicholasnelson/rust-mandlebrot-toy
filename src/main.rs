extern crate image;
extern crate num_complex;

use std::f32::consts::PI;
use num_complex::Complex;

fn main() {
    let max_iterations = 10000u16;
    let img_size = 1024u32;
    let cxmin = -2f32;
    let cxmax = 1f32;
    let cymin = -1.5f32;
    let cymax = 1.5f32;
    let scalex = (cxmax - cxmin) / img_size as f32;
    let scaley = (cymax - cymin) / img_size as f32;

    // New ImgBuf
    let mut imgbuf = image::ImageBuffer::new(img_size, img_size);

    // Calculate pixel values
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = cxmin + x as f32 * scalex;
        let cy = cymin + y as f32 * scaley;

        let c = Complex::new(cx, cy);
        let mut z = Complex::new(0f32, 0f32);

        let mut i = 0;
        for t in 0..max_iterations {
            if z.norm() > 2.0 {
                break;
            }
            z = z * z + c;
            i = t;
        }

        let scalarval = i as f32 / max_iterations as f32;

        *pixel = image::Rgb([
//            ((PI * scalarval).tan().max(0.0).min(1.0) * 255.0) as u8,
//            ((PI * (scalarval - 0.33)).tan().max(0.0).min(1.0) * 255.0) as u8,
//            ((PI * (scalarval - 0.66)).tan().max(0.0).min(1.0) * 255.0) as u8,
            (((((4.0 * PI * scalarval) - 1.5) + 1.0) / 2.0).sin().max(0.0).min(1.0) * 255.0) as u8,
            (((((8.0 * PI * scalarval) - 1.5) + 1.0) / 2.0).sin().max(0.0).min(1.0) * 255.0) as u8,
            (((((16.0 * PI * scalarval) - 1.5) + 1.0) / 2.0).sin().max(0.0).min(1.0) * 255.0) as u8,
        ]);

        if x == 0 {
            println!("Completed {} of {} rows.", y, img_size);
        }
    }

    // Save image
    imgbuf.save("fractal1.png").unwrap();
}

use image::{ImageBuffer, Luma, Rgb};
use ndarray::array;
use std::error::Error;

pub fn convert_to_grayscale(
    input_path: &str,
) -> Result<ImageBuffer<Luma<u8>, Vec<u8>>, Box<dyn Error>> {
    println!("==== 画像のグレースケール化を開始 ====");

    let original_img: ImageBuffer<Rgb<u8>, Vec<u8>> = image::open(input_path)?.into_rgb8();
    let (width, height) = original_img.dimensions();
    let mut grayscale_img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);

    for (x, y, pixel) in original_img.enumerate_pixels() {
        let r: f64 = pixel[0] as f64 * 0.299;
        let g: f64 = pixel[1] as f64 * 0.587;
        let b: f64 = pixel[2] as f64 * 0.114;

        grayscale_img.put_pixel(x, y, Luma([(r + g + b) as u8]));
    }

    println!("==== 画像のグレースケール化を完了 ====");
    Ok(grayscale_img)
}

pub fn apply_sobel(
    grayscale_img: ImageBuffer<Luma<u8>, Vec<u8>>,
) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    println!("==== 画像のエッジ検出を開始 ====");

    let (width, height) = grayscale_img.dimensions();
    let mut edge_img: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(width, height);
    let gx = array![[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];
    let gy = array![[-1, -2, -1], [0, 0, 0], [1, 2, 1]];

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let target_kernel = array![
                [
                    grayscale_img.get_pixel(x - 1, y - 1)[0] as i32,
                    grayscale_img.get_pixel(x, y - 1)[0] as i32,
                    grayscale_img.get_pixel(x + 1, y - 1)[0] as i32
                ],
                [
                    grayscale_img.get_pixel(x - 1, y)[0] as i32,
                    grayscale_img.get_pixel(x, y)[0] as i32,
                    grayscale_img.get_pixel(x + 1, y)[0] as i32
                ],
                [
                    grayscale_img.get_pixel(x - 1, y + 1)[0] as i32,
                    grayscale_img.get_pixel(x, y + 1)[0] as i32,
                    grayscale_img.get_pixel(x + 1, y + 1)[0] as i32
                ]
            ];

            let gx_val = (&target_kernel * &gx).sum() as f64;
            let gy_val = (&target_kernel * &gy).sum() as f64;
            let gradient = (gx_val.powi(2) + gy_val.powi(2)).sqrt().clamp(0.0, 255.0) as u8;

            edge_img.put_pixel(x, y, Luma([gradient]));
        }
    }

    println!("==== 画像のエッジ検出を終了 ====");
    edge_img
}

use image::{self, codecs::jpeg, GenericImageView};
use std::fs::File;
use std::path::Path;

fn main() {
    // Open the input image file
    let input_path = Path::new("input.jpeg");
    let img = image::open(&input_path).unwrap();

    // Get the dimensions of the image
    let (width, height) = img.dimensions();

    // Create a new buffer to hold the modified pixel data
    let mut buffer: Vec<u8> = vec![0; (width * height * 3) as usize];

    // Process the image data by iterating over each pixel
    for (x, y, pixel) in img.pixels() {
        // Get the RGB values of the current pixel
        let red = pixel[0];
        let green = pixel[1];
        let blue = pixel[2];

        let alpha = pixel[3];
        println!("{:?}", alpha);

        // Modify the RGB values of the current pixel
        let new_red = red / 2;
        let new_green = green / 2;
        let new_blue = blue / 2;
        let new_alpha = alpha / 2;
        // Store the modified RGB values in the buffer
        let index = ((y * width + x) * 3) as usize;
        buffer[index] = new_red;
        buffer[index + 1] = new_green;
        buffer[index + 2] = new_blue;
        buffer[index + 3] = new_alpha;
    }

    // Create a new image from the modified pixel data
    let output_path = Path::new("output.jpeg");
    let new_image = image::RgbImage::from_raw(width, height, buffer).unwrap();

    // Save the new image to a file
    let mut output_file = File::create(&output_path).unwrap();
    jpeg::JpegEncoder::new(&mut output_file)
        .encode(&new_image, width, height, image::ColorType::Rgb8)
        .unwrap();
}

extern crate image;

use image::{
    DynamicImage,
    GenericImage,
    GrayImage,
    ImageBuffer,
    Pixel,
    Luma,
};


fn main() {
    let img = image::open("test.png").unwrap().grayscale();
    normal_threshold(&img, 85).save("result-normal.png").unwrap();
    adaptive_threshold(&img, 15).save("result-adaptive.png").unwrap();
}

type IntegralImage = ImageBuffer<Luma<u32>, Vec<u32>>;

fn to_integral_image(img: &DynamicImage) -> IntegralImage {
    let w = img.width();
    let h = img.height();
    let mut out = ImageBuffer::from_pixel(w, h, Luma([0u32]));
    for i in 0..w {
        let mut sum = 0u32;
        for j in 0..h {
            sum += img.get_pixel(i, j).to_luma()[0] as u32;
            if i == 0 {
                out.put_pixel(i, j, Luma([sum]));
            } else {
                let s = out.get_pixel(i - 1, j).to_luma()[0] + sum;
                out.put_pixel(i, j, Luma([s]));
            }
        }
    }
    out
}

fn min(a: u32, b: u32) -> i32 {
    (if a < b { a } else { b }) as i32
}

fn clamp(v: i32, min: u32, max: u32) -> u32 {
    if v <= (min as i32) { min }
    else if v >= (max as i32) { max }
    else { v as u32 }
}

fn normal_threshold(
    img: &DynamicImage,
    threshold: i32, // 0 ~ 100
) -> GrayImage {
    let w = img.width();
    let h = img.height();
    let mut out = ImageBuffer::new(w, h);
    for i in 0..w {
        for j in 0..h {
            let curr = img.get_pixel(i, j).to_luma()[0];
            let p = if curr <= (((100 - threshold) * 255 / 100) as u8) {
                Luma([0u8])
            } else {
                Luma([255u8])
            };
            out.put_pixel(i, j, p);
        }
    }
    out
}

fn adaptive_threshold(
    img: &DynamicImage,
    threshold: i32, // 0 ~ 100
) -> GrayImage {
    let w = img.width();
    let h = img.height();
    let half_s = (min(w, h) / 8) / 2;
    let integral_image = to_integral_image(&img);
    let mut out = ImageBuffer::new(w, h);
    for i in 0..w {
        for j in 0..h {
            let x1 = clamp((i as i32) - half_s, 1, w - 1);
            let x2 = clamp((i as i32) + half_s, 1, w - 1);
            let y1 = clamp((j as i32) - half_s, 1, h - 1);
            let y2 = clamp((j as i32) + half_s, 1, h - 1);
            let count = ((x2 - x1) * (y2 - y1)) as i32;
            let a = integral_image.get_pixel(x1 - 1, y1 - 1).to_luma()[0] as i32;
            let b = integral_image.get_pixel(x2, y1 - 1).to_luma()[0] as i32;
            let c = integral_image.get_pixel(x1 - 1, y2).to_luma()[0] as i32;
            let d = integral_image.get_pixel(x2, y2).to_luma()[0] as i32;
            let sum = d - c - b + a;
            let curr = (img.get_pixel(i, j).to_luma()[0] as i32) * count;
            let p = if curr <= (sum * (100 - threshold) / 100) {
                Luma([0u8])
            } else {
                Luma([255u8])
            };
            out.put_pixel(i, j, p);
        }
    }
    out
}

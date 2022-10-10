use nokhwa::{Camera, CameraFormat};
use image::{ImageBuffer, Rgb, Pixel};
use image::imageops::{resize, FilterType};
use std::io::{self, Write};
use colored::*;


const GRADIENT: &'static str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const STANDARD_WIDTH: u32 = 640;
const STANDARD_HEIGHT: u32 = 480;
const TERMINAL_WIDTH: u32 = 80;
const TERMINAL_HEGITH: u32 = (TERMINAL_WIDTH * STANDARD_HEIGHT) / STANDARD_WIDTH;

fn main() {
    let mut camera = get_camera();
    camera.open_stream().unwrap();
    loop {
        match get_next_frame(&mut camera) {
            None => {
                println!("Skipped!")
            },
            Some(frame) => {
                let ascii_text = as_ascii(frame);
                io::stdout().write_all(ascii_text.as_bytes()).unwrap();
                print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            }
        };
    }
}

fn get_camera() -> Camera {
    let mut camera = Camera::new(
        0,
        Some(CameraFormat::new_from(STANDARD_WIDTH, STANDARD_HEIGHT, nokhwa::FrameFormat::MJPEG, 5))
    ).unwrap();
    camera
}

fn get_next_frame(camera: &mut Camera) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let frame = camera.frame().unwrap();
    let new = ImageBuffer::from_vec(
        frame.width(),
        frame.height(),
        frame.into_vec()
    )?;

    let resized: ImageBuffer<Rgb<u8>, Vec<u8>> = resize(&new, TERMINAL_WIDTH, TERMINAL_HEGITH, FilterType::Nearest);
    return Some(resized);
} 

fn as_ascii(frame: ImageBuffer<Rgb<u8>, Vec<u8>>) -> String {
    let mut ascii_frame = String::new();
    for line in frame.rows() {
        for pixel in line.into_iter(){
            let ascii_pixel = map_pixel(pixel);
            ascii_frame.push_str(ascii_pixel.as_str());
        }
        ascii_frame.push('\n');
    }
    return ascii_frame;
}

fn map_pixel(pixel: &Rgb<u8>) -> String {
    let intensity = usize::from(pixel.0[0]) + usize::from(pixel.0[1]) + usize::from(pixel.0[2]);
    let gradient_length = GRADIENT.len();
    let index = (intensity * (gradient_length - 1)) / (255 * 3);
    let character = GRADIENT.chars().collect::<Vec<char>>()[index];
    let colorized = character.to_string().truecolor(pixel.0[0], pixel.0[1], pixel.0[2]);
    return colorized.to_string();
}

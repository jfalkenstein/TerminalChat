mod ascii_frame;


use const_format::formatcp;
use image::imageops::{resize, FilterType};
use image::{ImageBuffer, Pixel, Rgb};
use nokhwa::{Camera, CameraFormat};
use tui::layout::Rect;
use tui::text::Text;
use std::io::{self, Write};
use crate::ascii_frame::{AsciiFrame};
use tui::{
    backend::CrosstermBackend,
    widgets::{Widget, Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};


const STANDARD_WIDTH: u32 = 640;
const STANDARD_HEIGHT: u32 = 480;
const TERMINAL_WIDTH: u32 = 120;
// Ascii text is twice as tall as it is wide, so we shink it double
const TERMINAL_HEIGHT: u32 = (TERMINAL_WIDTH * STANDARD_HEIGHT) / STANDARD_WIDTH / 2;
const FRAME_RATE: u32 = 15;
const RESET_FRAME_CODE: &str = formatcp!("{esc}[2J{esc}[1;1H", esc = 27 as char);

fn main() {
    // enable_raw_mode().unwrap();
    let mut stdout = io::stdout();
    // execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
    // let backend = CrosstermBackend::new(stdout);
    // let mut terminal = Terminal::new(backend).unwrap();
    
    let mut camera = get_camera();
    camera.open_stream().unwrap();
    loop {
        match get_next_frame(&mut camera) {
            None => {
                println!("Skipped!")
            }
            Some(frame) => {
                let ascii_frame = AsciiFrame::from(frame);
                let ascii_string = ascii_frame.to_string();
                // terminal.draw(|f|{
                //     let widget = Paragraph::new(Text::raw(ascii_string));
                //     f.render_widget(widget, f.size())
                // }).unwrap();
                // terminal.clear().unwrap();
                stdout.write_all(ascii_string.as_bytes()).unwrap();
                stdout.write_all(RESET_FRAME_CODE.as_bytes()).unwrap();
            }
        };
    }
}

fn get_camera() -> Camera {
    let mut camera = Camera::new(
        0,
        Some(CameraFormat::new_from(
            STANDARD_WIDTH,
            STANDARD_HEIGHT,
            nokhwa::FrameFormat::MJPEG,
            FRAME_RATE,
        )),
    )
    .unwrap();
    camera
}

fn get_next_frame(camera: &mut Camera) -> Option<ImageBuffer<Rgb<u8>, Vec<u8>>> {
    let frame = camera.frame().unwrap();
    let new = ImageBuffer::from_vec(frame.width(), frame.height(), frame.into_vec())?;

    let resized: ImageBuffer<Rgb<u8>, Vec<u8>> =
        resize(&new, TERMINAL_WIDTH, TERMINAL_HEIGHT, FilterType::Nearest);
    return Some(resized);
}

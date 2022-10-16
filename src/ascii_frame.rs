use image::{ImageBuffer, Rgb};
use std::ops::{Index, IndexMut};
use colored::*;
use tui::widgets::Widget;

const GRADIENT: &str = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
const EMPTY_CELL: &str = " ";

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Copy, Clone)]
enum PadDirection {
    Left,
    Right
}

fn map_pixel(pixel: &Rgb<u8>) -> String {
    let intensity = usize::from(pixel.0[0]) + usize::from(pixel.0[1]) + usize::from(pixel.0[2]);
    let gradient_length = GRADIENT.len();
    let index = (intensity * (gradient_length - 1)) / (255 * 3);
    // TODO: Determine we want to reverse the characters if the terminal background is black, but not if the terminal background is white.
    let character = GRADIENT.chars().rev().collect::<Vec<char>>()[index];
    let colorized = character
        .to_string()
        .truecolor(pixel.0[0], pixel.0[1], pixel.0[2]);
    return colorized.to_string();
}


#[derive(Debug)]
pub struct AsciiFrame {
    rows: Vec<Vec<String>>,
}

impl AsciiFrame {
    fn new(width: usize, height: usize) -> AsciiFrame {
        let mut rows = Vec::with_capacity(height);
        for _ in 0..height {
            rows.push(vec![String::from(EMPTY_CELL); width]);
        }
        return AsciiFrame { rows: rows }
    }

    fn rows(&self) -> Vec<Vec<String>> {
        return self.rows.clone();
    }
    
    
    pub fn height(&self) -> usize {
        return self.rows.len();
    }

    pub fn width(&self) -> usize {
        if self.height() == 0{
            return 0
        }
        return self.rows.iter().map(|row| row.len()).max().unwrap();
    }

    // fn overlay(&mut self, frame: &AsciiFrame, at: Option<(usize, usize)>){
    //     let (start_x, start_y) = at.unwrap_or((0, 0));
    //     for x in 0..frame.width(){
    //         let real_x = start_x + x;
    //         for y in 0..frame.height(){
    //             let real_y = start_y + y;
    //             self[(real_x as isize, real_y as isize)] = frame[(x as isize, y as isize)].clone()
    //         }
    //     }
    // }

    fn normalize(&mut self, direction: PadDirection) {
        let current_width = self.width();
        for row in self.rows.iter_mut() {
            let padding_needed = current_width - row.len();
            if padding_needed > 0 {
                row.reserve_exact(current_width)
            }
            for _ in 0..padding_needed {
                match direction {
                    PadDirection::Left => row.insert(0, String::from(EMPTY_CELL)),
                    PadDirection::Right => row.push(String::from(EMPTY_CELL)),
                }
            }
        }
    }

    // fn append(&mut self, direction: Direction, frame: &AsciiFrame) {
    //     match direction {
    //         Direction::Up => {
    //             let mut new_rows = frame.rows().clone();
    //             new_rows.append(self.rows.as_mut());
    //             self.rows = new_rows;
    //             self.normalize(PadDirection::Right);
    //         },
    //         Direction::Down => {
    //             self.rows.append(frame.rows().as_mut());
    //             self.normalize(PadDirection::Right);
    //         }
    //         Direction::Left => {
    //             for (row_index, row) in frame.rows().iter().enumerate(){
    //                 for column in row.iter().rev(){
    //                     self.rows[row_index].insert(0, column.clone());
    //                 }
    //             }
    //             self.normalize(PadDirection::Left);
    //         },
    //         Direction::Right => {
    //             for (row_index, row) in frame.rows().iter().enumerate(){
    //                 self.rows[row_index].append(row.clone().as_mut());
    //             }
    //             self.normalize(PadDirection::Right);
    //         }
    //     }
    // }

    fn translate_signed_index(&self, xy: (isize, isize)) -> (usize, usize){
        let x = AsciiFrame::translate_coordinate(xy.0, self.width());
        let y = AsciiFrame::translate_coordinate(xy.1, self.height());
        return (x, y)
    }
    fn translate_coordinate(val: isize, len: usize) -> usize {
        if val >= 0 {
            return val as usize
        }
        (len as isize + val - 1) as usize
    }

}

impl From<ImageBuffer<Rgb<u8>, Vec<u8>>> for AsciiFrame {
    fn from(image: ImageBuffer<Rgb<u8>, Vec<u8>>) -> AsciiFrame {
        let mut frame = AsciiFrame::new(image.width() as usize, image.height() as usize);

        for (row_index, line) in image.rows().enumerate() {
            for (column_index, pixel) in line.into_iter().enumerate() {
                let ascii_pixel = map_pixel(pixel);
                frame[(column_index as isize, row_index as isize)] = ascii_pixel
            }
        }
        return frame;
    }
}
impl From<String> for AsciiFrame {
    fn from(text: String) -> AsciiFrame {
        let lines: Vec<Vec<String>> = text.lines().map(|s| s.chars().map(|c| c.to_string()).collect()).collect();
        let max_width = lines.iter().map(|line| line.len()).max().unwrap();
        let mut frame = AsciiFrame::new(max_width, lines.len());
        for (row_index, line) in lines.iter().enumerate() {
            for (column_index, character) in line.iter().enumerate() {
                frame[(column_index as isize, row_index as isize)] = character.to_owned()
            }
        }
        return frame;
    }
}

impl ToString for AsciiFrame {
    fn to_string(&self) -> String {
        let mut rows = Vec::with_capacity(self.rows.len());

        for row in self.rows.iter() {
            rows.push(row.join(""));
        }
        return rows.join("\n").to_string();
    }
}

impl Index<(isize, isize)> for AsciiFrame{
    type Output = String;
    
    fn index(&self, index: (isize, isize)) -> &Self::Output {
        let (x, y) = self.translate_signed_index(index);
        return &self.rows[y][x];
    }
}

impl IndexMut<(isize, isize)> for AsciiFrame {

    fn index_mut(&mut self, index: (isize, isize)) -> &mut Self::Output {
        let (x, y) = self.translate_signed_index(index);
        return &mut self.rows[y][x];
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[test]
    fn height_returns_row_count() {
        let mut frame = AsciiFrame::new(0, 0);
        frame.rows.push(vec![String::from("hi")]);
        frame.rows.push(vec![String::from("bye")]);
        
        assert_eq!(frame.height(), 2);
    }

    #[test]
    fn width_returns_length_of_longest_row(){
        let mut frame = AsciiFrame::new(1,1);
        frame.rows.push(vec![String::from("hi")]);
        frame.rows.push(vec![String::from("bye"), String::from("so long!"), String::from("and thanks for all the fish")]);
        
        assert_eq!(frame.width(), 3);
    }

    #[rstest]
    fn normalize_sets_all_rows_to_have_same_length(
        #[values(PadDirection::Left, PadDirection::Right)] 
        direction: PadDirection
    ){
        let mut frame = AsciiFrame::new(1,1);
        frame.rows.push(vec![String::from("hi")]);
        frame.rows.push(vec![String::from("bye"), String::from("so long!"), String::from("and thanks for all the fish")]);
        frame.normalize(direction);
        assert_eq!(3, frame.rows[0].len());
        match &direction {
            PadDirection::Left => assert_eq!(frame[(0, 0)], String::from(" ")),
            PadDirection::Right => assert_eq!(frame[(-1, 0)], String::from(" "))
        };
    }

    #[test]
    fn with_capacity_height_is_what_is_specified(){
        let frame = AsciiFrame::new(100, 10);
        assert_eq!(100, frame.width());
        assert_eq!(10, frame.height());
    }

    #[test]
    fn test_indexing(){
        let mut frame = AsciiFrame::new(10, 10);
        let s = String::from("hello");
        frame[(4, 4)] = s.clone();
        assert_eq!(
            frame[(4,4)],
            s.clone()
        );
    }

    
    #[test]
    fn test_from_string_creates_correct_matrix(){
        let input_string = ["Hello my name is", "Jeremiah", "Don't you want to say hi back?"].join("\n");
        let frame = AsciiFrame::from(input_string);
        print!("{}", frame.to_string())
    }


}
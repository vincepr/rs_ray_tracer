
use std::fs;

use crate::visual::canvas::Canvas;

/// Maximum Color Value .Must be less than 65536 and more than zero.
pub const COLOR_MAXVAL : u16 = 255; 

impl Canvas {
    /// converts to a .ppm string
    pub fn canvas_to_ppm(&self) -> String {
        const HEADER_MAGIC_NR: &'static str = "P3";
        const HEADER_COMMENT: &'static str = "# automatically generated plain ppm file";
        let mut lines : Vec<String> = Vec::with_capacity(self.height);
        lines.push(HEADER_MAGIC_NR.to_string());
        lines.push(HEADER_COMMENT.to_string());
        lines.push(format!("{} {}", self.width, self.height));
        lines.push(COLOR_MAXVAL.to_string());
        for row in &self.arr {
            let mut row_str = String::new();
            for (j, col) in row.iter().enumerate() {
                row_str.push_str(&col.to_string());
                // if above 70 chars we want to squeeze in a \n
                if (j+1) % 5 == 0 { 
                    row_str.push_str("\n"); 
                }
            }
            lines.push(row_str);
        }
        lines.join("\n")
    }
}

pub fn write_to_file(path: &str, data: String) {
    fs::write(path, data).expect("unable to write file");
}


#[cfg(test)]
mod tests {
    use crate::visual::color::Col;

    use super::*;

    #[test]
    fn corret_headers_ppm() {
        let (w, h) = (5,3);
        let canvas = Canvas::new(w, h);
        let string = canvas.canvas_to_ppm();
        let exp = format!("P3\n# automatically generated plain ppm file\n{w} {h}\n255\n");
        assert!(string.starts_with(&exp));
    }
    
    #[test]
    fn correct_output_ppm() {
        let (w, h) = (5,3);
        let mut canvas = Canvas::new(w, h);
        canvas.write(0, 0, Col::new(1.5, 0.0, 0.0));
        canvas.write(2, 1, Col::new(0.0, 0.5, 0.0));
        canvas.write(4, 2, Col::new(-0.5, 0.0, 1.0));
        let result = canvas.canvas_to_ppm();

        let l1 = format!("P3\n# automatically generated plain ppm file\n{w} {h}\n255\n");
        let l2 = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n\n";
        let l3 = "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n\n";
        let l4 = "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n";
        let exp =(l1.to_owned()+l2+l3+l4); 
        assert_eq!(exp, result);
    }
}
// 123 567 900 130 160 123 123 123 123 123 123 123 123 123 123 123 123
//  1   2   3   4   5   6   7   8   9  10   11  12  13  14  15  16  17
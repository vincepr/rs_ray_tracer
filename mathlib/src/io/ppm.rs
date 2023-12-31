use std::fs;

use crate::visual::canvas::Canvas;

/// Maximum Color Value .Must be less than 65536 and more than zero.
pub const COLOR_MAXVAL: u16 = 255;

impl Canvas {
    /// converts to a .ppm string
    pub fn canvas_to_ppm(&self) -> String {
        const HEADER_MAGIC_NR: &str = "P3";
        const HEADER_COMMENT: &str = "# automatically generated plain ppm file";

        let mut lines: Vec<String> = Vec::with_capacity(self.height);

        lines.push(HEADER_MAGIC_NR.to_string());
        lines.push(HEADER_COMMENT.to_string());
        lines.push(format!("{} {}", self.width, self.height));
        lines.push(COLOR_MAXVAL.to_string());

        for row in &self.arr {
            let mut row_str = String::new();
            for (j, col) in row.iter().enumerate() {
                row_str.push_str(&col.to_string());
                // if above 70 chars we must squeeze in a \n
                if (j + 1) % 5 == 0 {
                    row_str.push('\n');
                }
            }
            lines.push(row_str);
        }

        let res = lines.join("\n");

        if res.ends_with('\n') {
            return res;
        }
        res + "\n"
    }
}

pub fn write_to_file(path: &str, data: String) {
    fs::write(path, data).expect("unable to write file");
}

#[cfg(test)]
mod tests {
    use crate::visual::color::{Col, WHITE};

    use super::*;

    #[test]
    fn corret_headers_ppm() {
        let (w, h) = (5, 3);
        let canvas = Canvas::new(w, h);
        let string = canvas.canvas_to_ppm();
        let exp = format!("P3\n# automatically generated plain ppm file\n{w} {h}\n255\n");
        assert!(string.starts_with(&exp));
    }

    #[test]
    fn correct_output_ppm() {
        let (w, h) = (5, 3);
        let mut canvas = Canvas::new(w, h);
        canvas.write_px(0, 0, Col::new(1.5, 0.0, 0.0));
        canvas.write_px(2, 1, Col::new(0.0, 0.5, 0.0));
        canvas.write_px(4, 2, Col::new(-0.5, 0.0, 1.0));
        let result = canvas.canvas_to_ppm();

        let l1 = format!("P3\n# automatically generated plain ppm file\n{w} {h}\n255\n");
        let l2 = "255 0 0  0 0 0  0 0 0  0 0 0  0 0 0  \n\n";
        let l3 = "0 0 0  0 0 0  0 128 0  0 0 0  0 0 0  \n\n";
        let l4 = "0 0 0  0 0 0  0 0 0  0 0 0  0 0 255  \n";
        let exp = l1.to_owned() + l2 + l3 + l4;
        assert_eq!(exp, result);
    }

    #[test]
    fn newline_after_5_colors_ppm() {
        let (w, h) = (11, 1);
        let mut canvas = Canvas::new(w, h);
        for col in canvas.arr[0].iter_mut() {
            *col = WHITE;
        }
        let result = canvas.canvas_to_ppm();

        let l1 = format!("P3\n# automatically generated plain ppm file\n{w} {h}\n255\n");
        let l2 = "255 255 255  255 255 255  255 255 255  255 255 255  255 255 255  \n";
        let l3 = "255 255 255  255 255 255  255 255 255  255 255 255  255 255 255  \n";
        let l4 = "255 255 255  \n";
        // let l3 = "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n\n";
        // let l4 = "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n";
        let exp = l1.to_owned() + l2 + l3 + l4;
        assert_eq!(exp, result);
    }
}


use std::fs;

use crate::visual::canvas::Canvas;


impl Canvas {
    pub fn canvas_to_ppm(&self) -> String {
        const HEADER: &'static str = "P3\n# automatically generated plain ppm file";
        let mut string = format!("{}\n {} {}\n{}\n", HEADER, self.width, self.height, 255);        
        
        for row in &self.arr {
            let row_str: String = row.into_iter().map( |&col| col.to_string() + " ").collect();
            string = string + &row_str + "\n";
        }
        string
    }
}

pub fn write_to_file(path: &str, data: String) {
    fs::write(path, data).expect("unable to write file");
}


#[cfg(test)]
mod tests {
    use crate::{cmp::ApproxEq, visual::color::Col};

    use super::*;

    #[test]
    fn corret_headers_ppm() {
        let (w, h) = (5,3);
        let canvas = Canvas::new(w, h);
        let string = canvas.canvas_to_ppm();
        let exp = format!("P3\n# automatically generated plain ppm file\n {w} {h}\n255\n");
        assert!(string.starts_with(&exp));
    }
    
    #[test]
    fn correct_output_ppm() {
        let (w, h) = (5,3);
        let mut canvas = Canvas::new(w, h);
        canvas.write(0, 0, Col::new(1.5, 0.0, 0.0));
        canvas.write(2, 1, Col::new(0.0, 0.5, 0.0));
        canvas.write(4, 2, Col::new(-0.5, 0.0, 1.0));
        let string = canvas.canvas_to_ppm();

        let l1 = format!("P3\n# automatically generated plain ppm file\n {w} {h}\n255\n");
        let l2 = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 \n";
        let l3 = "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0 \n";
        let l4 = "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 \n";
        let exp =(l1.to_owned()+l2+l3+l4); 
        assert_eq!(exp, string);
    }
}
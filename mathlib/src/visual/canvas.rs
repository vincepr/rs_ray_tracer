use std::ops::{Index, IndexMut};

use super::color::Col;

// #[derive(Debug, Clone)]
// struct Row(Vec<Col>);

/// rectangular grid representing the pixels we render into
// / ```
// / | 0,0  | 0,1 | 0,2 |
// / | 1,0  | 1,1 | 1,2 |
// / | 2,0  | 2,1 | 2,2 |
// / ```
#[derive(Debug, Clone)]
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    /// underlying vec: `cols[y][x]  - [height][width] - 0.0 is top left`
    pub arr: Vec<Vec<Col>>,
}

impl Canvas {
    pub fn new(w: usize, h: usize) -> Self {
        Canvas {
            width: w,
            height: h,
            arr: vec![vec![Col::new_black(); w]; h],
        }
    }

    /// writes color-pixel to x and y choordinates. Same format as book
    pub fn write_px(&mut self, x: usize, y: usize, col: Col) -> &mut Self {
        self[y][x] = col;
        self
    }

    /// reads color-pixel from x and y choordinates. Same format as book
    pub fn read(&self, x: usize, y: usize) -> &Col {
        &self[y][x]
    }
}

impl Index<usize> for Canvas {
    type Output = [Col];

    /// returns a row - canvas[0] would be top row
    fn index(&self, index: usize) -> &Self::Output {
        &self.arr[index]
    }
}

impl IndexMut<usize> for Canvas {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.arr[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::cmp::ApproxEq;

    use super::*;

    fn is_black(col: &Col) -> bool {
        col.b.apx_eq(&0.0) && col.b.apx_eq(&0.0) && col.b.apx_eq(&0.0)
    }

    #[test]
    fn create_canvas_all_black() {
        let (w, h) = (10, 20);
        let canvas = Canvas::new(w, h);
        for row in canvas.arr {
            for col in row {
                assert!(is_black(&col));
            }
        }
    }

    #[test]
    fn create_correct_size() {
        let (w, h) = (10, 20);
        let canvas = Canvas::new(w, h);
        let target = canvas.arr[h - 1][w - 1];
        assert_eq!(w, canvas.width);
        assert_eq!(h, canvas.height);
        assert!(is_black(&target));
    }

    #[test]
    fn writes_pixel() {
        let (w, h) = (10, 20);
        let mut canvas = Canvas::new(w, h);

        canvas[20 - 1][10 - 1] = Col::new_white();

        let lowest_row = &canvas[20 - 1];
        let last_pixel = lowest_row[10 - 1];
        assert_eq!(last_pixel, Col::new_white());

        canvas
            .write_px(5, 4, Col::new_white())
            .write_px(5, 5, Col::new_white());
        assert_eq!(canvas[4][5], Col::new_white());
        assert_eq!(canvas[5][5], Col::new_white());
    }

    #[test]
    fn read_pixel() {
        let (w, h) = (10, 20);
        let mut canvas = Canvas::new(w, h);
        canvas[4][4] = Col::new_white();
        assert_eq!(*canvas.read(4, 4), Col::new_white());
    }
}

use std::time::SystemTime;

mod attempts;
mod matrix;

use attempts::Attempts;
use matrix::Matrix;

fn main() {
    let now = SystemTime::now();

    let mut matrix = Matrix::new();
    let mut attempts = Attempts::new();

    let mut col: usize = 0;
    let mut row: usize = 0;

    while row < 9 {
        while col < 9 {
            let (mut val, mut index) = attempts.get(col, row);
            while attempts.len(col, row) > 0 && matrix.check(col, row, val) == false {
                attempts.remove(col, row, index);
                if attempts.len(col, row) > 0 {
                    (val, index) = attempts.get(col, row);
                }
            }
            if attempts.len(col, row) == 0 {
                matrix.reset(col, row);
                attempts.reset(col, row);
                if col == 0 {
                    col = 8;
                    row -= 1;
                } else {
                    col -= 1;
                }
            } else {
                matrix.set(col, row, val);
                attempts.remove(col, row, index);
                col += 1;
            }
        }
        row += 1;
        col = 0;
    }

    for iy in 0..9 {
        for ix in 0..9 {
            if ix == 3 || ix == 6 {
                print!(" ");
            }
            print!("{} ", matrix.get(ix, iy));
        }
        if iy == 2 || iy == 5 {
            println!();
        }
        println!();
    }

    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{} µs", elapsed.as_micros());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }
}

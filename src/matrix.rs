pub struct Matrix {
    data: [[u8; 9]; 9],
}

impl Matrix {
    pub fn new() -> Matrix {
        Matrix { data: [[0; 9]; 9] }
    }

    pub fn get(&self, column: usize, row: usize) -> u8 {
        self.data[column][row]
    }

    pub fn set(&mut self, column: usize, row: usize, value: u8) {
        self.data[column][row] = value;
    }

    pub fn reset(&mut self, column: usize, row: usize) {
        self.data[column][row] = 0;
    }

    pub fn check(&self, column: usize, row: usize, value: u8) -> bool {
        let x = ((column as f64 / 3.0).floor() * 3.0) as usize;
        let y = ((row as f64 / 3.0).floor() * 3.0) as usize;

        for iy in 0..9 {
            if self.data[column][iy] == value {
                return false;
            }
        }

        for ix in 0..9 {
            if self.data[ix][row] == value {
                return false;
            }
        }

        for iy in y..y + 3 {
            for ix in x..x + 3 {
                if self.data[ix][iy] == value {
                    return false;
                }
            }
        }
        return true;
    }
}

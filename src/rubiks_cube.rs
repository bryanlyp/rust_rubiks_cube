struct Face {
    values: [[i32; 3]; 3],
}
impl Face {
    fn display(&self) {
        for i in 0..3 {
            for j in 0..3 {
                print!("{} ", self.values[i][j]);
            }
            println!("");
        }
    }
    fn rotate_clockwise(&self) {
        let mut temp = [[-1; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                temp[i][j] = self.values[i][j];
            }
        }
        for i in 0..3 {
            for j in 0..3 {
                self.values[j][2-i] = temp[i][j];
            }
        }
    }
}

pub enum Face_Names {
    Front,
    Left,
    Right,
    Top,
    Bottom,
    Back,
}

pub struct Cube {
    faces: [Face; 6],
}
impl Cube {
}

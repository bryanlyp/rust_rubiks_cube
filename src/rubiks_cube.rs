pub struct Cube {
    faces: [[[usize; 3]; 3]; 6],
}

impl Cube {
    pub fn init_solved() -> Cube {
        let faces: [[[usize; 3]; 3]; 6] = [
            [[0; 3]; 3], 
            [[1; 3]; 3],
            [[2; 3]; 3], 
            [[3; 3]; 3],
            [[4; 3]; 3], 
            [[5; 3]; 3],
        ];
            return Cube {
                faces: faces,
            };
    }



    pub fn display(&self) {
        for i in 0..3 {
            println!("      {} {} {}", self.faces[2][i][0], self.faces[2][i][1], self.faces[2][i][2]);
        }
        for i in 0..3 {
            print!("{} {} {} ", self.faces[4][i][0], self.faces[4][i][1], self.faces[4][i][2]);
            print!("{} {} {} ", self.faces[0][i][0], self.faces[0][i][1], self.faces[0][i][2]);
            print!("{} {} {} ", self.faces[1][i][0], self.faces[1][i][1], self.faces[1][i][2]);
            println!("");
        }

        for i in 0..3 {
            println!("      {} {} {}", self.faces[3][i][0], self.faces[3][i][1], self.faces[3][i][2]);
        }
        for i in 0..3 {
            println!("      {} {} {}", self.faces[5][i][0], self.faces[5][i][1], self.faces[5][i][2]);
        }

    }

    fn _rotate_clockwise(&mut self, front_idx: usize, right_idx: usize, left_idx: usize, top_idx: usize, bottom_idx: usize) {
        let mut arr: [[usize; 5]; 5] = [[0; 5]; 5];
        for i in 0..3 { 
            arr[0][i+1] = self.faces[top_idx][2][i];
            arr[i+1][0] = self.faces[left_idx][i][2];
            arr[i+1][4] = self.faces[right_idx][i][0];
            arr[4][i+1] = self.faces[bottom_idx][0][i];
        }
        for i in 0..3 {
            for j in 0..3 {
                arr[i+1][j+1] = self.faces[front_idx][i][j];
            }
        }
        for i in 0..3 {
            self.faces[top_idx][2][i] = arr[3-i][0];
            self.faces[left_idx][i][2] = arr[4][i+1];
            self.faces[right_idx][i][0] = arr[0][i+1];
            self.faces[bottom_idx][0][i] = arr[i+1][4];
        }
        for i in 0..3 {
            for j in 0..3 {
                self.faces[front_idx][i][j] = arr[3-j][i+1];
            }
        }
    }

    pub fn rotate_clockwise(&mut self, front_idx: usize) {
        let right_idx: usize;
        let left_idx: usize;
        let bottom_idx: usize;
        let top_idx: usize;

        match front_idx {
            0 => {
                right_idx = 1; left_idx = 4; bottom_idx = 3; top_idx = 2;
            }
            1 => {
                right_idx = 4; left_idx = 0; top_idx = 2; bottom_idx = 3;
            }
            2 => {
                right_idx = 1; left_idx = 4; top_idx = 5; bottom_idx = 0;
            }
            3 => {
                right_idx = 1; left_idx = 4; top_idx = 0; bottom_idx = 5;
            }
            4 => {
                right_idx = 0; left_idx = 1; top_idx = 2; bottom_idx = 3;
            }
            5 => {
                right_idx = 1; left_idx = 4; top_idx = 3; bottom_idx = 2;
            }
            _ => {
                println!("default to rotating front face clockwise due to invalid input");
                //TODO: add in proper error handling for invalid input
                self.rotate_clockwise(0);
                return;
            }
        }
        self._rotate_clockwise(front_idx, right_idx, left_idx, top_idx, bottom_idx);
    }
    pub fn rotate_counter_clockwise(&mut self, front_idx: usize) {
        //TODO: implement proper counter clockwise rotation?
        for _i = 0..3 {
            self.rotate_clockwise(front_idx);
        }
    }
}



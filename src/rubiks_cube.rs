use std::io;
use std::io::Write;
use rand::Rng;
//use std::collections::HashMap;

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
        for _i in 0..3 {
            self.rotate_clockwise(front_idx);
        }
    }
}

pub struct GameManager {
    cube: Cube,
    moves: Vec<u8>,
}

impl GameManager {
    pub fn new() -> GameManager {
        return GameManager {
            moves: String::from("").into_bytes(),
            cube: Cube::init_solved(),
        };
    }
    fn randomize(&mut self) {
        let faces_arr = [b'F', b'B', b'b', b'R', b'L', b'T'];
        let dir_arr = [b'C', b'c'];
        let mut rng = rand::thread_rng();

        let mut move_seq = Vec::new();
        for _i in 0..100 {
            move_seq.push(faces_arr[rng.gen_range(0..6)]);
            move_seq.push(dir_arr[rng.gen_range(0..2)]);
        }
        self.process_moves(&move_seq);

        return;
    }


    pub fn start_game(&mut self) {
        let mut move_sequence = String::new();
        let mut to_randomize = String::new();
        println!("Starting game...");
        print!("Randomize cube? [Y/N]: ");


        loop {
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut to_randomize)
                .expect("Read error, try again.");

            if to_randomize.trim() == "Y" {
                self.randomize();
                break;
            } else if to_randomize.trim() == "N" {
                break;
            } else {
                print!("Enter Y/N only: ");
                to_randomize.clear();
            }
        }
        println!("Cube initialized.");
        println!("Enter 'q' to quit or 'h' for input help.");
        loop {
            self.cube.display();
            println!("Enter sequence of moves: ");
            io::stdin()
                .read_line(&mut move_sequence)
                .expect("Read error, try again.");

            move_sequence = move_sequence
                .trim()
                .to_string();
            
            if move_sequence.len() == 1 && move_sequence == "q" {
                println!("Exiting game.");
                return;
            } else if move_sequence.len() == 1 && move_sequence == "h" {
                println!("Front face: F");
                println!("Back face: b");
                println!("Right face: R");
                println!("Left face: L");
                println!("Bottom face: B");
                println!("Top face: T");
                println!("Append C or c to denote clockwise and counter-clockwise respectively.");
            } else if move_sequence.len() % 2 == 0 {
                let byte_str = move_sequence.as_bytes().to_vec();
                self.process_moves(&byte_str);
            }
            move_sequence.clear();
        }
        //println!("{}",move_sequence);
        //GameManager::vectorize(&move_sequence);
    }

    fn process_moves(&mut self, bytes: &Vec<u8>) {
        let mut i = 0;
        let mut value: usize;
        let mut move_sequence = Vec::new();

        for byte in bytes {
            if i % 2 == 0 {
                match *byte as char {
                    'F' => {
                        value = 0;
                    }
                    'R' => {
                        value = 1;
                    }
                    'T' => {
                        value = 2;
                    }
                    'B' => {
                        value = 3;
                    }
                    'L' => {
                        value = 4;
                    }
                    'b' => {
                        value = 5;
                    }
                    _ => {
                        println!("Invalid input.");
                        return;
                    }
                }

            } else {
                match *byte as char {
                    'C' => { 
                        value = 0;
                    }
                    'c' => {
                        value = 1;
                    }
                    _ => {
                        println!("Invalid input.");
                        return;
                    }
                }
            }
            move_sequence.push(value);
            i += 1;
        }
        i = 0;
        value = move_sequence[0];
        for moves in move_sequence {
            if i % 2 == 0 {
                value = moves;
            } else {
                if moves == 0 {
                    self.cube.rotate_clockwise(value);
                } else {
                    self.cube.rotate_counter_clockwise(value);
                }
            }
            i += 1;
        }


    }

}




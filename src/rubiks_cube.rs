struct Face {
    values: [[usize; 3]; 3],
}
impl Face {
    fn init(val: usize) -> Face {
        return Face {
            values: [[val; 3]; 3],
        };

    }
}

pub struct Cube {
    faces: [Face; 6],
}

impl Cube {
    pub fn init_solved() -> Cube {
        let faces: [Face; 6] = [Face::init(0), Face::init(1), Face::init(2), Face::init(3), Face::init(4), Face::init(5)];
        return Cube {
            faces: faces,
        };
    }

    pub fn display(&self) {
        for i in 0..3 {
            println!("      {} {} {}", self.faces[2].values[i][0], self.faces[2].values[i][1], self.faces[2].values[i][2]);
        }
        for i in 0..3 {
            print!("{} {} {} ", self.faces[4].values[i][0], self.faces[4].values[i][1], self.faces[4].values[i][2]);
            print!("{} {} {} ", self.faces[0].values[i][0], self.faces[0].values[i][1], self.faces[0].values[i][2]);
            print!("{} {} {} ", self.faces[1].values[i][0], self.faces[1].values[i][1], self.faces[1].values[i][2]);
            println!("");
        }

        for i in 0..3 {
            println!("      {} {} {}", self.faces[3].values[i][0], self.faces[3].values[i][1], self.faces[3].values[i][2]);
        }
        for i in 0..3 {
            println!("      {} {} {}", self.faces[5].values[i][0], self.faces[5].values[i][1], self.faces[5].values[i][2]);
        }

    }

}



mod rubiks_cube;
fn main() {
    let mut cube = rubiks_cube::Cube::init_solved();
    cube.display();
    println!("Rotating front face clockwise...");
    cube.rotate_clockwise(0);
    cube.display();
    println!("Rotating top face clockwise...");
    cube.rotate_clockwise(2);
    cube.display();
    cube.rotate_counter_clockwise(2);
    cube.rotate_counter_clockwise(0);
    cube.display();
}

mod rubiks_cube;
fn main() {
    let cube = rubiks_cube::Cube::init_solved();
    cube.display();
}

mod rubiks_cube;
fn main() {
    let mut gm = rubiks_cube::GameManager::new();
    gm.start_game();
}

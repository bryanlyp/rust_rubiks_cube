# Rubik's Cube in Rust
## Description
This is a 3x3 Rubik's Cube implemented in Rust.

Code is currently working, and implements a functional game :)
 
The indexes are chosen to match a standard die face.

The game is played on the command line, the player inputs a sequence of moves to manipulate the cube.  

For instance, the input string "FcTCbC" will be interpreted by the program as the following sequence:

1. Substring "Fc" -> Turn front face counter-clockwise.
2. Substring "TC" -> Turn top face clockwise.
3. Substring "bC" -> Turn back face clockwise.

Set of faces: {"F": Front face, "T": Top face, "b": Back face, "B": Bottom face, "L": Left face, "R": Right face}.

Set of turning directions: {"C": clockwise, "c": counter-clockwise}.


## Todo
1. Add proper error handling.
2. Add game saving and loading functions.
3. Code refactoring.
4. Implement TUI?

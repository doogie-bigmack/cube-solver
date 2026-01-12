use rubiks_cube_solver::cube::{Cube, Move, Color};

fn main() {
    let mut cube = Cube::new(3);
    
    // Print initial state
    println!("=== SOLVED CUBE ===");
    print_cube_state(&cube);
    
    // Apply simple scramble
    println!("\n=== APPLYING SCRAMBLE: R U R' U' ===");
    cube.apply_move(Move::R);
    println!("After R:");
    print_cube_state(&cube);
    
    cube.apply_move(Move::U);
    println!("After U:");
    print_cube_state(&cube);
    
    cube.apply_move(Move::RPrime);
    println!("After R':");
    print_cube_state(&cube);
    
    cube.apply_move(Move::UPrime);
    println!("After U':");
    print_cube_state(&cube);
    
    // Generate face string
    println!("\n=== GENERATING FACE STRING ===");
    let face_string = cube_to_face_string(&cube);
    println!("Face string: {}", face_string);
    println!("Length: {}", face_string.len());
    
    // Count each character
    println!("\nCharacter counts:");
    for c in ['U', 'R', 'F', 'D', 'L', 'B'] {
        println!("  {}: {}", c, face_string.matches(c).count());
    }
    
    // Try to parse with kewb
    println!("\n=== TESTING WITH KEWB ===");
    match kewb::FaceCube::try_from(face_string.as_str()) {
        Ok(_) => println!("Success! kewb accepted the face string"),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn print_cube_state(cube: &Cube) {
    println!("  Up: {:?} {:?} {:?}", 
        cube.up.get(0, 0), cube.up.get(0, 1), cube.up.get(0, 2));
    println!("      {:?} {:?} {:?}", 
        cube.up.get(1, 0), cube.up.get(1, 1), cube.up.get(1, 2));
    println!("      {:?} {:?} {:?}", 
        cube.up.get(2, 0), cube.up.get(2, 1), cube.up.get(2, 2));
    
    println!("  Right: {:?} {:?} {:?}", 
        cube.right.get(0, 0), cube.right.get(0, 1), cube.right.get(0, 2));
    println!("         {:?} {:?} {:?}", 
        cube.right.get(1, 0), cube.right.get(1, 1), cube.right.get(1, 2));
    println!("         {:?} {:?} {:?}", 
        cube.right.get(2, 0), cube.right.get(2, 1), cube.right.get(2, 2));
    
    println!("  Front: {:?} {:?} {:?}", 
        cube.front.get(0, 0), cube.front.get(0, 1), cube.front.get(0, 2));
    println!("         {:?} {:?} {:?}", 
        cube.front.get(1, 0), cube.front.get(1, 1), cube.front.get(1, 2));
    println!("         {:?} {:?} {:?}", 
        cube.front.get(2, 0), cube.front.get(2, 1), cube.front.get(2, 2));
}

fn cube_to_face_string(cube: &Cube) -> String {
    let mut result = String::with_capacity(54);

    // Get center colors to determine face identities
    let up_center = cube.up.get(1, 1);      // White
    let right_center = cube.right.get(1, 1);  // Red
    let front_center = cube.front.get(1, 1);  // Green
    let down_center = cube.down.get(1, 1);   // Yellow
    let left_center = cube.left.get(1, 1);   // Orange
    let back_center = cube.back.get(1, 1);   // Blue

    println!("Center colors:");
    println!("  Up: {:?}", up_center);
    println!("  Right: {:?}", right_center);
    println!("  Front: {:?}", front_center);
    println!("  Down: {:?}", down_center);
    println!("  Left: {:?}", left_center);
    println!("  Back: {:?}", back_center);

    // Helper to convert color to kewb face letter based on center matching
    let color_to_char = |color: Color| -> char {
        if color == up_center {
            'U'
        } else if color == right_center {
            'R'
        } else if color == front_center {
            'F'
        } else if color == down_center {
            'D'
        } else if color == left_center {
            'L'
        } else if color == back_center {
            'B'
        } else {
            panic!("Unknown color: {:?}", color)
        }
    };

    // Up face - indices 0-8
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.up.get(row, col)));
        }
    }

    // Right face - indices 9-17
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.right.get(row, col)));
        }
    }

    // Front face - indices 18-26
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.front.get(row, col)));
        }
    }

    // Down face - indices 27-35
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.down.get(row, col)));
        }
    }

    // Left face - indices 36-44
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.left.get(row, col)));
        }
    }

    // Back face - indices 45-53
    for row in 0..3 {
        for col in 0..3 {
            result.push(color_to_char(cube.back.get(row, col)));
        }
    }

    result
}

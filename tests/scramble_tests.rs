use rubiks_cube_solver::cube::{
    scramble::{generate_scramble, Scramble, ScrambleConfig},
    Cube, Move,
};

// Test scram_001: Generate 20-move scramble
#[test]
fn test_scram_001_generate_20_move_scramble() {
    let config = ScrambleConfig::new(20, 3);
    let scramble = generate_scramble(&config);
    assert_eq!(scramble.moves.len(), 20, "Scramble should have 20 moves");
}

// Test scram_002: Scramble avoids R R sequence
#[test]
fn test_scram_002_scramble_avoids_r_r_sequence() {
    // Generate many scrambles and verify no same face in succession
    for _ in 0..100 {
        let config = ScrambleConfig::new(20, 3);
        let scramble = generate_scramble(&config);

        for i in 1..scramble.moves.len() {
            let prev_move = scramble.moves[i - 1];
            let curr_move = scramble.moves[i];

            // Check that consecutive moves are not on the same face
            let prev_face = get_face_from_move(prev_move);
            let curr_face = get_face_from_move(curr_move);

            assert_ne!(
                prev_face, curr_face,
                "Found same face in succession: {:?} followed by {:?}",
                prev_move, curr_move
            );
        }
    }
}

// Test scram_003: Scramble avoids R R' sequence
#[test]
fn test_scram_003_scramble_avoids_r_rprime_sequence() {
    // Generate many scrambles and verify no move followed by its inverse
    for _ in 0..100 {
        let config = ScrambleConfig::new(20, 3);
        let scramble = generate_scramble(&config);

        for i in 1..scramble.moves.len() {
            let prev_move = scramble.moves[i - 1];
            let curr_move = scramble.moves[i];

            // Since we're avoiding same face, R R' is already impossible
            // But let's verify this explicitly
            let prev_face = get_face_from_move(prev_move);
            let curr_face = get_face_from_move(curr_move);

            assert_ne!(
                prev_face, curr_face,
                "Found move followed by inverse (same face): {:?} followed by {:?}",
                prev_move, curr_move
            );
        }
    }
}

// Test scram_004: Scramble is random (100 scrambles unique)
#[test]
fn test_scram_004_scramble_is_random() {
    let config = ScrambleConfig::new(20, 3);
    let mut scrambles = Vec::new();

    // Generate 100 scrambles
    for _ in 0..100 {
        let scramble = generate_scramble(&config);
        scrambles.push(scramble.to_notation());
    }

    // Count unique scrambles
    let mut unique_scrambles = scrambles.clone();
    unique_scrambles.sort();
    unique_scrambles.dedup();

    // With 20 moves and 18 basic moves, there are billions of possibilities
    // We should have at least 95% unique (very conservative estimate)
    assert!(
        unique_scrambles.len() >= 95,
        "Expected at least 95 unique scrambles out of 100, got {}",
        unique_scrambles.len()
    );
}

// Test scram_005: Scramble returns valid cube state
#[test]
fn test_scram_005_scramble_returns_valid_cube_state() {
    // Test various cube sizes
    for size in [2, 3, 4, 5, 7] {
        let config = ScrambleConfig::new(20, size);
        let scramble = generate_scramble(&config);

        // Cube state should be valid
        assert!(
            scramble.cube.validate().is_ok(),
            "Scrambled cube of size {} should be valid",
            size
        );

        // Cube should not be solved
        let solved = Cube::new(size);
        assert_ne!(
            scramble.cube, solved,
            "Scrambled cube should not equal solved cube"
        );
    }
}

// Additional test: Short scramble (5 moves)
#[test]
fn test_scramble_short() {
    let config = ScrambleConfig::new(5, 3);
    let scramble = generate_scramble(&config);
    assert_eq!(scramble.moves.len(), 5);
    assert!(scramble.cube.validate().is_ok());
}

// Additional test: Long scramble (50 moves)
#[test]
fn test_scramble_long() {
    let config = ScrambleConfig::new(50, 3);
    let scramble = generate_scramble(&config);
    assert_eq!(scramble.moves.len(), 50);
    assert!(scramble.cube.validate().is_ok());
}

// Additional test: 2x2 scramble
#[test]
fn test_scramble_2x2() {
    let config = ScrambleConfig::new(15, 2);
    let scramble = generate_scramble(&config);
    assert_eq!(scramble.moves.len(), 15);
    assert!(scramble.cube.validate().is_ok());

    // 2x2 should not have slice moves
    for m in &scramble.moves {
        assert!(
            !matches!(
                m,
                Move::M
                    | Move::MPrime
                    | Move::M2
                    | Move::E
                    | Move::EPrime
                    | Move::E2
                    | Move::S
                    | Move::SPrime
                    | Move::S2
            ),
            "2x2 scramble should not contain slice moves"
        );
    }
}

// Additional test: 4x4 scramble (even cube, no slice moves)
#[test]
fn test_scramble_4x4() {
    let config = ScrambleConfig::new(40, 4);
    let scramble = generate_scramble(&config);
    assert_eq!(scramble.moves.len(), 40);
    assert!(scramble.cube.validate().is_ok());

    // 4x4 should not have slice moves (even cube)
    for m in &scramble.moves {
        assert!(
            !matches!(
                m,
                Move::M
                    | Move::MPrime
                    | Move::M2
                    | Move::E
                    | Move::EPrime
                    | Move::E2
                    | Move::S
                    | Move::SPrime
                    | Move::S2
            ),
            "4x4 scramble should not contain slice moves"
        );
    }
}

// Additional test: To notation and back
#[test]
fn test_scramble_notation_roundtrip() {
    let config = ScrambleConfig::new(10, 3);
    let scramble = generate_scramble(&config);
    let notation = scramble.to_notation();

    // Should have correct format
    let moves_in_notation = notation.split_whitespace().collect::<Vec<_>>();
    assert_eq!(moves_in_notation.len(), 10);

    // Each move should be valid notation
    for move_str in moves_in_notation {
        assert!(
            !move_str.is_empty() && move_str.len() <= 3,
            "Move notation '{}' has invalid length",
            move_str
        );
    }
}

// Additional test: Apply scramble to cube manually
#[test]
fn test_apply_scramble_manually() {
    let config = ScrambleConfig::new(10, 3);
    let scramble = generate_scramble(&config);

    // Create a fresh cube and apply moves manually
    let mut manual_cube = Cube::new(3);
    for m in &scramble.moves {
        manual_cube.apply_move(*m);
    }

    // Should match the scramble's cube state
    assert_eq!(manual_cube, scramble.cube);
}

// Additional test: Default config
#[test]
fn test_default_scramble_config() {
    let config = ScrambleConfig::default();
    assert_eq!(config.length, 20);
    assert_eq!(config.size, 3);

    let scramble = generate_scramble(&config);
    assert_eq!(scramble.moves.len(), 20);
}

// Helper function to get face identifier from move
fn get_face_from_move(m: Move) -> u8 {
    match m {
        Move::R | Move::RPrime | Move::R2 => 0,
        Move::L | Move::LPrime | Move::L2 => 1,
        Move::U | Move::UPrime | Move::U2 => 2,
        Move::D | Move::DPrime | Move::D2 => 3,
        Move::F | Move::FPrime | Move::F2 => 4,
        Move::B | Move::BPrime | Move::B2 => 5,
        Move::M | Move::MPrime | Move::M2 => 6,
        Move::E | Move::EPrime | Move::E2 => 7,
        Move::S | Move::SPrime | Move::S2 => 8,
        Move::X | Move::XPrime | Move::X2 => 0, // Treat as R
        Move::Y | Move::YPrime | Move::Y2 => 2, // Treat as U
        Move::Z | Move::ZPrime | Move::Z2 => 4, // Treat as F
    }
}

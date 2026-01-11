//! Integration tests for R1.9: State serialization (save/load)
//!
//! Tests from test-plan.md:
//! - serial_001: Serialize solved cube to JSON
//! - serial_002: Deserialize JSON to solved cube
//! - serial_003: Round-trip scrambled cube
//! - serial_004: Handle invalid JSON gracefully

use rubiks_cube_solver::cube::{Cube, Move, SerializationError};

#[test]
fn serial_001_serialize_solved_to_json() {
    let cube = Cube::new(3);
    let json = cube.to_json().expect("Serialization should succeed");

    // Verify JSON contains expected fields
    assert!(json.contains("\"version\":"));
    assert!(json.contains("\"cube\":"));
    assert!(json.contains("\"size\":3"));

    // Verify it's valid JSON
    let _: serde_json::Value = serde_json::from_str(&json)
        .expect("JSON should be valid");
}

#[test]
fn serial_002_deserialize_json_to_solved() {
    let original = Cube::new(3);
    let json = original.to_json().unwrap();

    let restored = Cube::from_json(&json)
        .expect("Deserialization should succeed");

    assert_eq!(original, restored);
    assert!(restored.is_solved());
    assert_eq!(restored.size(), 3);
}

#[test]
fn serial_003_round_trip_scrambled_cube() {
    let mut cube = Cube::new(3);

    // Apply a scramble
    let moves = vec![
        Move::R, Move::U, Move::RPrime, Move::UPrime,
        Move::R, Move::U2, Move::RPrime, Move::U,
        Move::R, Move::UPrime, Move::RPrime,
    ];

    for m in moves {
        cube.apply_move(m);
    }

    // Cube should not be solved
    assert!(!cube.is_solved());

    // Serialize and deserialize
    let json = cube.to_json().unwrap();
    let restored = Cube::from_json(&json).unwrap();

    // Should be exactly the same
    assert_eq!(cube, restored);
    assert!(!restored.is_solved());

    // Color counts should match
    assert_eq!(cube.count_colors(), restored.count_colors());
}

#[test]
fn serial_004_handle_invalid_json() {
    // Test various invalid JSON inputs

    // Completely invalid JSON
    let result = Cube::from_json("not json at all");
    assert!(result.is_err());
    match result {
        Err(SerializationError::DeserializationFailed(_)) => {}
        _ => panic!("Expected DeserializationFailed error"),
    }

    // Valid JSON but wrong structure
    let result = Cube::from_json(r#"{"wrong": "structure"}"#);
    assert!(result.is_err());

    // Empty JSON
    let result = Cube::from_json("");
    assert!(result.is_err());

    // Null
    let result = Cube::from_json("null");
    assert!(result.is_err());
}

#[test]
fn test_serialize_2x2_cube() {
    let cube = Cube::new(2);
    let json = cube.to_json().unwrap();
    let restored = Cube::from_json(&json).unwrap();

    assert_eq!(cube, restored);
    assert_eq!(restored.size(), 2);
}

#[test]
fn test_serialize_large_cube() {
    let cube = Cube::new(10);
    let json = cube.to_json().unwrap();
    let restored = Cube::from_json(&json).unwrap();

    assert_eq!(cube, restored);
    assert_eq!(restored.size(), 10);
}

#[test]
fn test_pretty_json_formatting() {
    let cube = Cube::new(2);
    let pretty_json = cube.to_json_pretty().unwrap();

    // Pretty JSON should have newlines
    assert!(pretty_json.contains('\n'));

    // Should still deserialize correctly
    let restored = Cube::from_json(&pretty_json).unwrap();
    assert_eq!(cube, restored);
}

#[test]
fn test_version_in_json() {
    let cube = Cube::new(3);
    let json = cube.to_json().unwrap();

    // Parse as generic JSON to check version field
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    let version = parsed["version"].as_u64().expect("Should have version field");
    assert_eq!(version, 1);
}

#[test]
fn test_wrong_version_error() {
    // Manually construct JSON with unsupported version
    let json = r#"{
        "version": 999,
        "cube": {
            "size": 3,
            "up": {"stickers": [["White","White","White"],["White","White","White"],["White","White","White"]], "size": 3},
            "down": {"stickers": [["Yellow","Yellow","Yellow"],["Yellow","Yellow","Yellow"],["Yellow","Yellow","Yellow"]], "size": 3},
            "front": {"stickers": [["Green","Green","Green"],["Green","Green","Green"],["Green","Green","Green"]], "size": 3},
            "back": {"stickers": [["Blue","Blue","Blue"],["Blue","Blue","Blue"],["Blue","Blue","Blue"]], "size": 3},
            "left": {"stickers": [["Orange","Orange","Orange"],["Orange","Orange","Orange"],["Orange","Orange","Orange"]], "size": 3},
            "right": {"stickers": [["Red","Red","Red"],["Red","Red","Red"],["Red","Red","Red"]], "size": 3}
        }
    }"#;

    let result = Cube::from_json(json);
    assert!(result.is_err());
    match result {
        Err(SerializationError::UnsupportedVersion { found, supported }) => {
            assert_eq!(found, 999);
            assert_eq!(supported, 1);
        }
        _ => panic!("Expected UnsupportedVersion error"),
    }
}

#[test]
fn test_multiple_serialization_rounds() {
    let mut cube = Cube::new(3);
    cube.apply_move(Move::R);
    cube.apply_move(Move::U);

    // First serialization
    let json1 = cube.to_json().unwrap();
    let cube1 = Cube::from_json(&json1).unwrap();

    // Second serialization
    let json2 = cube1.to_json().unwrap();
    let cube2 = Cube::from_json(&json2).unwrap();

    // Third serialization
    let json3 = cube2.to_json().unwrap();
    let cube3 = Cube::from_json(&json3).unwrap();

    // All should be identical
    assert_eq!(cube, cube1);
    assert_eq!(cube1, cube2);
    assert_eq!(cube2, cube3);
}

#[test]
fn test_serialize_all_cube_sizes() {
    for size in 2..=20 {
        let cube = Cube::new(size);
        let json = cube.to_json()
            .expect(&format!("Serialization failed for size {}", size));
        let restored = Cube::from_json(&json)
            .expect(&format!("Deserialization failed for size {}", size));

        assert_eq!(cube.size(), restored.size());
        assert_eq!(cube, restored);
    }
}

#[test]
fn test_serialization_preserves_state_after_moves() {
    let mut cube = Cube::new(3);

    // Apply various move types
    cube.apply_move(Move::R);
    cube.apply_move(Move::UPrime);
    cube.apply_move(Move::L2);
    cube.apply_move(Move::D);
    cube.apply_move(Move::FPrime);
    cube.apply_move(Move::B2);

    let json = cube.to_json().unwrap();
    let restored = Cube::from_json(&json).unwrap();

    // Verify exact state match
    assert_eq!(cube.up, restored.up);
    assert_eq!(cube.down, restored.down);
    assert_eq!(cube.front, restored.front);
    assert_eq!(cube.back, restored.back);
    assert_eq!(cube.left, restored.left);
    assert_eq!(cube.right, restored.right);
}

#[test]
fn test_error_messages_are_descriptive() {
    // Test serialization error display
    let err = SerializationError::SerializationFailed("test error".to_string());
    assert!(err.to_string().contains("Serialization failed"));
    assert!(err.to_string().contains("test error"));

    // Test deserialization error display
    let err = SerializationError::DeserializationFailed("parse error".to_string());
    assert!(err.to_string().contains("Deserialization failed"));
    assert!(err.to_string().contains("parse error"));

    // Test version error display
    let err = SerializationError::UnsupportedVersion { found: 2, supported: 1 };
    assert!(err.to_string().contains("version"));
    assert!(err.to_string().contains("v2"));
    assert!(err.to_string().contains("v1"));

    // Test invalid state error display
    let err = SerializationError::InvalidCubeState("bad colors".to_string());
    assert!(err.to_string().contains("Invalid cube state"));
    assert!(err.to_string().contains("bad colors"));
}

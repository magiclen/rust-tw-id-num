#![cfg(feature = "rand")]

use tw_id_num::{
    check, check_national, check_resident, generate, generate_national, generate_resident,
};

const TESTS_COUNT: usize = 10000;

#[test]
fn test_generate_national() {
    for _ in 0..TESTS_COUNT {
        let id = generate_national(None);
        assert!(check_national(id));
    }
}

#[test]
fn test_generate_resident() {
    for _ in 0..TESTS_COUNT {
        let id = generate_resident(None);
        assert!(check_resident(id));
    }
}

#[test]
fn test_generate() {
    for _ in 0..TESTS_COUNT {
        let id = generate(None);
        assert!(check(id));
    }
}

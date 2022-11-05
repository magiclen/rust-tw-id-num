use tw_id_num::*;

const WRONG_IDS: [&str; 6] =
    ["A12345678", "A123456788", "a123456789", "A80000001", "A800000013", "a800000014"];

const NATIONAL_IDS: [&str; 28] = [
    "A123456789",
    "A223456781",
    "A172571595",
    "B181009370",
    "C169458754",
    "D196831313",
    "E178155482",
    "F138163250",
    "G184812936",
    "H127931619",
    "J165790462",
    "K188000873",
    "L105926509",
    "M193275034",
    "N140833272",
    "P153330503",
    "Q195078956",
    "R185881335",
    "S182317099",
    "T173757568",
    "U151440878",
    "V150131184",
    "X191999039",
    "Y124111055",
    "W107979296",
    "Z190712568",
    "O151687359",
    "I189730444",
];

const RESIDENT_IDS: [&str; 2] = ["A800000014", "A900000016"];

#[test]
fn test_check_national() {
    for id in WRONG_IDS {
        assert!(!check_national(id));
    }

    for id in NATIONAL_IDS {
        assert!(check_national(id));
    }

    for id in RESIDENT_IDS {
        assert!(!check_national(id));
    }
}

#[test]
fn test_check_resident() {
    for id in WRONG_IDS {
        assert!(!check_resident(id));
    }

    for id in NATIONAL_IDS {
        assert!(!check_resident(id));
    }

    for id in RESIDENT_IDS {
        assert!(check_resident(id));
    }
}

#[test]
fn test_check() {
    for id in WRONG_IDS {
        assert!(!check(id));
    }

    for id in NATIONAL_IDS {
        assert!(check(id));
    }

    for id in RESIDENT_IDS {
        assert!(check(id));
    }
}

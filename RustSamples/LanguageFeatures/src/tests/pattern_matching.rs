use std::cmp::Ordering;

#[test]
pub fn enum_match() {
    enum Platform {
        Linux,
        MacOS,
        Windows,
        Unknown,
    }

    let p = Platform::Linux;
    match p {
        Platform::Linux => assert!(true),
        Platform::MacOS => assert!(false),
        _ => assert!(false)
    }
}

#[test]
pub fn int_match() {
    let age: u32 = 42;
    let mut res: String;
    match age {
        0..=18 => res = String::from("Child"),
        19..=21 => res = String::from("Young Adult"),
        22..=u32::MAX => res = String::from("Adult"),
        _ => panic!(),
    }
    assert_eq!("Adult",res);
}

#[test]
pub fn ordering_match() {
    let age: u32 = 18;
    let mut res: String;
    match age.cmp(&18) {
        Ordering::Less => res = String::from("Child"),
        Ordering::Greater => res = String::from("Adult"),
        Ordering::Equal => res = String::from("Happy Birthday!"),
        _ => panic!(),
    }
    assert_eq!("Happy Birthday!",res);
}


use std::collections::HashMap;

#[test]
pub fn hashmap(){
    let mut animals: HashMap<String, i32> = HashMap::new();
    animals.insert(String::from("cat"), 10);

    assert!(animals.contains_key(&String::from("cat")));

    assert_eq!(&10, animals.get(&String::from("cat")).unwrap());
}

#[test]
pub fn indexing_strings() {
    let s: &str = "012345";

    // Indexing into strings as if they are byte slices is not permitted in rust.
    // If you are certain the string only contains ascii, you can covert to a byte slice
    assert_eq!('0' as u8, s.as_bytes()[0]);

    // If the string contains unicode, use an iterator instead.
    assert_eq!('2', s.chars().nth(2).unwrap());
    
    let mut count: u32 = 0;
    for c in s.chars() {
        assert_eq!(c.to_string(), count.to_string());
        count += 1;
    }
    
    for (c, i) in s.chars().enumerate() {
        assert_eq!(c.to_string(), i.to_string());
    }

}

#[test]
pub fn loop_string() {
    // Split words
    {
        let s : String = String::from("The Quick Brown Fox Jumped Over The Lazy Dog.");
        let mut v: Vec<&str> = Vec::new();
        for word in s.split_whitespace()
        {
            v.push(word.clone());
        }
        assert_eq!("The", v[0]);
        v.sort();
        assert_eq!("Brown", v[0]);
    }
}

#[test]
pub fn mutate_string() {
    let mut s: &str = "0123456789";
    assert_eq!(10, s.len());
    let x = s.replace("0","x");
    assert_eq!("x", x.chars().nth(0).unwrap().to_string());
}

#[test]
pub fn slicing_strings() {
    let s: &str = "0123456789";
    let slice = &s[..5];

    assert_eq!(10, s.len());
    assert_eq!(5, slice.len());
}
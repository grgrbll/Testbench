
fn borrow_string(_s : &String) { }

fn destroy_string(_s : String) { }

#[test]
pub fn borrowing() {

    {
        let s = String::from("012345");
        destroy_string(s);
        //assert_eq!(6, s.len()) // <-- Fails to compile! 
    }
    
    {
        let s = String::from("012345");
        borrow_string(&s);
        assert_eq!(6, s.len())
    }
    
    {
        let mut s = String::from("012345");
        assert_eq!(6, s.len());
        s.push_str("6789");
        assert_eq!(10, s.len());
        {
            let a = &s;
            // s.push_str("abc"); // <-- fails to compile! borrowed objects are immutable for the duration!
            assert_eq!(10, a.len());
        }
    }
    
    {
        let mut s = String::from("012345");
        assert_eq!(6, s.len());
        s.push_str("6789");
        assert_eq!(10, s.len());
        {
            let a = & mut s;
            // s.push_str("abc"); // <-- Fails to compile! s is borrows by a. Only a can mutate.
            a.push_str("abc"); 
            assert_eq!(13, a.len());
        }
    }
}

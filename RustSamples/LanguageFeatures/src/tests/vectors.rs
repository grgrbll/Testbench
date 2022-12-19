
fn vec_compare(va: &[u32], vb: &[u32]) -> bool {
    (va.len() == vb.len()) &&  // zip stops at the shortest
     va.iter()
       .zip(vb)
       .all(|(a,b)| *a == *b)
}

#[test]
pub fn vector() {
    // let vector: Vec<i32> = Vec::new();

    // Indexing
    {
        let vector = vec![0, 1, 2, 3, 4];
        assert_eq!(0, vector[0])
    }

    // Slicing
    {
        let vector = vec![0, 1, 2, 3, 4];
        let slice = &vector[1..4];
        assert_eq!(&1, slice.first().unwrap());
        assert_eq!(&3, slice.last().unwrap());      
    }

    // Appending
    {
        let mut vector = vec![0, 1, 2, 3, 4];
        vector.push(10);
        assert_eq!(10, *vector.last().unwrap());
    }

    // Extending
    {
        let mut vector1 = vec![0, 1, 2, 3, 4];
        let vector2 = vec![5, 6, 7, 8, 9];
        vector1.extend_from_slice(&vector2);
    }
}

#[test]
pub fn vector_iterators() {

    // Looping read only borrow
    {
        let mut vector = vec![0, 1, 2, 3, 4];
        let mut i = 0;
        for v in vector.iter() {
            assert_eq!(i, *v);
            i += 1;
        }
    }

    // Looping consume 
    {
        let mut vector = vec![0, 1, 2, 3, 4];
        let mut i = 0;
        for v in vector.into_iter() {
            assert_eq!(i, v);
            i += 1;
        }
        //assert_eq!(5, vector.len()) // --> Does not compile. into_iter consumed vector.
    }
    
    // Looping mutable 
    {
        let mut vector = vec![0, 1, 2, 3, 4];
        let mut i = 0;
        for v in vector.iter_mut() {
            assert_eq!(i, *v);
            *v += 1;
            i += 1;
        }
        assert!(vec_compare(&vector[..], &(vec![1, 2, 3, 4, 5])[..]));
    }
}

#[test]
pub fn vector_functional() {
    
    // for each 
    {
        let mut vector = vec![0, 1, 2, 3, 4];
        let mut i = 0;
        vector.iter().for_each(|v| {
            assert_eq!(i, *v);
            i += 1;
        })
    }

    // map 
    {
        let mut vector = vec![0, 1, 2, 3, 4];
        let res = vector.into_iter()
               .map(|v| {v * 2});

        let mut i = 0;
        res.for_each(|v| {
            assert!(v % 2 == 0);
            i += 1;
        })
    }
}


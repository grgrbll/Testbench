use itertools::Itertools;


#[test]
fn map() {
    {
        let even = (0..10).map(|x|2*x).collect_vec();
        assert!(even.iter().all(|x| x%2 == 0));
    }
    
    {
        let fizzbuzz = (0..100).filter_map(|x| match x {
            y if y % 3 == 0 && y % 5 == 0 => Some(String::from("FizzBuzz")),
            y if y % 3 == 0 => Some(String::from("Fizz")),
            y if y % 5 == 0 => Some(String::from("Buzz")),
            _ => None,
        }).collect_vec();

        let expected_len = 100/3 + 100/5 - 100/15;
        assert_eq!(expected_len, fizzbuzz.len());
    }

    // fold
    {
        assert_eq!(((100.0/2.0) * 201.0) as i32, (51..=150).fold(0, |sum, val| sum + val));
    }
}

#[test]
fn zip_and_interleave() {
    let even = (0..10).step_by(2).collect_vec();
    let odd = (1..10).step_by(2).collect_vec();
    
    let all : Vec<i32> = (0..10).step_by(2).interleave((1..10).step_by(2)).collect();
    assert!(all.into_iter().zip((0..10)).all(|(a,b)|{a==b}));
}
fn solution(num: i32) -> i32 {
  // time to code
   let mut sum = 0;
    for i in 1..num {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }
    sum
}
fn disemvowel(s: &str) -> String {
    s.chars().filter(|c| {!"aeiou".contains(*c)} ).collect()
}


fn dig_pow(n: i64, p: i32) -> i64 {
    let s : Vec<i32> = n.to_string().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();
    print!("{:?}", s);
    let zip : Vec<(&i32, i32)> =  s.iter().zip(p .. s.len() as i32).collect();
    print!("{:?}", zip);
    let total : i64 = s.iter().zip(p .. s.len() as i32).map(|(a,b)| a.pow(b as u32) as i64).sum();
    match total {
        total if total % n == 0 => total / n,
        _ => -1
    }
}

fn solution2(num: i32) -> i32 {
  // time to code
  let threes : i32 = (num-1) / 3;
  let fives : i32 = (num-1) / 5;
  let fifteens : i32 = (num-1) / 15;
    
  let mut sum = ((threes as f64 / 2.0 ) * ((1 + threes) as f64).round()) as i32 * 3;
  sum += ((fives as f64 / 2.0 ) * ((1 + fives) as f64).round()) as i32 * 5;
  sum -= ((fifteens as f64 / 2.0 ) * ((1 + fifteens) as f64).round()) as i32 * 15;
  sum
}

fn open_or_senior(data: Vec<(i32, i32)>) -> Vec<String> {
  data.iter().map(
        |x| {
            match x {(55.., 7..) => "Senior".to_owned(), _ => "Open".to_owned()} 
        }
    ).collect()
}

fn square_digits(num: u64) -> u64 {
    print!("{}", num.to_string());
    let s : &[u8] = num.to_string().as_bytes();
    let o : String = s.iter().map(|x| {x * x}).fold("".to_owned(), |s,v| {s + &v.to_string()});
    o.parse().unwrap()
}

fn get_count(string: &str) -> usize {
    string.matches(|x| "aeiou".contains(x)).count()
}
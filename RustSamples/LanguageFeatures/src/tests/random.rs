use rand::Rng;

#[test]
pub fn random_number() {
    let random_num = rand::thread_rng().gen_range(1..101);
}
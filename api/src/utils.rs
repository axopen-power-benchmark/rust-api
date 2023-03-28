use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub fn random_int() -> i32 {
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(0..1000);

    random_int
}

pub fn random_int_range(min: i32, max: i32) -> i32 {
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(min..max);

    random_int
}

pub fn random_uint_range(min: u32, max: u32) -> u32 {
    let mut rng = rand::thread_rng();
    let random_int = rng.gen_range(min..max);

    random_int
}

pub fn random_string() -> String {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();

    rand_string
}

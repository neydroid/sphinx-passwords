extern crate rand;
extern crate random_things;

use rand::Rng;
use random_things::random_string;

#[test]
fn it_returns_a_defined_length_string() {
    use rand::Rng;
    let length:u32 = rand::thread_rng().gen_range(10, 21);
    let password = random_string(length);
    assert_eq!(password.len() as u32, length)
}

#[test]
fn it_returns_random_passwords() {
    let length:u32 = rand::thread_rng().gen_range(10, 21);
    let password_a = random_string(length);
    let password_b = random_string(length);
    assert!(password_a != password_b, "Password A: {} Password B: {}", password_a, password_b)
}

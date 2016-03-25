extern crate rand;
use rand::random;

pub fn random_string(length:u32) -> String {
    (0..length).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}

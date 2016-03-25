extern crate argparse;
extern crate rand;

use rand::{random, Rng};
use argparse::{ArgumentParser, Store};

fn random_password(length:u32) -> String {
    (0..length).map(|_| (0x20u8 + (random::<f32>() * 96.0) as u8) as char).collect()
}

fn main() {
    let mut length:u32 = rand::thread_rng().gen_range(10, 21);
    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("This tool generates a random password");
        ap.refer(&mut length)
        .add_option(&["-l", "--length"], Store,
        "Length of password, defaults to a random length between 10 and 20");
        ap.parse_args_or_exit();
    }
    println!("Generated password of length {} => {}", length, random_password(length));
}

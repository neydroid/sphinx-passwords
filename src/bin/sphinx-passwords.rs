extern crate argparse;
extern crate ansi_term;
extern crate rand;
extern crate random_things;

use rand::{Rng};
use argparse::{ArgumentParser, Store};
use ansi_term::Colour;
use random_things::random_string;

fn main() {
    let mut length:u32 = rand::thread_rng().gen_range(10, 21);
    let minimum_length = 10;
    let warning_message = Colour::Red.paint("WARNING: Password length should be at least: ");

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("This tool generates a random password");
        ap.refer(&mut length)
        .add_option(&["-l", "--length"], Store,
        "Length of password, defaults to a random length between 10 and 20");
        ap.parse_args_or_exit();
    }

    if length < minimum_length {
        println!("{}{}", warning_message, Colour::Red.paint(length.to_string()));
    }

    println!("Generated password of length: {}", Colour::Green.paint(length.to_string()));
    println!("Password: {}", Colour::Green.paint(random_string(length)));
}

extern crate argparse;
extern crate ansi_term;
extern crate rand;
extern crate random_things;
extern crate clipboard;

use rand::{Rng};
use argparse::{ArgumentParser, StoreTrue, Store};
use ansi_term::Colour;
use random_things::random_string;
use clipboard::ClipboardContext;

fn main() {
    let mut length:u32 = rand::thread_rng().gen_range(10, 21);
    let minimum_length = 10;
    let warning_message = Colour::Red.paint("WARNING: Password length should be at least: ");
    let password:String;
    let mut silently = false;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("This tool generates a random password and copies it to clipboard");
        ap.refer(&mut length)
        .add_option(&["-l", "--length"], Store, "Sets the length of password, defaults to a random length between 10 and 20");
        ap.refer(&mut silently)
        .add_option(&["-s"], StoreTrue, "Mutes all messages, only returns the password");
        ap.parse_args_or_exit();
    }

    if length < minimum_length {
        println!("{}{}", warning_message, Colour::Red.paint(minimum_length.to_string()));
    }

    password = random_string(length);

    println!("Generated password of length: {}", Colour::Green.paint(length.to_string()));
    println!("Password: {}", Colour::Green.paint(password.clone()));
    copy_to_clipboard(password);
}

fn copy_to_clipboard(content:String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(content).expect("Failed to copy to clipboard");
}

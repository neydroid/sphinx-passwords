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

const MINIMUM_LENGTH:u32 = 10;

struct Options {
    length: u32,
    silently: bool
}

fn main() {
    let mut options = Options{length: rand::thread_rng().gen_range(MINIMUM_LENGTH, 21), silently: false};
    let password:String;

    {  // this block limits scope of borrows by ap.refer() method
        let mut ap = ArgumentParser::new();
        ap.set_description("This tool generates a random password and copies it to clipboard");
        ap.refer(&mut options.length)
        .add_option(&["-l", "--length"], Store, "Sets the length of password, defaults to a random length between 10 and 20");
        ap.refer(&mut options.silently)
        .add_option(&["-s"], StoreTrue, "Mutes all messages, only returns the password");
        ap.parse_args_or_exit();
    }

    password = random_string(options.length);
    print_output(options, password.clone());
    copy_to_clipboard(password);
}

fn print_output(options:Options, password:String) {
    if options.silently {
        println!("{}", Colour::Green.paint(password));
    } else {
        let warning_message = Colour::Red.paint("WARNING: Password length should be at least: ");
        if options.length < MINIMUM_LENGTH {
            println!("{}{}", warning_message, Colour::Red.paint(MINIMUM_LENGTH.to_string()));
        }
        println!("Generated password of length: {}", Colour::Green.paint(options.length.to_string()));
        println!("Password: {}", Colour::Green.paint(password));
    }
}

fn copy_to_clipboard(content:String) {
    let mut ctx = ClipboardContext::new().unwrap();
    ctx.set_contents(content).expect("Failed to copy to clipboard");
}

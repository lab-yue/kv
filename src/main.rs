extern crate clap;
use clap::{App, Arg};

fn main() {
    let key_name = "KEY";
    let value_name = "VALUE";
    let delete_name = "DELETE";
    let show_name = "SHOW";

    let matches = App::new("kv")
        .version("v0.1.0")
        .arg(
            Arg::with_name(key_name)
                .help("Get Value of Key")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name(value_name)
                .help("Set Value of the Key")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name(delete_name)
                .short("d")
                .long("delete")
                .help("delete a key-value pair"),
        )
        .arg(
            Arg::with_name(show_name)
                .short("s")
                .long("show")
                .help("show all key-value pairs"),
        )
        .get_matches();

    if matches.occurrences_of(show_name) > 0 {
        println!("show!");
        return;
    }

    if matches.occurrences_of(delete_name) > 0 {
        println!("delete!");
        return;
    }

    if let Some(key) = matches.value_of(key_name) {
        println!("Key: {}", key);
    }

    if let Some(value) = matches.value_of(value_name) {
        println!("Value: {}", value);
    }
}

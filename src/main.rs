extern crate clap;
use clap::{App, Arg};
mod store;

fn main() {
    let key_name = "KEY";
    let value_name = "VALUE";
    let delete_name = "DELETE";
    let show_name = "SHOW";
    let reset_name = "RESET";

    let matches = App::new("kv")
        .version("v0.1.0")
        .arg(
            Arg::with_name(key_name)
                .help("Get Value of the given Key")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name(value_name)
                .help("Set Value of the given Key")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name(delete_name)
                .short("d")
                .long("delete")
                .help("Delete a key-value pair"),
        )
        .arg(
            Arg::with_name(show_name)
                .short("s")
                .long("show")
                .help("Show all key-value pairs"),
        )
        .arg(
            Arg::with_name(reset_name)
                .short("r")
                .long("reset")
                .help("reset data"),
        )
        .get_matches();

    let mut store = store::new();

    if matches.occurrences_of(reset_name) > 0 {
        #[cfg(debug_assertions)]
        println!("reset!");

        store.reset();
        return;
    }

    if matches.occurrences_of(show_name) > 0 {
        #[cfg(debug_assertions)]
        println!("show!");

        for (k, v) in store.data.iter() {
            println!("{}: {}", k, v);
        }
        return;
    }

    if let Some(key) = matches.value_of(key_name) {
        #[cfg(debug_assertions)]
        dbg!(key);

        if let Some(value) = matches.value_of(value_name) {
            #[cfg(debug_assertions)]
            println!("Set {} to {}", key, value);

            store.set(key, value);
        } else {
            #[cfg(debug_assertions)]
            println!("Lookup {}", key);

            store.get(key);

            if matches.occurrences_of(delete_name) > 0 {
                #[cfg(debug_assertions)]
                println!("delete!");
                store.delete(key);
                return;
            }
        }
    }
}

// TODO - jnrl.sh kinda clone in Rust
//  *   reading from command line (DONE)
//  *   create txt files like 2020-05-17-<first sentence>.log
//  *   use semantics for timestamps like yesterday, today or tomorrow
//  *   first line is: <timestamp>: <first sentence> followed by the
//      body - <timestamp> is like 17.05.2020 or 05/17/2020
//      depending on your system locale or the one specified in the
//      config file
use std::env;

const APP_NAME: &str = "jrnl";
const CFG_FILE_NAME: &str = "jrnl.json";
const MAN_CLI_CALL: &str = "jrnl man";

fn create_jrnl_entry(args: &Vec<String>) {
    // TODO #[allow(unused_variables)] wieder entfernen, wenn implementiert
    #[allow(unused_variables)]
    let cfg = read_cfg_file(&args);
    // TODO Iteriere über die Argumente, da eine andere Konfigurationsdatei
    //      angegeben werden kann (die Reihenfolge der Argumente kann sich
    //      ändern).
    if args.len() > 2 {
        if args[2].contains("@") {
            println!(
                "TODO @EL: Adding tags {:?} to json file using standard '{}'.",
                args[2], CFG_FILE_NAME
            );
        }
    }
}

fn init_jrnl(args: &Vec<String>) {
    println!(
        "TODO @EL: Write a default json file config '{}' in '{}' directory.",
        if args.len() <= 3 {
            CFG_FILE_NAME
        } else {
            // TODO Ermittle den korrekten Namen der Konfigurationsdatei.
            args[3].trim()
        },
        if args.len() < 2 {
            APP_NAME
        } else {
            if args.len() > 3 {
                // TODO Ermittle den korrekten Pfad.
                args[3].trim()
            } else {
                panic!("Missing argument! Type '{}' for manual.", MAN_CLI_CALL)
            }
        }
    );

    write_cfg_file(&args);
}

fn print_man() {
    // TODO Schreibe ein kleines Handbuch zur Anwendung.
    println!("Manual for '{}'", APP_NAME);
    println!("-------------------------------------------------------------------------------");
    println!("TODO @EL: Write the manual!");
}

fn read_cfg_file(args: &Vec<String>) -> Vec<String> {
    println!(
        "TODO @EL: Using config file '{}'.",
        if args.len() > 2 {
            args[2].trim()
        } else {
            CFG_FILE_NAME
        }
    );

    // TODO Auslesen der JSON-Datei.
    return vec!["".to_string(), "".to_string()];
}

fn write_cfg_file(args: &Vec<String>) {
    println!(
        "TODO @EL: Writing config file '{}'.",
        if args.len() > 3 {
            args[3].trim()
        } else {
            CFG_FILE_NAME
        }
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Type '{}' for manual.", MAN_CLI_CALL);
    } else {
        match args[1].trim() {
            "cfg" => create_jrnl_entry(&args),
            "init" => init_jrnl(&args),
            "man" => print_man(),
            "today" => {
                println!(
                    "TODO @EL: How to get the current time and convert it (attention to '{}').",
                    CFG_FILE_NAME
                );
                create_jrnl_entry(&args);
            }
            "tomorrow" => {
                println!("TODO @EL: How to get the current time plus one day and convert it (attention to '{}').", CFG_FILE_NAME);
                create_jrnl_entry(&args);
            }
            "yesterday" => {
                println!("TODO @EL: How to get the current time minus one day and convert it (attention to '{}').", CFG_FILE_NAME);
                create_jrnl_entry(&args);
            }
            &_ => panic!("Unknown argument! Type '{}' for manual.", MAN_CLI_CALL),
        }
    }

    println!("\nThe training you should finish first.");
    println!("Shortcuts the path to the dark side they are.\n");
}

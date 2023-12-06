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
    println!("Manual for '{}'", APP_NAME);
    println!("-------------------------------------------------------------------------------");
    println!("Usage is as follows:");
    println!("jrnl today \"It was a productive day without any interruptions.\"\n");
    println!("As you can see in its simplest form jrnl will generate a commonmark markdown");
    println!("file with a default header. Your can change this with a template of your own.\n");
    println!("Arguments:");
    println!("  jrnl allows you to write a journal entry with different arguments ahead.\n");
    println!("  *   cfg path-to-config\\your-name.config");
    println!("      Loads your config file. If omitted jrnl defaults to default.config in the");
    println!("      directory where jrnl executable lies. If default.config does not exist,");
    println!("      it will be created for you at the first start with standard values.\n");
    println!("  *   yesterday");
    println!("      Will use the date and time of the day before for the creation of the");
    println!("      journal entry.\n");
    println!("  *   today");
    println!("      Will use the current date and time for the creation of the journal entry.");
    println!("      The argument 'today' can be omitted. jrnl will assume you write your entry");
    println!("      for today.\n");
    println!("  *   tomorrow");
    println!("      Will use the date and time of the day after for the creation of the");
    println!("      journal entry.\n");
    println!("  *   specific journal name");
    println!("      Will write to the corresponding journal. jrnl today\n");
    println!("  *   add");
    println!("      Using 'add' will alow you to add to an already written entry.");
    println!("      For example jrnl add ~/folderjournal/2021/05/05.md or");
    println!("      jrnl add ~/folderjournal/2021-05-05.md will let you add some text to your");
    println!("      entry and updating the time of change in the header of the opened file.");
    println!("      With jrnl add today (today can be omitted) you could add to the current");
    println!("      entry. jrnl add tomorrow and jrnl add yesterday will work also.");
    println!("      If you omit the file the current entry will be modified and the provided");
    println!("      String will be added as a new line. e.g. ~/folderjournal/2021/05/05.md");
    println!("      when you add something and today would be the 5th May of 2023.\n");
    println!("Configuration:");
    println!("  The configuration file is a simple key-value store. You can comment with a '#'");
    println!("  as first character per line. Currently there is not much to set.");
    println!("  *   journals");
    println!("      Here you define the journals (path-to-and-name-of-the-journal) in a comma");
    println!("      ',' separated list. E.g. journals=default,/home/pale-rider/jrnl/work,");
    println!("      /home/pale-rider/jrnl/private.");
    println!("      The last entry of the above given path is the journal name which can be");
    println!("      an cli argument as mentioned above. Default writes to the directory the");
    println!("      jrnl executable lies in. The default is 'default'. You should change this.\n");
    println!("  *   mode");
    println!("      Can be either mode=folders or mode=files. The default is mode=files.");
    println!("      Mode 'folders' will generate a journal entry like this:");
    println!("                                  ~/folderjournal/2021/05/05.md.");
    println!("      Mode 'files' will generate a journal entry like this:");
    println!("                                  ~/folderjournal/2021-05-05.md.\n");
    println!("  *   encryption");
    println!("      This enables encryption with encryption=enabled for your notes.");
    println!("      The default is encryption=disabled.");
    println!("      Maybe you should change this, if you plan to store them on someone else'");
    println!("      computer.\n");
    println!("  *   editor");
    println!("      This gives you the possibility to use an external editor of your choice for");
    println!("      finishing your journal entry. For example editor=nvim will start NeoVIM");
    println!("      with the freshly generated file. The default is editor=none.\n");
    println!("  *   template");
    println!("      You have the option to set up a template commonmark markdown file for your");
    println!("      journal entries with e.g. template=/home/pale-rider/jrnl/my-jrnl-entry.md.");
    println!("      By default it is set to template=none.\n");
    println!("  *   stardate");
    println!("      When enabled with stardate=enabled it will generate a stardate to display");
    println!("      in the header of the journal entry. Default is stardate=disabled.\n");
    println!("  *   editing_mark");
    println!("      With editing_mark=enabled every editing through jrnl add is traceable");
    println!("      because it will add the timestamp above the added lines.");
    println!("      Default is editing_mark=disabled.\n");
    println!("\nThe training you should finish first.");
    println!("Shortcuts the path to the dark side they are.\n");
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

// TODO - jnrl.sh kinda clone in Rust
//  *   reading from command line (DONE)
//  *   create txt files like 2020-05-17-<first sentence>.log
//  *   use semantics for timestamps like yesterday, today or tomorrow
//  *   first line is: <timestamp>: <first sentence> followed by the
//      body - <timestamp> is like 17.05.2020 or 05/17/2020
//      depending on your system locale or the one specified in the
//      config file
use std::env;
use std::fs::File;
use std::io::prelude::*;

const CFG_FILE_NAME: &str = "jrnl.cfg";
const MAN_CLI_CALL: &str = "jrnl man";

fn create_jrnl_entry(args: &Vec<String>) {
    // TODO Write only if config file is not provided and no default
    //      jrnl.cfg could be found.
    write_cfg_file(&args);
    // TODO #[allow(unused_variables)] wieder entfernen, wenn implementiert
    #[allow(unused_variables)]
    let cfg = read_cfg_file(&args);

    // TODO Iteriere über die Argumente, da eine andere Konfigurationsdatei
    //      angegeben werden kann (die Reihenfolge der Argumente kann sich
    //      ändern).
    if args.len() > 2 {
        if args[2].contains("@") {
            println!(
                "TODO @EL: Adding tags {:?} to config file using standard '{}'.",
                args[2], CFG_FILE_NAME
            );
        }
    }
}

fn print_man() {
    let man_content = "Manual for 'jrnl'
-------------------------------------------------------------------------------
Usage is as follows:
jrnl today \"It was a productive day without any interruptions.\"\n
As you can see in its simplest form jrnl will generate a commonmark markdown
file with a default header. Your can change this with a template of your own.\n
Arguments:
  jrnl allows you to write a journal entry with different arguments ahead.\n
  *   cfg path-to-config\\your-name.cfg
      Loads your config file. If omitted jrnl defaults to jrnl.cfg in the
      directory where jrnl executable lies. If jrnl.cfg does not exist,
      it will be created for you at the first start with standard values.\n
  *   yesterday
      Will use the date and time of the day before for the creation of the
      journal entry.\n
  *   today
      Will use the current date and time for the creation of the journal entry.
      The argument 'today' can be omitted. jrnl will assume you write your entry
      for today.\n
  *   tomorrow
      Will use the date and time of the day after for the creation of the
      journal entry.\n
  *   specific journal name
      Will write to the corresponding journal. jrnl today\n
  *   add
      Using 'add' will alow you to add to an already written entry.
      For example jrnl add ~/folderjournal/2021/05/05.md or
      jrnl add ~/folderjournal/2021-05-05.md will let you add some text to your
      entry and updating the time of change in the header of the opened file.
      With jrnl add today (today can be omitted) you could add to the current
      entry. jrnl add tomorrow and jrnl add yesterday will work also.
      If you omit the file the current entry will be modified and the provided
      String will be added as a new line. e.g. ~/folderjournal/2021/05/05.md
      when you add something and today would be the 5th May of 2023.\n
Configuration:
  The configuration file is a simple key-value store. You can comment with a '#'
  as first character per line. Currently there is not much to set.
  *   journals
      Here you define the journals (path-to-and-name-of-the-journal) in a comma
      ',' separated list. E.g. journals=default,/home/pale-rider/jrnl/work,
      /home/pale-rider/jrnl/private.
      The last entry of the above given path is the journal name which can be
      an cli argument as mentioned above. Default writes to the directory the
      jrnl executable lies in. The default is 'default'. You should change this.\n
  *   mode
      Can be either mode=folders or mode=files. The default is mode=files.
      Mode 'folders' will generate a journal entry like this:
                                  ~/folderjournal/2021/05/05.md.
      Mode 'files' will generate a journal entry like this:
                                  ~/folderjournal/2021-05-05.md.\n
  *   encryption
      This enables encryption with encryption=enabled for your notes.
      The default is encryption=disabled.
      Maybe you should change this, if you plan to store them on someone else'
      computer.\n
  *   editor
      This gives you the possibility to use an external editor of your choice for
      finishing your journal entry. For example editor=nvim will start NeoVIM
      with the freshly generated file. The default is editor=none.\n
  *   template
      You have the option to set up a template commonmark markdown file for your
      journal entries with e.g. template=/home/pale-rider/jrnl/my-jrnl-entry.md.
      By default it is set to template=none.\n
  *   stardate
      When enabled with stardate=enabled it will generate a stardate to display
      in the header of the journal entry. Default is stardate=disabled.\n
  *   editing_mark
      With editing_mark=enabled every editing through jrnl add is traceable
      because it will add the timestamp above the added lines.
      Default is editing_mark=disabled.\n
\nThe training you should finish first.
Shortcuts the path to the dark side they are.\n";

    println!("{}", man_content);
}

fn read_cfg_file(args: &Vec<String>) -> Vec<String> {
    println!(
        "TODO @EL: Using config file {}.",
        if args.len() > 2 {
            args[2].trim()
        } else {
            CFG_FILE_NAME
        }
    );

    // TODO Auslesen der config-Datei.
    return vec!["".to_string(), "".to_string()];
}

// TODO @EL remove the underscore from args when cli cfg change functionality
// is implemented.
fn write_cfg_file(_args: &Vec<String>) {
    // TODO @EL add mut when cli cfg change functionality is implemented.
    let /*mut*/ cfg_file_name = CFG_FILE_NAME;

    // TODO @EL check the args whether there is cfg keyword.

    let f = File::create(&cfg_file_name).expect("Could not create file.");

    if cfg_file_name.trim().eq(CFG_FILE_NAME.trim()) {
        println!("Writing default config file {}...", CFG_FILE_NAME);

        writeln!(&f, "# jrnl config file\n").expect("Could not write to file.");
        writeln!(
            &f,
            "# Here you define the journals (path-to-and-name-of-the-journal) in a comma ','"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# separated list. E.g. journals=default,/home/pale-rider/jrnl/work,"
        )
        .expect("Could not write to file.");
        writeln!(&f, "# /home/pale-rider/jrnl/private.").expect("Could not write to file.");
        writeln!(&f, "#").expect("Could not write to file.");
        writeln!(
            &f,
            "# The last entry of the above given path is the journal name which can be"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# an cli argument as mentioned above. Default writes to the directory the"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# jrnl executable lies in. The default is 'default'. You should change this."
        )
        .expect("Could not write to file.");
        writeln!(&f, "journal=default\n").expect("Could not write to file.");
        writeln!(
            &f,
            "# Can be either mode=folders or mode=files. The default is mode=files."
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# Mode 'folders' will generate a journal entry like this:"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "#                                               ~/folderjournal/2021/05/05.md."
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# Mode 'files' will generate a journal entry like this:"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "#                                               ~/folderjournal/2021-05-05.md."
        )
        .expect("Could not write to file.");
        writeln!(&f, "mode=files\n").expect("Could not write to file.");
        writeln!(
            &f,
            "# This enables encryption with encryption=enabled for your notes."
        )
        .expect("Could not write to file.");
        writeln!(&f, "# The default is encryption=disabled.").expect("Could not write to file.");
        writeln!(
            &f,
            "# Maybe you should change this, if you plan to store them on someone else'"
        )
        .expect("Could not write to file.");
        writeln!(&f, "# computer.").expect("Could not write to file.");
        writeln!(&f, "encryption=disabled\n").expect("Could not write to file.");
        writeln!(
            &f,
            "# This gives you the possibility to use an external editor of your choice for"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# finishing your journal entry. For example editor=nvim will start NeoVIM"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# with the freshly generated file. The default is editor=none."
        )
        .expect("Could not write to file.");
        writeln!(&f, "editor=none\n").expect("Could not write to file.");
        writeln!(
            &f,
            "# You have the option to set up a template commonmark markdown file for your"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# journal entries with e.g. template=/home/pale-rider/jrnl/my-jrnl-entry.md."
        )
        .expect("Could not write to file.");
        writeln!(&f, "# By default it is set to template=none.").expect("Could not write to file.");
        writeln!(&f, "template=none\n").expect("Could not write to file.");
        writeln!(
            &f,
            "# When enabled with stardate=enabled it will generate a stardate to display"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# in the header of the journal entry. Default is stardate=disabled."
        )
        .expect("Could not write to file.");
        writeln!(&f, "stardate=disabled\n").expect("Could not write to file.");
        writeln!(
            &f,
            "# With editing_mark=enabled every editing through jrnl add is traceable"
        )
        .expect("Could not write to file.");
        writeln!(
            &f,
            "# because it will add the timestamp above the added lines."
        )
        .expect("Could not write to file.");
        writeln!(&f, "# Default is editing_mark=disabled.").expect("Could not write to file.");
        writeln!(&f, "editing_mark=disabled").expect("Could not write to file.");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        println!("Type '{}' for manual.", MAN_CLI_CALL);
    } else {
        match args[1].trim() {
            // "set" => write_cfg_file(&args),
            "man" => print_man(),
            &_ => create_jrnl_entry(&args),
        }
    }
}

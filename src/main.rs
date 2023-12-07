use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

const CFG_FILE_NAME: &str = "jrnl.cfg";

fn create_jrnl_entry(args: &Vec<String>) {
    let cfg = read_cfg_file(&args);

    if cfg.len() != 0 {
        println!("Config not empty.");
    } else {
        eprintln!("Config not found.");
    }
}

fn print_man() {
    let man_content = "Manual for 'jrnl'
-------------------------------------------------------------------------------
Usage is as follows:
jrnl today \"It was a productive day without any interruptions.\"

As you can see in its simplest form jrnl will generate a commonmark markdown
file with a default header. Your can change this with a template of your own.

Arguments:
  jrnl allows you to write a journal entry with different arguments ahead.

  * init
        Creates the default config file jrnl.cfg in the same directory as the
        program itself is located at unless you provide
                                                cfg path-to-config\\my.cfg.
        Has to be the first argument.
        Does not work with journals, editor and template configuration entries.

  * set
        Lets you change the value-key pair in the config file. Will change the
        default config unless provided with cfg path-to-config\\my.cfg.
        Changing the encryption in my.cfg located at ~/jrnl would look like
        this: jrnl set enrcyption=enabled cfg ~/jrnl/my.cfg.
        It's possible to change multiple config entries separating them with a
        comma ',' (e.g. encryption=enabled,mode=folders).
        Has to be the first argument.

  * cfg path-to-config\\my.cfg
        Loads your config file. If omitted jrnl defaults to jrnl.cfg in the
        directory where jrnl executable lies. If jrnl.cfg does not exist or can
        not be read it will stop execution and prompt you.
        it will be created for you at the first start with standard values.

  * yesterday
        Will use the date and time of the day before for the creation of the
        journal entry.

  * today
        Will use the current date and time for the creation of the journal
        entry. The argument 'today' can be omitted. jrnl will assume you write
        your entry for today.

  * tomorrow
        Will use the date and time of the day after for the creation of the
        journal entry.

  * specific journal name
        Will write to the corresponding journal. jrnl work today [...]

  * add
        Using 'add' will alow you to add to an already written entry.
        For example jrnl add ~/jrnl/2021/05/05.md or
        jrnl add ~/jrnl/2021-05-05.md will let you add some text to
        your entry and updating the time of change in the header of the opened
        file. With jrnl add today (today can be omitted) you could add to the
        current entry. jrnl add tomorrow and jrnl add yesterday will work also.
        If you omit the file the current entry will be modified and the
        provided String will be added as a new line.
        For example ~/jrnl/2021/05/05.md when you add something and today would
        be the 5th May of 2023.

Configuration:
  The configuration file is a simple key-value store. You can comment with a
  '#' as first character per line. Currently there is not much to set.

  * journals
        Here you define the journals (path-to-and-name-of-the-journal) in a
        comma ',' separated list. E.g. journals=default,
        /home/pale-rider/jrnl/work,/home/pale-rider/jrnl/private.
        The last entry of the above given path is the journal name which can be
        an cli argument as mentioned above. Default writes to the directory the
        jrnl executable lies in. The default is 'default'.
        You should change this.

  * mode
        Can be either mode=folders or mode=files. The default is mode=files.
        Mode 'folders' will generate a journal entry like this:
                                                        ~/jrnl/2021/05/05.md.
        Mode 'files' will generate a journal entry like this:
                                                        ~/jrnl/2021-05-05.md.

  * encryption
        This enables encryption with encryption=enabled for your notes.
        The default is encryption=disabled.
        Maybe you should change this, if you plan to store them on someone
        else' computer.

  * editor
        This gives you the possibility to use an external editor of your choice
        for finishing your journal entry. For example editor=nvim will start
        NeoVIM with the freshly generated file. The default is editor=none.

  * template
        You have the option to set up a template commonmark markdown file for
        your journal entries with e.g. template=~/jrnl/my-jrnl-entry.md.
        By default it is set to template=none.

  * stardate
        When enabled with stardate=enabled it will generate a stardate to
        display in the header of the journal entry.
        Default is stardate=disabled.

  * editing_mark
        With editing_mark=enabled every editing through jrnl add is traceable
        because it will add the timestamp above the added lines.
        Default is editing_mark=disabled.

The training you should finish first.
Shortcuts the path to the dark side they are.";

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
    return Vec::new();
}

fn write_cfg_file(args: &Vec<String>) {
    let mut cfg_file_name = format!("./{}", CFG_FILE_NAME);
    let mut cfg_content: String = "".to_string();
    let mut own_cfg_file: bool = false;

    for a in args {
        if own_cfg_file {
            cfg_file_name = a.trim().to_string();
            own_cfg_file = false;
        }
        if a.trim().eq("cfg".trim()) {
            own_cfg_file = true;
        }
    }

    if args[1].trim().eq("set") {
        cfg_content = fs::read_to_string(&cfg_file_name).expect("Could not read file content");
    }

    let f = File::create(&cfg_file_name).expect("Could not create file");

    if cfg_file_name
        .trim()
        .eq(format!("./{}", CFG_FILE_NAME).trim())
        || args[1].trim().eq("init")
    {
        println!("Writing default config to file {}...", cfg_file_name);

        cfg_content = "# jrnl config file

# Here you define the journals (path-to-and-name-of-the-journal) in a comma ','
# separated list. E.g. 'default,/home/pale-rider/jrnl/work,
# /home/pale-rider/jrnl/private'.
#
# The last entry of the above given path is the journal name which can be
# an cli argument as mentioned above. Default writes to the directory the
# jrnl executable lies in. The default is 'default'. You should change this.
journals=default

# Can be either 'folders' or 'files'. The default is 'files'.
# Mode 'folders' will generate a journal entry like this:
#                                               ~/folderjournal/2021/05/05.md.
# Mode 'files' will generate a journal entry like this:
#                                               ~/folderjournal/2021-05-05.md.
mode=files

# This enables encryption for your notes.
# The default is 'disabled'.
# Maybe you should change this, if you plan to store them on someone else'
# computer.
encryption=disabled

# This gives you the possibility to use an external editor of your choice for
# finishing your journal entry. For example 'nvim' will start NeoVIM
# with the freshly generated file. The default is 'none'.
editor=none

# You have the option to set up a template commonmark markdown file for your
# journal entries with e.g. '/home/pale-rider/jrnl/my-jrnl-entry.md'.
# By default it is set to 'none'.
template=none

# When enabled it will generate a stardate to display in the header of the
# journal entry. Default is 'disabled'.
stardate=disabled

# When enabled every editing through jrnl add is traceable because it will add
# the timestamp above the added lines. Default is 'disabled'.
editing_mark=disabled"
            .to_string();
    }

    match args[1].trim() {
        "set" => {
            let mut new_cfg_content: String = cfg_content.clone();

            let cfg_args: Vec<&str> = args[2].as_str().split(",").collect();

            for cfg_arg_entry in cfg_args {
                let cfg_arg: Vec<&str> = cfg_arg_entry.split("=").collect();

                if cfg_arg[1].eq("disabled") {
                    new_cfg_content = new_cfg_content
                        .replace(
                            format!("{}=enabled", cfg_arg[0]).as_str(),
                            format!("{}={}", cfg_arg[0], cfg_arg[1]).as_str(),
                        )
                        .to_string();
                }
                if cfg_arg[1].eq("enabled") {
                    new_cfg_content = new_cfg_content
                        .replace(
                            format!("{}=disabled", cfg_arg[0]).as_str(),
                            format!("{}={}", cfg_arg[0], cfg_arg[1]).as_str(),
                        )
                        .to_string();
                }
                if cfg_arg[1].eq("files") {
                    new_cfg_content = new_cfg_content
                        .replace(
                            format!("{}=folders", cfg_arg[0]).as_str(),
                            format!("{}={}", cfg_arg[0], cfg_arg[1]).as_str(),
                        )
                        .to_string();
                }
                if cfg_arg[1].eq("folders") {
                    new_cfg_content = new_cfg_content
                        .replace(
                            format!("{}=files", cfg_arg[0]).as_str(),
                            format!("{}={}", cfg_arg[0], cfg_arg[1]).as_str(),
                        )
                        .to_string();
                }
            }
            writeln!(&f, "{}", new_cfg_content).expect("Could not write to file.");
        }
        "init" => writeln!(&f, "{}", cfg_content).expect("Could not write to file."),
        &_ => todo!(), // Should not happen anyway.
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_man();
    } else {
        match args[1].trim() {
            "set" => write_cfg_file(&args),
            "init" => write_cfg_file(&args),
            &_ => create_jrnl_entry(&args),
        }
    }
}

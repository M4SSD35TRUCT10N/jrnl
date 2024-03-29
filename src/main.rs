use chrono::prelude::*;
use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

const CFG_FILE_NAME: &str = "jrnl.cfg";
// const FEATURE_NOT_IMPLEMENTED: &str = "This feature is not implemented yet.";
const CONFIG_ENTRY_NOT_IMPLEMENTED: &str = "This configuration entry is not implemented yet.";

fn create_jrnl_entry(args: &Vec<String>) {
    let mut cfg_file_name = format!("./{}", CFG_FILE_NAME);

    let mut cfg_editor: String = "".to_string();
    let mut cfg_editing_mark: bool = false;
    let mut cfg_encryption: bool = false;
    let mut cfg_journals: String = "default".to_string();
    let mut cfg_mode: String = "files".to_string();
    let mut cfg_stardate: bool = false;
    let mut cfg_template: String = "".to_string();

    let mut arg_add: bool = false;
    let mut arg_cfg: bool = false;
    let mut arg_journal: String = "".to_string();
    let mut arg_journal_entry: &str = "";
    let mut arg_tags: String = "".to_string();
    let mut arg_today: bool = false;
    let mut arg_tomorrow: bool = false;
    let mut arg_yesterday: bool = false;

    let mut jrnl_time = Local::now();
    let mut stardate_offset: f64 = 0.0;

    for a in args {
        if arg_cfg {
            cfg_file_name = a.trim().to_string();
            arg_cfg = false;
        }
        if a.trim().eq("cfg".trim()) {
            arg_cfg = true;
        }
    }

    let cfg_content: String =
        fs::read_to_string(cfg_file_name).expect("Could not read file content");

    let cfg_options: Vec<&str> = cfg_content.split("\n").collect();

    if cfg_options.len() != 0 {
        let mut cfg_options_no_comments: Vec<&str> = Vec::new();

        for co in &cfg_options {
            if !co.trim().starts_with("#") && co.len() != 0 {
                cfg_options_no_comments.push(co);
            }
        }

        for co in &cfg_options_no_comments {
            let cfg_arg: Vec<&str> = co.split("=").collect();
            if cfg_arg[0].to_string().eq("encryption") || cfg_arg[0].to_string().eq("template") {
                eprintln!("{} {}", cfg_arg[0], CONFIG_ENTRY_NOT_IMPLEMENTED);
            }
            if cfg_arg[0].to_string().eq("editor") {
                cfg_editor = cfg_arg[1].to_string();
            }
            if cfg_arg[0].to_string().eq("editing_mark") {
                cfg_editing_mark = cfg_arg[1].to_string().eq("enabled");
            }
            if cfg_arg[0].to_string().eq("encryption") {
                cfg_encryption = cfg_arg[1].to_string().eq("enabled");
            }
            if cfg_arg[0].to_string().eq("journals") {
                cfg_journals = cfg_arg[1].to_string();
            }
            if cfg_arg[0].to_string().eq("mode") {
                cfg_mode = cfg_arg[1].to_string();
            }
            if cfg_arg[0].to_string().eq("stardate") {
                cfg_stardate = cfg_arg[1].to_string().eq("enabled");
            }
            if cfg_arg[0].to_string().eq("template") {
                cfg_template = cfg_arg[1].to_string();
            }
        }
    } else {
        eprintln!("Config not found.");
    }

    for a in args {
        if arg_journal.eq("") {
            let journals_entries_tmp = cfg_journals.clone();
            let journal_entries = journals_entries_tmp.split(",");
            for entry in journal_entries {
                let journal_path_parts: Vec<_> = entry.split("/").collect();
                if journal_path_parts[journal_path_parts.len() - 1].eq(a.trim()) {
                    arg_journal = entry.to_string();
                }
            }
        }
        if !arg_add {
            arg_add = a.trim().eq("add".trim());
        }
        if !arg_yesterday && !arg_today && !arg_tomorrow {
            arg_yesterday = a.trim().eq("yesterday".trim());
            arg_today = a.trim().eq("today".trim());
            arg_tomorrow = a.trim().eq("tomorrow".trim());
        }
        if arg_tags.eq("") && a.trim().starts_with("@") {
            arg_tags = a.trim().replace("@", "").to_string();
        }

        if a.trim().eq(&args[&args.len() - 1].to_string())
            && a.trim().ne("add".trim())
            && a.trim().ne("cfg".trim())
            && !a.trim().starts_with("@")
            && a.trim().ne("today".trim())
            && a.trim().ne("tomorrow".trim())
            && a.trim().ne("yesterday".trim())
            && a.trim().matches("\\").count() == 0
            && a.trim().matches("/").count() == 0
        // TODO: ignore journal names too
        {
            arg_journal_entry = a.trim();
        }
    }

    // take care of ommitable arguments
    if !arg_yesterday && !arg_today && !arg_tomorrow {
        arg_today = true;
    }

    if cfg_encryption {}
    if cfg_template.ne("") {}

    if arg_today {}
    if arg_tomorrow {
        jrnl_time = jrnl_time.checked_add_days(chrono::Days::new(1)).unwrap();
        stardate_offset = 86400000.0;
    }
    if arg_yesterday {
        jrnl_time = jrnl_time.checked_sub_days(chrono::Days::new(1)).unwrap();
        stardate_offset = -86400000.0;
    }

    if arg_journal.eq("") {
        arg_journal = "default".to_string();
    }

    let mut journal_file_name: String = ".md".to_string();
    if cfg_mode.eq("files") {
        journal_file_name = format!(
            "{}{}{}",
            if arg_journal.eq("default") {
                "".to_string()
            } else {
                format!("{}/", arg_journal.as_str())
            },
            jrnl_time.date_naive().to_string(),
            journal_file_name
        );
    }
    if cfg_mode.eq("folders") {
        fs::create_dir_all(format!(
            "{}/{}/{}",
            if arg_journal.eq("default") {
                ".".to_string()
            } else {
                format!("{}", arg_journal.as_str())
            },
            jrnl_time.year().to_string(),
            jrnl_time.month().to_string()
        ))
        .expect("Couldn't create folders.");
        journal_file_name = format!(
            "{}/{}/{}/{}{}",
            if arg_journal.eq("default") {
                ".".to_string()
            } else {
                format!("{}", arg_journal.as_str())
            },
            jrnl_time.year().to_string(),
            jrnl_time.month().to_string(),
            jrnl_time.day().to_string(),
            journal_file_name
        );
    }

    let stardate_time = (((Local::now().timestamp_millis() as f64 + stardate_offset
        - Utc
            .with_ymd_and_hms(1987, 7, 15, 0, 0, 0)
            .unwrap()
            .timestamp_millis() as f64)
        / 3155760.0)
        + 410000.0)
        .floor()
        / 10.0;

    let default_header: String = format!(
        "---\ntags: [{}]\ntitle: '{}'\ncreated: '{}'\nmodified: '{}'\n---\n\n# {}{}\n",
        if arg_tags.ne("") {
            arg_tags.as_str()
        } else {
            "Daily Report"
        },
        jrnl_time.date_naive(),
        jrnl_time,
        jrnl_time,
        jrnl_time.date_naive(),
        if cfg_stardate {
            format!(", Stardate: {}", stardate_time.to_string())
        } else {
            "".to_string()
        }
    );

    if cfg_stardate && arg_add && cfg_editing_mark {
        let journal_file_content = fs::read_to_string(&journal_file_name)
            .expect(format!("Couldn't open file {}.", &journal_file_name).as_str());
        fs::write(
            &journal_file_name,
            format!(
                "{}\n({} Supplemental [Stardate: {}])\n{}",
                &journal_file_content,
                &jrnl_time.to_string(),
                stardate_time.to_string(),
                arg_journal_entry
            ),
        )
        .expect(format!("Could not write file {}.", &journal_file_name).as_str());
    }
    if !cfg_stardate && arg_add && cfg_editing_mark {
        let journal_file_content = fs::read_to_string(&journal_file_name)
            .expect(format!("Couldn't open file {}.", &journal_file_name).as_str());
        fs::write(
            &journal_file_name,
            format!(
                "{}({})\n{}",
                &journal_file_content,
                &jrnl_time.to_string(),
                arg_journal_entry
            ),
        )
        .expect(format!("Could not write file {}.", &journal_file_name).as_str());
    }
    if (!cfg_stardate && arg_add && !cfg_editing_mark)
        || (cfg_stardate && arg_add && !cfg_editing_mark)
    {
        let journal_file_content = fs::read_to_string(&journal_file_name)
            .expect(format!("Couldn't open file {}.", &journal_file_name).as_str());
        fs::write(
            &journal_file_name,
            format!("{}\n{}", &journal_file_content, arg_journal_entry),
        )
        .expect(format!("Could not write file {}.", &journal_file_name).as_str());
    }

    if !arg_add {
        fs::write(
            &journal_file_name,
            format!("{}\n{}", default_header, arg_journal_entry),
        )
        .expect(format!("Could not write file {}.", &journal_file_name).as_str());
    }

    if cfg_editor.ne("") && cfg_editor.ne("none") {
        let mut run_editor = Command::new(&cfg_editor);
        run_editor.arg(&journal_file_name);
        run_editor
            .spawn()
            .expect(format!("{} not found", &cfg_editor).as_str());
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

  * @ (tags)
        Using @ in a double quotation mark will overwrite the default tag in
        the default header and in the personalized template (if implemented).
        jrnl today \"@Daily Report, Rust, NeoVIM, Lapce\" \"It's fun!\" will
        add the tags Daily Report, Rust, NeoVIM and Lapce to your journal entry.

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
            "set" | "init" => write_cfg_file(&args),
            &_ => create_jrnl_entry(&args),
        }
    }
}

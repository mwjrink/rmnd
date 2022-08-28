#![feature(slice_group_by)]

mod reminder;
mod configFile;

use core::panic;
use std::{ffi::OsString, fs, path::{Path, PathBuf}, env::current_dir};

use clap::{builder::NonEmptyStringValueParser, Arg, ArgAction, ArgMatches, Command};
use colored::Colorize;
use configFile::ConfigSum;
use reminder::{LocalReminder, Reminder, Priority, Color};
use text_io::read;

use crate::configFile::ConfigFile;

const CONFIG_DIR: &str = "/Users/maxrink/.config/";
const CONFIG_NAME: &str = r"rmnd.toml";

/// This function defines the structure of the command, the help descriptions, and
/// some parameter validation. No other command functionality is defined here.
fn cli() -> Command<'static> {
    Command::new("rmnd")
        .about("A CLI tool for reminders.")
        .subcommand_required(false)
        .arg_required_else_help(true) // TODO: FALSE
        .allow_external_subcommands(false)
        // Show
        .subcommand(
            Command::new("show")
                .alias("s")
                .about("Show reminders for the current context.")
                .arg(
                    Arg::new("all")
                        .long("--all")
                        .short('a')
                        .action(ArgAction::SetTrue)
                        .takes_value(false)
                        .help("Show all reminders, including all contextual reminders across the entire system."),
                )
                .arg(
                    Arg::new("priorities")
                        .long("--priorities")
                        .short('p')
                        .action(ArgAction::Set)
                        .help("Show reminders with specific priorities. Takes a comma delimited list of priorities by name or id. (surround in quotes)"),
                )
                .arg(
                    Arg::new("show-ids")
                        .long("--show-ids")
                        .short('i')
                        .action(ArgAction::SetTrue)
                        .takes_value(false)
                        .help("Show ids for all displayed reminders, useful for commands that can take an id (remove, edit)."),
                ),
        )
        // Add
        .subcommand(
            Command::new("add")
                .alias("a")
                .about("Add a reminder or priority.")
                .arg(
                  Arg::new("global")
                      .long("--global")
                      .short('g')
                      .action(ArgAction::SetTrue)
                      .takes_value(false)
                      .help("Add to the global context."),
              )
              .arg(Arg::new("reminder").action(ArgAction::Set).value_parser(NonEmptyStringValueParser::new()))
              .subcommand(
               Command::new("reminder")
               .about("Add a reminder.")
               .aliases(&["remind", "r"])
               .arg(Arg::new("priority").short('p').action(ArgAction::Set).required(false))
               .arg_required_else_help(true)
               .arg(Arg::new("reminder").required(true).action(ArgAction::Set).value_parser(NonEmptyStringValueParser::new())))
              .subcommand(
                Command::new("priority")
                .about("Add a priority.")
                .aliases(&["prio", "p"])
                .arg_required_else_help(true)
                .arg(Arg::new("priority").required(true).action(ArgAction::Set).value_parser(NonEmptyStringValueParser::new()))
                .arg(Arg::new("color").long("--color").short('c').required(false).action(ArgAction::Set).value_parser(NonEmptyStringValueParser::new())))
        )
        // Remind
        .subcommand(
         Command::new("remind")
             .alias("r")
             .about("Add, remove or edit a reminder.")
             .arg(Arg::new("global").long("--global").short('g').action(ArgAction::SetTrue).takes_value(false).help("Operation prioritizes global over contextual."),)
             .arg(Arg::new("reminder").action(ArgAction::Set).takes_value(true).help("The reminder to be added."),)
             .subcommand(Command::new("add").about("Add a reminder.").long_flag("--add").short_flag('a').arg(Arg::new("reminder").action(ArgAction::Set).takes_value(true).help("The reminder to be added."),))
             .subcommand(Command::new("remove").about("Remove a reminder").long_flag("--remove").short_flag('r').arg(Arg::new("reminder").action(ArgAction::Set).takes_value(true).help("The reminder to be removed, id or name."),))
             .subcommand(Command::new("edit").about("Edit a reminder.").long_flag("--edit").short_flag('e').arg(Arg::new("reminder").action(ArgAction::Set).takes_value(true).help("The reminder to be modified, id or name."),))
        )
        // Remove UNSTARTED HERE DOWN
        .subcommand(
            Command::new("remove")
                .alias("r")
                .about("Remove a reminder, priority or context.")
                .arg_required_else_help(true)
                .arg(Arg::new("id"))
                .arg(Arg::new("priority"))
                .arg(Arg::new("reminder"))
                .arg(Arg::new("context"))
        )
        // Priorities
        .subcommand(
            Command::new("prio")
                .alias("priorities")
                .about("Manage priorities.")
                
        )
        // Init
        .subcommand(
            Command::new("init")
                .about("Initialize a local contextual reminder file in this directory.")
        )
}

fn read_config(path: &PathBuf) -> ConfigFile {
    // println!("Attempting to read: {:?}", path);
    match fs::read(path) {
        Ok(bytes) => {
            let loaded = toml::from_slice::<ConfigFile>(&bytes);
            if let Ok(mut success) = loaded {
                success.path = Some(path.clone());
                return success;
            } else {
                panic!("Failed to read the global config file, it may be corrupted.");
            }
        },
        Err(error) => {
            panic!("{:?}", error);
        },
    }
}

fn get_all() -> ConfigSum {
    // fs::try_exists(CONFIG_DIR);
    // fs::try_exists(CONFIG_PATH);
    // fs::create_dir(CONFIG_DIR);
    // fs::write(CONFIG_PATH, contents);
    let global_config = load_global_config();

    let mut result = ConfigSum::new();
    result.priorities = global_config.priorities;
    // does crossbeam/tokio speed up multi io at all?
    for path in global_config.config_paths { 
        let read = read_config(&Path::new(&path).to_path_buf());
        for reminder in read.reminders {
            result.reminders.push(LocalReminder { reminder, path: Path::new(&path).canonicalize().unwrap() })
        }
    }

    for reminder in global_config.reminders {
        result.reminders.push(LocalReminder { reminder, path: global_config.path.as_ref().unwrap().clone() })
    }

    return result;
}

fn get_local() -> ConfigSum {
    // fs::try_exists(CONFIG_DIR);
    // fs::try_exists(CONFIG_PATH);
    // fs::create_dir(CONFIG_DIR);
    // fs::write(CONFIG_PATH, contents);
    let global_config = load_global_config();

    let current_dir = current_dir().unwrap().canonicalize().unwrap(); // TODO no unwrap here

    // let local = current_dir.join(CONFIG_NAME);
    // if local.is_file() {
    //     let local_config = read_config(&local);
    //     // TODO add up all the reminders from above too
    //     return ConfigSum { priorities: global_config.priorities, reminders: local_config.reminders }
    // }

    let mut result = ConfigSum::new();
    result.priorities = global_config.priorities;
    // does crossbeam/tokio speed up multi io at all?
    // let mut closest = (u32::MAX, Path::new(""));
    for path in global_config.config_paths {
        let path = PathBuf::from(&path).canonicalize().unwrap();
        let container = path.parent().unwrap().to_path_buf();

        if current_dir.starts_with(&container) {
            // let remainder = current_dir.strip_prefix(path).unwrap();
            // let count = remainder.components().count();
            // if count < closest.0 {
            //     closest.0 = count;
            //     closest.1 = path;
            // }

            let read = read_config(&path);
            for reminder in read.reminders {
                result.reminders.push(LocalReminder { reminder, path: path.clone() })
            }
        }
    }

    return result;
}

fn load_global_config() -> ConfigFile {
    let CONFIG_PATH = PathBuf::from(CONFIG_DIR).canonicalize().unwrap().join(CONFIG_NAME);

    let mut global_config;

    if !Path::new(&CONFIG_DIR).exists() {
        println!("creating dir: {}", CONFIG_DIR);
        fs::create_dir(CONFIG_DIR).unwrap();
    }

    if !Path::new(&CONFIG_PATH).exists() {
        global_config = ConfigFile::default_global_config();
        let output = toml::to_string_pretty(&global_config).unwrap();
        fs::write(&CONFIG_PATH, &output);
    } else if !Path::new(&CONFIG_PATH).is_file() {
        panic!("Something exists at {}, this is the default location for the global config.", CONFIG_PATH.to_str().unwrap());
    } else {
        global_config = read_config(&CONFIG_PATH);
    }

    global_config.path = Some(CONFIG_PATH);
    global_config
}

fn find_most_local_config() -> PathBuf {
    let global_config = load_global_config();

    let current_dir = current_dir().unwrap().canonicalize().unwrap(); // TODO no unwrap here
    
    println!("0");
    let local = current_dir.join(CONFIG_NAME);
    println!("1");
    if local.is_file() {
        println!("2");
        return local;
    }

    let mut result = ConfigSum::new();
    result.priorities = global_config.priorities;
    // does crossbeam/tokio speed up multi io at all?
    let mut closest = (usize::MAX, PathBuf::new());
    for path in global_config.config_paths {
        let path = PathBuf::from(&path).canonicalize().unwrap();
        let container = path.parent().unwrap().to_path_buf();
        if current_dir.starts_with(&container) {
            let remainder = current_dir.strip_prefix(&container).unwrap();
            let count = remainder.components().count();
            if count < closest.0 {
                closest.0 = count;
                closest.1 = path;
            }
        }
    }

    if closest.0 == usize::MAX {
        return global_config.path.unwrap()
    }

    closest.1
}

fn load_local_config() -> ConfigFile {
    let local_config = find_most_local_config();
    let mut local = read_config(&local_config);
    local.path = Some(local_config);
    local
}

fn show(sub_matches: &ArgMatches) {
    println!("Showing...");

    let reminders;

    if sub_matches.get_one::<bool>("all") != None && *sub_matches.get_one::<bool>("all").unwrap()  {
        reminders = get_all();
    } else {
        reminders = get_local();
    }
    
    // let sorted_reminders = reminders.reminders.group_by(|a, b| { a.reminder.priority == b.reminder.priority });
    let sorted_reminders = reminders.reminders.group_by(|a, b| { a.path == b.path });
    
    for reminder_group in sorted_reminders {
        println!("{}", reminder_group[0].path.to_str().unwrap());
        for reminder in reminder_group {
            // println!("{}", reminder.reminder.text);
            
            if let Some(priority) = reminders.priorities.iter().find(|v| { v.name == reminder.reminder.priority }) {
                println!("{}", reminder.reminder.text.color(priority.color));
            } else {
                // white & log couldn't find?
            }
        }
    }

    /*
    println!("Showing {}", sub_matches.get_one::<String>("REMOTE").expect("required"));
             let add_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
             match add_command {
                 | ("apply", sub_matches) => {
                     let stash = sub_matches.get_one::<String>("STASH");
                     println!("Applying {:?}", stash);
                 },
                 | ("pop", sub_matches) => {
                     let stash = sub_matches.get_one::<String>("STASH");
                     println!("Popping {:?}", stash);
                 },
                 | ("push", sub_matches) => {
                     let message = sub_matches.get_one::<String>("message");
                     println!("Pushing {:?}", message);
                 },
                 | (name, _) => {
                     unreachable!("Unsupported subcommand `{}`", name)
                 },
             }
    */
}

fn find_priority(name: String) -> Priority {
    let global = load_global_config();
    if let Some(priority) = global.priorities.iter().find(|v| { v.name == name }) {
        return priority.clone();
    }

    panic!("Could not find an existing priority based on the name: {}", name); // TODO could fuzzy find here
}

fn add(sub_matches: &ArgMatches) {
    let subcommand = sub_matches.subcommand();
    let global: bool = *sub_matches.get_one("global").unwrap_or(&false);

    if subcommand != None && subcommand.unwrap().0 == "priority" {
        let text: String = subcommand.unwrap().1.get_one::<String>("priority").unwrap().clone();
        let mut config = load_global_config();
        let color: String = subcommand.unwrap().1.get_one::<String>("color").unwrap().clone(); // TODO color parsing

        // TODO color
        
        // TODO the id
        config.priorities.push(Priority { name: text, id: String::from("0"), color: reminder::Color::Cyan });
        let output = toml::to_string_pretty(&config).unwrap();
        fs::write(&config.path.unwrap(), &output);
    } else {
        if let Some(text) = sub_matches.get_one::<String>("reminder") {
            // let author: String = sub_matches.get_one::<String>("author").unwrap().clone(); // TODO: figure this out
            let author = "".to_string();
            let priority: String = sub_matches.get_one::<String>("priority").unwrap().clone(); // TODO: figure this out

            add_reminder(global, text.clone(), priority, author);
        } else {
            let sub_matches = sub_matches.subcommand_matches("reminder").unwrap();
            let text = sub_matches.get_one::<String>("reminder").unwrap();

            let author = "".to_string();
            let priority: String = sub_matches.get_one::<String>("priority").unwrap().clone(); // TODO: figure this out

            add_reminder(global, text.clone(), priority, author);
        }
    }
}

fn add_reminder(global: bool, text: String, priority: String, author: String) {
    let mut config;
    if global {
        config = load_global_config();
    } else {
        config = load_local_config();
    }

    let priority = find_priority(priority);

    config.reminders.push(Reminder { priority: priority.name, author, text });
    let output = toml::to_string_pretty(&config).unwrap();
    fs::write(&config.path.unwrap(), &output);
}

fn remind(sub_matches: &ArgMatches) {
    let subcommand = sub_matches.subcommand();
        let global: bool = *sub_matches.get_one("global").unwrap_or(&false);

        match subcommand {
            Some(("add", _)) | None => {
                let text: String = sub_matches.get_one::<String>("reminder").unwrap().clone();
                // let author: String = sub_matches.get_one::<String>("author").unwrap().clone(); // TODO: figure this out
                let author = "".to_string();
                // let priority: String = sub_matches.get_one::<String>("priority").unwrap().clone(); // TODO: figure this out
                let priority = "Critical".to_string();

                add_reminder(global, text, priority, author);
            },
            Some(("remove", sub_matches)) => {
                // find the specific reminder
                // use function so "remove()" can also use it
            },
            Some(("edit", sub_matches)) => {
                // find the specific reminder
            },
            Some((_, _)) => {
                panic!("Unknown subcommand.");
            }
        }

        let mut config;
        if global {
            config = load_global_config();
        } else {
            config = load_local_config();
        }
}

fn remove(sub_matches: &ArgMatches) {
    println!("Removing...");
}

fn prio(sub_matches: &ArgMatches) {
    println!("Priorities...");
}

fn init(sub_matches: &ArgMatches) {
    let mut global_config = load_global_config();

    let current_dir = current_dir().unwrap().canonicalize().unwrap(); // TODO no unwrap here

    let local = current_dir.join(CONFIG_NAME);
    println!("Local is {:?}", local);

    for path in &global_config.config_paths {
        let path = Path::new(&path).to_path_buf();
        if path == local {
            println!("Local config file already exists and is in global config.");
            return;
        }
    }

    if local.is_file() {
        loop {
            println!("Local config file found that is not in global config, would you like to add it? [y/n]");
            let input: char = read!();
            if input.to_lowercase().to_string() == "y" {
                global_config.config_paths.push(local.to_str().unwrap().to_string());

                // TODO this cannot merge reminders in that you didnt have, store reminders locally as well so it can?
                break;
            } else if input.to_lowercase().to_string() == "n" {
                println!("Nothing to do.");
                return;
            }
        }
    } else {
        let mut global = load_global_config();
        global.config_paths.push(local.to_str().unwrap().to_string());
        let output = toml::to_string_pretty(&global).unwrap();
        fs::write(&global.path.unwrap(), &output);

        println!("Writing to {:?}", local);
        let mut local_config = ConfigFile::default_local_config();
        let output = toml::to_string_pretty(&local_config).unwrap();
        fs::write(local, &output);
    }
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        | Some(("show", sub_matches)) => show(sub_matches),
        | Some(("add", sub_matches)) => add(sub_matches),
        | Some(("remind", sub_matches)) => remind(sub_matches),
        | Some(("remove", sub_matches)) => remove(sub_matches),
        | Some(("prio", sub_matches)) => prio(sub_matches),
        | Some(("init", sub_matches)) => init(sub_matches),
        | Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Unknown command {:?} with arguments {:?}", ext, args);
            cli().print_long_help().expect("Failed to print the help message to the console.");
        },
        | _ => unreachable!(), // If all subcommands are defined above, anything else is unreachabe!()
    }

    // Continued program logic goes here...
}

/* commands
 * show
 *    -a, --all
 *    -p, --priorities
 *    -q, --quotes
 *    -i, --show-ids
 *    --author (only show the reminders/quotes by a specific author) // TODO NEED TO ADD
 *       option to specify authors for quotes && reminders, how? qauthors, rauthors? regex options for all?
 * add
 *    quote
 *       --author (git username, or default username? for use in git projects)
 *    remind
 *       --author
 *    global, --global, -g (global, defaults to contextual)
 *    context // TODO NEED TO ADD
 * remind, r (alias for add remind)
 * remove
 *    --regex (or just accept regex anywhere)
 *    -i
 *    -p
 *    -q
 *    -c (remove context file, warn if contains data)
 * list (alias for show)
 * prio, priorities
 *    add
 *    edit
 *       -c, --color
 *       -i
 *    remove
 * init
 *    add rmnd file locally (check if already exists ie git clone, rsync and ask if overrie or keep)
 *    --no-file (can add a context to the global file if you dont want to have to add the file to your git ignore file, if you want the reminders synced, do not use this option)
 *
 */

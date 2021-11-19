extern crate clap;

use clap::{Arg, App, ArgMatches};
use std::fs;
use std::time::{Duration};
use std::path::Path;
use std::io::{stdin,stdout,Write};

struct Config {
    time: Duration,
    confirmation: bool
}

fn set_config(matches: ArgMatches ) -> Config {
    // Get default values
    let mut config  = Config {
        time: Duration::from_secs(60),
        confirmation: true
    };

    // Check if, and use when, time variable is set
    let secs = matches.value_of("seconds")
        .unwrap_or("60")
        .parse()
        .unwrap();
    config.time = Duration::from_secs(secs);

    // Check if 'no confirmation' flag is set
    if matches.is_present("no confirmation") {
        config.confirmation = false
    }

    return config
}

fn main()  {
    let matches = App::new("rewind")
        .version("0.1.1")
        .about("Rewinds your working directory to a point in time as far as newly added files go.")
        .author("debruinf")
        .arg(Arg::with_name("seconds")
             .short("t")
             .long("time")
             .help("Rewind to how long ago in seconds (defaults to 60 seconds)")
             .takes_value(true))
        .arg(Arg::with_name("no confirmation")
             .long("no-conf")
             .help("Doesn't ask for confirmation")
             .takes_value(false))
        .get_matches();

    let config = set_config(matches);
    println!("{:?} seconds will be used and a confirmation will be asked {:?}", config.time, config.confirmation);


    // let file_count = remove_stuff(Path::new("./"), time, force);

    // // if force == true update user. Otherwise, ask user to confirm for removal
    // if file_count > 0 {
    //     if force {
    //         println!("were removed\n")
    //     } else {
    //         let mut s=String::new();
    //         print!("\nwill be removed. Continue? (yes to continue): ");
    //         let _=stdout().flush();
    //         // stdin().read_line(&mut s).expect("Did not enter a correct string");
    //         if let Some('\n')=s.chars().next_back() {
    //             s.pop();
    //         }
    //         if let Some('\r')=s.chars().next_back() {
    //             s.pop();
    //         }
    //         if s == "yes" {
    //             force = true;
    //             remove_stuff(Path::new("./"), time, force);
    //         } else {
    //             println!("\nRewind aborted")
    //         }
    //     }
    // } else {
    //     println!("Nothing to rewind")
    // }
}

fn remove_stuff(path: &Path, time: Duration, force: bool) -> i64 {
    let mut iter: i64 = 0;
    for entry in fs::read_dir(path).unwrap() {
        let dir = entry.unwrap();
        match dir.metadata() {
            Ok(m) =>
                if !m.is_dir() {
                    match m.modified() {
                        Ok(mod_time) => {
                            if mod_time.elapsed().unwrap() < time {
                                iter += 1;
                                if force {
                                    println!("{}", dir.path().display());
                                    fs::remove_file(dir.path()).unwrap();
                                } else {
                                    println!("{}", dir.path().display());
                                }
                            }
                        }
                        Err(err) => println!("{:?}", err),
                    }
                }
            Err(err) => println!("{:?}", err),
        }
    }
    iter
}


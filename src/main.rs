extern crate clap;

use clap::{Arg, App};
use std::fs;
use std::time::{Duration};
use std::path::Path;
use std::io::{stdin,stdout,Write};

fn main()  {
    let matches = App::new("rewind")
        .version("0.1")
        .about("Rewinds your filesystem to how it was a minute ago. Asks for confirmation before removing files.")
        .author("debruinf")
        .arg(Arg::with_name("seconds")
             .short("t")
             .long("time")
             .help("Rewind to how long ago in seconds")
             .takes_value(true))
        .arg(Arg::with_name("force")
             .short("f")
             .long("force")
             .help("Doesn't ask for confirmation")
             .takes_value(false))
        .get_matches();



    // Check if, and use when, time variable is set
    let secs = matches.value_of("seconds")
        .unwrap_or("60")
        .parse()
        .unwrap();
    let time = Duration::from_secs(secs);

    // Check if 'force' flag is set
    let mut force = matches.is_present("force");

    let file_count = remove_stuff(Path::new("./"), time, force);

    // if force == true update user. Otherwise, ask user to confirm for removal
    if file_count > 0 {
        if force {
            println!("were removed\n")
        } else {
            let mut s=String::new();
            print!("will be removed. Continue? (yes to continue): ");
            let _=stdout().flush();
            stdin().read_line(&mut s).expect("Did not enter a correct string");
            if let Some('\n')=s.chars().next_back() {
                s.pop();
            }
            if let Some('\r')=s.chars().next_back() {
                s.pop();
            }
            if s == "yes" {
                force = true;
                remove_stuff(Path::new("./"), time, force);
            } else {
                println!("\nRewind aborted")
            }
        }
    } else {
        println!("Nothing to rewind")
    }
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


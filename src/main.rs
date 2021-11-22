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

struct Ffile {
    modified: Duration,
    path: Path
}

fn set_config(matches: ArgMatches ) -> Config {
    // Get default values
    let mut config  = Config {
        time: Duration::from_secs(60),
        confirmation: true
    };

    // Check if, and use when, time variable is set
    let secs = matches.value_of("seconds")
        .unwrap_or("60") // TODO already set, can we just unwrap?
        .parse()
        .unwrap();
    config.time = Duration::from_secs(secs);

    // Check if 'no confirmation' flag is set
    if matches.is_present("no confirmation") {
        config.confirmation = false
    }

    return config
}

fn collect_files_paths() {
    return 
}

fn main()  {
    let matches = App::new("rewind")
        .version("0.1.1")
        .about("Rewinds your working directory to a certain point in time (as far as newly added files go).")
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

    let to_be_deleted_files = collect_files_paths();

    print_file_times()
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

fn get_mod_time() -> Duration {
    Duration::from_secs(60)
}

fn print_file_times() {
    // Return a list of all files in the working directory filtered on modified_time
    let aaa = fs::read_dir("./").iter();
    println!("{:?}", &aaa)
    // aaa.filter(|x| )
    // for entry in fs::read_dir("./").unwrap() {
    //     let ffile = Ffile {
    //         path: entry.unwap(),
    //         modified: get_mod_time()
    //     };
    //     aa.push(ffile)
    // }
    //     let dir = entry.unwrap();
    //     match dir.metadata() {
    //         Ok(m) =>
    //             if !m.is_dir() {
    //                 match m.modified() {
    //                     Ok(mod_time) => {
    //                         return mod_time.elapsed()
    //                             // println!("{:?}", dir);
    //                             // println!("{:?}", mod_time.elapsed().unwrap());
    //                     }
    //                     Err(err) => println!("{:?}", err),
    //                 }
    //             }
    //         Err(err) => println!("{:?}", err),
    //     }
    // };

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


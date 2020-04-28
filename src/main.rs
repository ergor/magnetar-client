mod consts;
mod create_tables;
mod error;
mod fs_indexer;
mod index_once;
mod listener;
mod db_models;

use clap::{App, Arg, AppSettings};
use std::env;
use std::result;
use std::time::{SystemTime};
use std::process::exit;


pub type Result<T, E = crate::error::Error> = result::Result<T, E>;

fn main() -> crate::Result<()> {
    let args = App::new(consts::PROGRAM_NAME)
        .setting(AppSettings::TrailingVarArg)
        .version(clap::crate_version!())
        .about("filesystem indexer client")
        .arg(Arg::with_name("daemonize")
            .short("d")
            .long("daemonize")
            .help("Run the program in background")
            .takes_value(false))
        .arg(Arg::with_name("force")
            .short("f")
            .long("force")
            .help("Create a new index instead of diffing current")
            .takes_value(false))
        .arg(Arg::with_name("listen")
            .short("l")
            .long("listen")
            .help("Listen for filesystem changes instead of active indexing")
            .takes_value(false))
        .arg(Arg::with_name("directories")
            .required(true)
            .multiple(true))
        .get_matches();

    let directories = args.values_of("directories").unwrap();
    for dir in directories.clone() { // TODO: naive subdir check. doesn't guard against links
        for other_dir in directories.clone() {
            if dir == other_dir {
                continue;
            }
            if dir.starts_with(other_dir) {
                eprintln!("{} is subdirectory of {}. abort.", dir, other_dir);
                exit(consts::EXIT_INVALID_ARGS);
            }
        }
    }

    if args.is_present("daemonize") {
        unimplemented!()
    }

    let time_now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("somehow, time now is before start of UNIX epoch");

    let db_filename = format!("{}-{}.db", consts::PROGRAM_NAME, time_now.as_secs());

    let mut tmp_dir = env::temp_dir();
    tmp_dir.push(db_filename.as_str());
    let db_path = tmp_dir.to_str()
        .expect("could not create temporary database (illegal filename)");

    if args.is_present("listen") {
        listener::start();
    } else {
        index_once::start(db_path, directories, args.is_present("force"))?;
    }

    return Ok(());
}

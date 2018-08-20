#[macro_use] extern crate clap;

extern crate librojo;

use std::path::{Path, PathBuf};
use std::process;

use librojo::pathext::canonicalish;

fn main() {
    let matches = clap_app!(rojo =>
        (version: env!("CARGO_PKG_VERSION"))
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))

        (@subcommand init =>
            (about: "Creates a new Rojo project")
            (@arg PATH: "Path to the place to create the project. Defaults to the current directory.")
        )

        (@subcommand serve =>
            (about: "Serves the project's files for use with the Rojo Studio plugin.")
            (@arg PROJECT: "Path to the project to serve. Defaults to the current directory.")
            (@arg port: --port +takes_value "The port to listen on. Defaults to 8000.")
        )

    ).get_matches();

    match matches.subcommand() {
        ("init", sub_matches) => {
            let sub_matches = sub_matches.unwrap();
            let project_path = Path::new(sub_matches.value_of("PATH").unwrap_or("."));
            let full_path = canonicalish(project_path);

            librojo::commands::init(&full_path);
        },
        ("serve", sub_matches) => {
            let sub_matches = sub_matches.unwrap();

            let project_path = match sub_matches.value_of("PROJECT") {
                Some(v) => canonicalish(PathBuf::from(v)),
                None => std::env::current_dir().unwrap(),
            };

            librojo::commands::serve(&project_path);
        },
        _ => {
            eprintln!("Please specify a subcommand!");
            eprintln!("Try 'rojo help' for information.");
            process::exit(1);
        },
    }
}
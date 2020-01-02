use std::{error::Error, panic, process};

use log::error;
use structopt::StructOpt;

use librojo::cli::{self, Options, Subcommand};

fn main() {
    let options = Options::from_args();

    {
        let log_filter = match options.verbosity {
            0 => "warn",
            1 => "warn,librojo=info",
            2 => "warn,librojo=trace",
            _ => "trace",
        };

        let log_env = env_logger::Env::default().default_filter_or(log_filter);

        env_logger::Builder::from_env(log_env)
            .format_timestamp(None)
            .init();
    }

    let panic_result = panic::catch_unwind(|| {
        if let Err(err) = run(options.subcommand) {
            log::error!("{}", err);
            process::exit(1);
        }
    });

    if let Err(error) = panic_result {
        let message = match error.downcast_ref::<&str>() {
            Some(message) => message.to_string(),
            None => match error.downcast_ref::<String>() {
                Some(message) => message.clone(),
                None => "<no message>".to_string(),
            },
        };

        show_crash_message(&message);
        process::exit(1);
    }
}

fn run(subcommand: Subcommand) -> Result<(), Box<dyn Error>> {
    match subcommand {
        Subcommand::Init(init_options) => cli::init(init_options)?,
        Subcommand::Serve(serve_options) => cli::serve(serve_options)?,
        Subcommand::Build(build_options) => cli::build(build_options)?,
        Subcommand::Upload(upload_options) => cli::upload(upload_options)?,
    }

    Ok(())
}

fn show_crash_message(message: &str) {
    error!("Rojo crashed!");
    error!("This is a bug in Rojo.");
    error!("");
    error!("Please consider filing a bug: https://github.com/rojo-rbx/rojo/issues");
    error!("");
    error!("Details: {}", message);
}

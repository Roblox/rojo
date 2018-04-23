use std::collections::HashMap;
use std::path::PathBuf;
use std::process;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use rand;

use project::Project;
use web;
use session_config::SessionConfig;
use fs_session::FsSession;
use partition::Partition;

pub fn serve(project_path: &PathBuf, port: Option<u64>) {
    let server_id = rand::random::<u64>();

    let project = match Project::load(project_path) {
        Ok(v) => {
            println!("Using project from {}", project_path.display());
            v
        },
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        },
    };

    // let web_config = web::WebConfig {
    //     port: port.unwrap_or(project.serve_port),
    //     server_id,
    //     project: project.clone(),
    //     start_time: Instant::now(),
    // };

    // println!("Server listening on port {}", web_config.port);

    // web::start(web_config);

    let mut partitions = HashMap::new();

    for (partition_name, partition) in project.partitions.iter() {
        let path = project_path.join(&partition.path);
        let target = partition.target.split(".").map(String::from).collect::<Vec<_>>();

        partitions.insert(partition_name.clone(), Partition {
            path,
            target,
        });
    }

    let config = SessionConfig {
        partitions
    };

    println!("Using session config {:#?}", config);

    let session = {
        let mut session = FsSession::new(config.clone());
        session.init();

        Arc::new(Mutex::new(session))
    };

    // TODO: Let FsSession handle the main loop and spawn a bunch of threads
    // ...since Rust stable doesn't have a channel select implementation
    thread::spawn(move || {
        loop {
            session.lock().unwrap().step();
        }
    });

    loop {
        thread::park();
    }
}
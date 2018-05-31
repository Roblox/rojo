use std::collections::HashMap;
use std::sync::{mpsc, RwLock, Arc};

use rouille::{self, Response};

use web_util::read_json;
use id::Id;
use rbx::RbxInstance;
use rbx_session::RbxSession;
use message_session::{MessageSession, Message};
use partition::Partition;

/// The set of configuration the web server needs to start.
pub struct WebConfig {
    pub port: u64,
    pub server_id: u64,
    pub rbx_session: Arc<RwLock<RbxSession>>,
    pub message_session: MessageSession,
    pub partitions: HashMap<String, Partition>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ServerInfoResponse<'a> {
    server_id: &'a str,
    server_version: &'static str,
    protocol_version: u64,
    partitions: &'a HashMap<String, &'a [String]>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReadAllResponse<'a> {
    server_id: &'a str,
    message_cursor: i32,
    instances: &'a HashMap<Id, RbxInstance>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct ReadResponse<'a> {
    server_id: &'a str,
    message_cursor: i32,
    instances: HashMap<Id, &'a RbxInstance>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SubscribeResponse<'a> {
    server_id: &'a str,
    message_cursor: i32,
    messages: &'a [Message],
}

/// Start the Rojo web server and park our current thread.
#[allow(unreachable_code)]
pub fn start(config: WebConfig) {
    let address = format!("localhost:{}", config.port);
    let server_version = env!("CARGO_PKG_VERSION");

    let server_id = config.server_id.to_string();

    rouille::start_server(address, move |request| {
        router!(request,
            (GET) (/) => {
                // Get a summary of information about the server.

                let mut partitions = HashMap::new();

                for partition in config.partitions.values() {
                    partitions.insert(partition.name.clone(), partition.target.as_slice());
                }

                Response::json(&ServerInfoResponse {
                    server_version,
                    protocol_version: 2,
                    server_id: &server_id,
                    partitions: &partitions,
                })
            },

            (GET) (/subscribe/{ cursor: i32 }) => {
                // Retrieve any messages past the given cursor index, and if
                // there weren't any, subscribe to receive any new messages.

                // Did the client miss any messages since the last subscribe?
                {
                    let messages = config.message_session.messages.read().unwrap();

                    if cursor > messages.len() as i32 {
                        return Response::json(&SubscribeResponse {
                            server_id: &server_id,
                            messages: &[],
                            message_cursor: messages.len() as i32 - 1,
                        });
                    }

                    if cursor < messages.len() as i32 - 1 {
                        let new_messages = &messages[(cursor + 1) as usize..];
                        let new_cursor = cursor + new_messages.len() as i32;

                        return Response::json(&SubscribeResponse {
                            server_id: &server_id,
                            messages: new_messages,
                            message_cursor: new_cursor,
                        });
                    }
                }

                let (tx, rx) = mpsc::channel();

                let sender_id = config.message_session.subscribe(tx);

                match rx.recv() {
                    Ok(_) => (),
                    Err(_) => return Response::text("error!").with_status_code(500),
                }

                config.message_session.unsubscribe(sender_id);

                {
                    let messages = config.message_session.messages.read().unwrap();
                    let new_messages = &messages[(cursor + 1) as usize..];
                    let new_cursor = cursor + new_messages.len() as i32;

                    Response::json(&SubscribeResponse {
                        server_id: &server_id,
                        messages: new_messages,
                        message_cursor: new_cursor,
                    })
                }
            },

            (GET) (/read_all) => {
                let rbx_session = config.rbx_session.read().unwrap();

                let message_cursor = {
                    let messages = config.message_session.messages.read().unwrap();
                    messages.len() as i32 - 1
                };

                Response::json(&ReadAllResponse {
                    server_id: &server_id,
                    message_cursor,
                    instances: &rbx_session.instances,
                })
            },

            (POST) (/read) => {
                let requested_ids = match read_json::<Vec<Id>>(request) {
                    Some(body) => body,
                    None => return rouille::Response::text("Malformed JSON").with_status_code(400),
                };

                let rbx_session = config.rbx_session.read().unwrap();

                let message_cursor = {
                    let messages = config.message_session.messages.read().unwrap();
                    messages.len() as i32 - 1
                };

                let mut instances = HashMap::new();

                for requested_id in &requested_ids {
                    let requested_instance = match rbx_session.instances.get(requested_id) {
                        Some(instance) => instance,
                        None => continue,
                    };

                    instances.insert(*requested_id, requested_instance);

                    // Oops; this needs to be recursive.
                    for (child_id, instance) in &rbx_session.instances {
                        if instance.parent == Some(*requested_id) {
                            instances.insert(*child_id, instance);
                        }
                    }
                }

                Response::json(&ReadResponse {
                    server_id: &server_id,
                    message_cursor,
                    instances,
                })
            },

            _ => Response::empty_404()
        )
    });
}

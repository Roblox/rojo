use std::{
    borrow::Cow,
    collections::HashMap,
    sync::{mpsc, Arc},
};

use rouille::{
    self,
    router,
    Request,
    Response,
};
use rbx_tree::{RbxId, RootedRbxInstance};

use crate::{
    message_queue::Message,
    session::Session,
    session_id::SessionId,
    project::Project,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfoResponse<'a> {
    pub session_id: SessionId,
    pub server_version: &'a str,
    pub protocol_version: u64,
    pub root_instance_id: RbxId,
    pub project: Cow<'a, Project>,
    pub project_paths_to_ids: Cow<'a, HashMap<RbxId, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadResponse<'a> {
    pub session_id: SessionId,
    pub message_cursor: u32,
    pub instances: HashMap<RbxId, Cow<'a, RootedRbxInstance>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscribeResponse<'a> {
    pub session_id: SessionId,
    pub message_cursor: u32,
    pub messages: Cow<'a, [Message]>,
}

pub struct Server {
    session: Arc<Session>,
    server_version: &'static str,
}

impl Server {
    pub fn new(session: Arc<Session>) -> Server {
        Server {
            session,
            server_version: env!("CARGO_PKG_VERSION"),
        }
    }

    #[allow(unreachable_code)]
    pub fn handle_request(&self, request: &Request) -> Response {
        trace!("Request {} {}", request.method(), request.url());

        router!(request,
            (GET) (/) => {
                Response::text("Rojo is up and running!")
            },

            (GET) (/api/rojo) => {
                // Get a summary of information about the server.

                let rbx_session = self.session.rbx_session.lock().unwrap();
                let tree = rbx_session.get_tree();

                Response::json(&ServerInfoResponse {
                    server_version: self.server_version,
                    protocol_version: 2,
                    session_id: self.session.session_id,
                    root_instance_id: tree.get_root_id(),
                    project: Cow::Borrowed(&self.session.project),
                    project_paths_to_ids: Cow::Borrowed(rbx_session.get_project_path_map()),
                })
            },

            (GET) (/api/subscribe/{ cursor: u32 }) => {
                // Retrieve any messages past the given cursor index, and if
                // there weren't any, subscribe to receive any new messages.

                let message_queue = Arc::clone(&self.session.message_queue);

                // Did the client miss any messages since the last subscribe?
                {
                    let (new_cursor, new_messages) = message_queue.get_messages_since(cursor);

                    if !new_messages.is_empty() {
                        return Response::json(&SubscribeResponse {
                            session_id: self.session.session_id,
                            messages: Cow::Borrowed(&[]),
                            message_cursor: new_cursor,
                        })
                    }
                }

                let (tx, rx) = mpsc::channel();

                let sender_id = message_queue.subscribe(tx);

                match rx.recv() {
                    Ok(_) => (),
                    Err(_) => return Response::text("error!").with_status_code(500),
                }

                message_queue.unsubscribe(sender_id);

                {
                    let (new_cursor, new_messages) = message_queue.get_messages_since(cursor);

                    return Response::json(&SubscribeResponse {
                        session_id: self.session.session_id,
                        messages: Cow::Owned(new_messages),
                        message_cursor: new_cursor,
                    })
                }
            },

            (GET) (/api/read/{ id_list: String }) => {
                let message_queue = Arc::clone(&self.session.message_queue);

                let requested_ids: Option<Vec<RbxId>> = id_list
                    .split(',')
                    .map(RbxId::parse_str)
                    .collect();

                let requested_ids = match requested_ids {
                    Some(id) => id,
                    None => return rouille::Response::text("Malformed ID list").with_status_code(400),
                };

                let rbx_session = self.session.rbx_session.lock().unwrap();
                let tree = rbx_session.get_tree();

                let message_cursor = message_queue.get_message_cursor();

                let mut instances = HashMap::new();

                for &requested_id in &requested_ids {
                    if let Some(instance) = tree.get_instance(requested_id) {
                        instances.insert(instance.get_id(), Cow::Borrowed(instance));

                        for descendant in tree.descendants(requested_id) {
                            instances.insert(descendant.get_id(), Cow::Borrowed(descendant));
                        }
                    }
                }

                Response::json(&ReadResponse {
                    session_id: self.session.session_id,
                    message_cursor,
                    instances,
                })
            },

            _ => Response::empty_404()
        )
    }

    pub fn listen(self, port: u64) {
        let address = format!("localhost:{}", port);

        rouille::start_server(address, move |request| self.handle_request(request));
    }
}
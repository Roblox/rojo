use std::{
    borrow::Cow,
    str,
};

use maplit::hashmap;
use rbx_dom_weak::{RbxTree, RbxValue, RbxId};

use crate::{
    imfs::new::{Imfs, ImfsFetcher, ImfsEntry},
    snapshot::InstanceSnapshot,
};

use super::{
    middleware::{SnapshotMiddleware, SnapshotInstanceResult, SnapshotFileResult},
};

pub struct SnapshotLua;

impl SnapshotMiddleware for SnapshotLua {
    fn from_imfs<F: ImfsFetcher>(
        imfs: &mut Imfs<F>,
        entry: &ImfsEntry,
    ) -> SnapshotInstanceResult<'static> {
        if entry.is_directory() {
            return Ok(None);
        }

        let file_name = entry.path()
            .file_name().unwrap().to_string_lossy();

        let (class_name, instance_name) = if let Some(name) = match_trailing(&file_name, ".server.lua") {
            ("Script", name)
        } else if let Some(name) = match_trailing(&file_name, ".client.lua") {
            ("LocalScript", name)
        } else if let Some(name) = match_trailing(&file_name, ".lua") {
            ("ModuleScript", name)
        } else {
            return Ok(None);
        };

        let contents = entry.contents(imfs)?;
        let contents_str = str::from_utf8(contents)
            .expect("File content was not valid UTF-8")
            .to_string();

        let properties = hashmap! {
            "Source".to_owned() => RbxValue::String {
                value: contents_str,
            },
        };

        Ok(Some(InstanceSnapshot {
            snapshot_id: None,
            name: Cow::Owned(instance_name.to_owned()),
            class_name: Cow::Borrowed(class_name),
            properties,
            children: Vec::new(),
        }))
    }

    fn from_instance(
        _tree: &RbxTree,
        _id: RbxId,
    ) -> SnapshotFileResult {
        unimplemented!("Snapshotting Script instances");
    }
}

fn match_trailing<'a>(input: &'a str, trailer: &str) -> Option<&'a str> {
    if input.ends_with(trailer) {
        let end = input.len().saturating_sub(trailer.len());
        Some(&input[..end])
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use maplit::hashmap;

    use crate::imfs::new::{ImfsSnapshot, NoopFetcher};

    #[test]
    fn module_from_imfs() {
        let mut imfs = Imfs::new(NoopFetcher);
        let file = ImfsSnapshot::file("Hello there!");

        imfs.load_from_snapshot("/foo.lua", file);

        let entry = imfs.get("/foo.lua").unwrap();
        let instance_snapshot = SnapshotLua::from_imfs(&mut imfs, &entry).unwrap().unwrap();

        assert_eq!(instance_snapshot.name, "foo");
        assert_eq!(instance_snapshot.class_name, "ModuleScript");
        assert_eq!(instance_snapshot.properties, hashmap! {
            "Source".to_owned() => RbxValue::String {
                value: "Hello there!".to_owned(),
            },
        });
    }

    #[test]
    fn server_from_imfs() {
        let mut imfs = Imfs::new(NoopFetcher);
        let file = ImfsSnapshot::file("Hello there!");

        imfs.load_from_snapshot("/foo.server.lua", file);

        let entry = imfs.get("/foo.server.lua").unwrap();
        let instance_snapshot = SnapshotLua::from_imfs(&mut imfs, &entry).unwrap().unwrap();

        assert_eq!(instance_snapshot.name, "foo");
        assert_eq!(instance_snapshot.class_name, "Script");
        assert_eq!(instance_snapshot.properties, hashmap! {
            "Source".to_owned() => RbxValue::String {
                value: "Hello there!".to_owned(),
            },
        });
    }

    #[test]
    fn client_from_imfs() {
        let mut imfs = Imfs::new(NoopFetcher);
        let file = ImfsSnapshot::file("Hello there!");

        imfs.load_from_snapshot("/foo.client.lua", file);

        let entry = imfs.get("/foo.client.lua").unwrap();
        let instance_snapshot = SnapshotLua::from_imfs(&mut imfs, &entry).unwrap().unwrap();

        assert_eq!(instance_snapshot.name, "foo");
        assert_eq!(instance_snapshot.class_name, "LocalScript");
        assert_eq!(instance_snapshot.properties, hashmap! {
            "Source".to_owned() => RbxValue::String {
                value: "Hello there!".to_owned(),
            },
        });
    }
}
use std::borrow::Cow;

use rbx_dom_weak::{RbxTree, RbxId};

use crate::{
    imfs::new::{Imfs, ImfsFetcher, ImfsEntry},
    snapshot::InstanceSnapshot,
};

use super::{
    middleware::{SnapshotMiddleware, SnapshotInstanceResult, SnapshotFileResult},
};

pub struct SnapshotRbxmx;

impl SnapshotMiddleware for SnapshotRbxmx {
    fn from_imfs<F: ImfsFetcher>(
        imfs: &mut Imfs<F>,
        entry: &ImfsEntry,
    ) -> SnapshotInstanceResult<'static> {
        if entry.is_directory() {
            return Ok(None);
        }

        let file_name = entry.path()
            .file_name().unwrap().to_string_lossy();

        if !file_name.ends_with(".rbxmx") {
            return  Ok(None);
        }

        let instance_name = entry.path()
            .file_stem().expect("Could not extract file stem")
            .to_string_lossy().to_string();

        let options = rbx_xml::DecodeOptions::new()
            .property_behavior(rbx_xml::DecodePropertyBehavior::ReadUnknown);

        let temp_tree = rbx_xml::from_reader(entry.contents(imfs)?, options)
            .expect("TODO: Handle rbx_xml errors");

        let root_instance = temp_tree.get_instance(temp_tree.get_root_id()).unwrap();
        let children = root_instance.get_children_ids();

        if children.len() == 1 {
            let mut snapshot = InstanceSnapshot::from_tree(&temp_tree, children[0]);
            snapshot.name = Cow::Owned(instance_name);

            Ok(Some(snapshot))
        } else {
            panic!("Rojo doesn't have support for model files with zero or more than one top-level instances yet.");
        }
    }

    fn from_instance(
        _tree: &RbxTree,
        _id: RbxId,
    ) -> SnapshotFileResult {
        unimplemented!("Snapshotting models");
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use rbx_dom_weak::RbxValue;
    use maplit::hashmap;

    use crate::imfs::new::{ImfsSnapshot, NoopFetcher};

    // #[test]
    fn model_from_imfs() {
        let mut imfs = Imfs::new(NoopFetcher);
        let file = ImfsSnapshot::file("Hello there!");

        imfs.load_from_snapshot("/foo.lua", file);

        let entry = imfs.get("/foo.lua").unwrap();
        let instance_snapshot = SnapshotRbxmx::from_imfs(&mut imfs, &entry).unwrap().unwrap();

        assert_eq!(instance_snapshot.name, "foo");
        assert_eq!(instance_snapshot.class_name, "ModuleScript");
        assert_eq!(instance_snapshot.properties, hashmap! {
            "Source".to_owned() => RbxValue::String {
                value: "Hello there!".to_owned(),
            },
        });
    }
}
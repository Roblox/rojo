use std::{borrow::Cow, collections::HashMap, path::Path};

use memofs::Vfs;
use rbx_dom_weak::types::Variant;
use serde::Deserialize;

use crate::snapshot::{InstanceContext, InstanceSnapshot};

use super::{error::SnapshotError, middleware::SnapshotInstanceResult};

pub fn snapshot_json_model(
    context: &InstanceContext,
    vfs: &Vfs,
    path: &Path,
    instance_name: &str,
) -> SnapshotInstanceResult {
    let contents = vfs.read(path)?;
    let instance: JsonModel = serde_json::from_slice(&contents)
        .map_err(|source| SnapshotError::malformed_model_json(source, path))?;

    let mut snapshot = instance.core.into_snapshot(instance_name.to_owned());

    snapshot.metadata = snapshot
        .metadata
        .instigating_source(path)
        .relevant_paths(vec![path.to_path_buf()])
        .context(context);

    Ok(Some(snapshot))
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct JsonModel {
    name: Option<String>,

    #[serde(flatten)]
    core: JsonModelCore,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct JsonModelInstance {
    name: String,

    #[serde(flatten)]
    core: JsonModelCore,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct JsonModelCore {
    class_name: String,

    #[serde(default = "Vec::new", skip_serializing_if = "Vec::is_empty")]
    children: Vec<JsonModelInstance>,

    // FIXME: Use unresolved value type here
    #[serde(default = "HashMap::new", skip_serializing_if = "HashMap::is_empty")]
    properties: HashMap<String, Variant>,
}

impl JsonModelCore {
    fn into_snapshot(self, name: String) -> InstanceSnapshot {
        let class_name = self.class_name;

        let children = self
            .children
            .into_iter()
            .map(|child| child.core.into_snapshot(child.name))
            .collect();

        InstanceSnapshot {
            snapshot_id: None,
            metadata: Default::default(),
            name: Cow::Owned(name),
            class_name: Cow::Owned(class_name),
            properties: self.properties,
            children,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use memofs::{InMemoryFs, VfsSnapshot};

    #[test]
    fn model_from_vfs() {
        let mut imfs = InMemoryFs::new();
        imfs.load_snapshot(
            "/foo.model.json",
            VfsSnapshot::file(
                r#"
                    {
                      "Name": "children",
                      "ClassName": "IntValue",
                      "Properties": {
                        "Value": 5
                      },
                      "Children": [
                        {
                          "Name": "The Child",
                          "ClassName": "StringValue"
                        }
                      ]
                    }
                "#,
            ),
        )
        .unwrap();

        let mut vfs = Vfs::new(imfs);

        let instance_snapshot = snapshot_json_model(
            &InstanceContext::default(),
            &mut vfs,
            Path::new("/foo.model.json"),
            "foo",
        )
        .unwrap()
        .unwrap();

        insta::assert_yaml_snapshot!(instance_snapshot);
    }
}

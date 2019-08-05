//! Defines the structure of an instance snapshot.

use std::{
    borrow::Cow,
    collections::HashMap,
};

use rbx_dom_weak::{RbxId, RbxValue};

/// A lightweight description of what an instance should look like. Attempts to
/// be somewhat memory efficient by borrowing from its source data, indicated by
/// the lifetime parameter, `'source`.
///
// Possible future improvements:
// - Use refcounted/interned strings
// - Replace use of RbxValue with a sum of RbxValue + borrowed value
#[derive(Debug, Clone, PartialEq)]
pub struct InstanceSnapshot<'source> {
    pub snapshot_id: Option<RbxId>,

    pub name: Cow<'source, str>,
    pub class_name: Cow<'source, str>,
    pub properties: HashMap<String, RbxValue>,
    pub children: Vec<InstanceSnapshot<'source>>,

    // TODO: Snapshot source, like a file or a project node?
}

impl<'source> InstanceSnapshot<'source> {
    pub fn get_owned(&'source self) -> InstanceSnapshot<'static> {
        let children: Vec<InstanceSnapshot<'static>> = self.children.iter()
            .map(InstanceSnapshot::get_owned)
            .collect();

        InstanceSnapshot {
            snapshot_id: None,
            name: Cow::Owned(self.name.clone().into_owned()),
            class_name: Cow::Owned(self.class_name.clone().into_owned()),
            properties: self.properties.clone(),
            children,
        }
    }
}
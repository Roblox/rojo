//! Defines the semantics that Rojo uses to turn entries on the filesystem into
//! Roblox instances using the instance snapshot subsystem.

#![allow(dead_code)]

mod context;
mod csv;
mod dir;
mod error;
mod json_model;
mod lua;
mod meta_file;
mod middleware;
mod project;
mod rbxlx;
mod rbxm;
mod rbxmx;
mod txt;
mod user_plugins;
mod util;

pub use self::error::*;

use std::path::PathBuf;

use rbx_dom_weak::{RbxId, RbxTree};

use self::{
    context::{InstanceSnapshotContext, SnapshotPluginContext},
    csv::SnapshotCsv,
    dir::SnapshotDir,
    json_model::SnapshotJsonModel,
    lua::SnapshotLua,
    middleware::{SnapshotFileResult, SnapshotInstanceResult, SnapshotMiddleware},
    project::SnapshotProject,
    rbxlx::SnapshotRbxlx,
    rbxm::SnapshotRbxm,
    rbxmx::SnapshotRbxmx,
    txt::SnapshotTxt,
    user_plugins::SnapshotUserPlugins,
};
use crate::imfs::{Imfs, ImfsEntry, ImfsFetcher};

macro_rules! middlewares {
    ( $($middleware: ident,)* ) => {
        /// Generates a snapshot of instances from the given ImfsEntry.
        pub fn snapshot_from_imfs<F: ImfsFetcher>(
            imfs: &mut Imfs<F>,
            entry: &ImfsEntry,
        ) -> SnapshotInstanceResult<'static> {
            // TODO: Accept this context as an argument instead so that it can
            // be derived from the current project.
            let mut context = InstanceSnapshotContext {
                plugin_context: Some(SnapshotPluginContext::new(vec![
                    PathBuf::from("test-projects/plugins/test-plugin.lua"),
                ])),
            };

            $(
                log::trace!("trying middleware {} on {}", stringify!($middleware), entry.path().display());

                if let Some(snapshot) = $middleware::from_imfs(&mut context, imfs, entry)? {
                    log::trace!("middleware {} success on {}", stringify!($middleware), entry.path().display());
                    return Ok(Some(snapshot));
                }
            )*

            log::trace!("no middleware returned Ok(Some)");
            Ok(None)
        }

        /// Generates an in-memory filesystem snapshot of the given Roblox
        /// instance.
        pub fn snapshot_from_instance(tree: &RbxTree, id: RbxId) -> SnapshotFileResult {
            $(
                if let Some(result) = $middleware::from_instance(tree, id) {
                    return Some(result);
                }
            )*

            None
        }
    };
}

middlewares! {
    SnapshotProject,
    SnapshotUserPlugins,
    SnapshotJsonModel,
    SnapshotRbxlx,
    SnapshotRbxmx,
    SnapshotRbxm,
    SnapshotLua,
    SnapshotCsv,
    SnapshotTxt,
    SnapshotDir,
}

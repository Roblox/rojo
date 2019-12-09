use std::{
    fs::File,
    io::{self, BufWriter, Write},
    path::PathBuf,
};

use failure::Fail;

use crate::{
    common_setup,
    project::ProjectError,
    vfs::{FsError, RealFetcher, Vfs, WatchMode},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputKind {
    Rbxmx,
    Rbxlx,
    Rbxm,
    Rbxl,
}

fn detect_output_kind(options: &BuildOptions) -> Option<OutputKind> {
    let extension = options.output_file.extension()?.to_str()?;

    match extension {
        "rbxlx" => Some(OutputKind::Rbxlx),
        "rbxmx" => Some(OutputKind::Rbxmx),
        "rbxl" => Some(OutputKind::Rbxl),
        "rbxm" => Some(OutputKind::Rbxm),
        _ => None,
    }
}

#[derive(Debug)]
pub struct BuildOptions {
    pub fuzzy_project_path: PathBuf,
    pub output_file: PathBuf,
    pub output_kind: Option<OutputKind>,
}

#[derive(Debug, Fail)]
pub enum BuildError {
    #[fail(display = "Could not detect what kind of file to create")]
    UnknownOutputKind,

    #[fail(display = "IO error: {}", _0)]
    IoError(#[fail(cause)] io::Error),

    #[fail(display = "XML model error: {}", _0)]
    XmlModelEncodeError(#[fail(cause)] rbx_xml::EncodeError),

    #[fail(display = "Binary model error: {:?}", _0)]
    BinaryModelEncodeError(rbx_binary::EncodeError),

    #[fail(display = "{}", _0)]
    ProjectError(#[fail(cause)] ProjectError),

    #[fail(display = "{}", _0)]
    FsError(#[fail(cause)] FsError),
}

impl_from!(BuildError {
    io::Error => IoError,
    rbx_xml::EncodeError => XmlModelEncodeError,
    rbx_binary::EncodeError => BinaryModelEncodeError,
    ProjectError => ProjectError,
    FsError => FsError,
});

fn xml_encode_config() -> rbx_xml::EncodeOptions {
    rbx_xml::EncodeOptions::new().property_behavior(rbx_xml::EncodePropertyBehavior::WriteUnknown)
}

pub fn build(options: &BuildOptions) -> Result<(), BuildError> {
    let output_kind = options
        .output_kind
        .or_else(|| detect_output_kind(options))
        .ok_or(BuildError::UnknownOutputKind)?;

    log::debug!("Hoping to generate file of type {:?}", output_kind);

    log::trace!("Constructing in-memory filesystem");
    let vfs = Vfs::new(RealFetcher::new(WatchMode::Disabled));

    let (_maybe_project, tree) = common_setup::start(&options.fuzzy_project_path, &vfs);
    let root_id = tree.get_root_id();

    log::trace!("Opening output file for write");
    let mut file = BufWriter::new(File::create(&options.output_file)?);

    match output_kind {
        OutputKind::Rbxmx => {
            // Model files include the root instance of the tree and all its
            // descendants.

            rbx_xml::to_writer(&mut file, tree.inner(), &[root_id], xml_encode_config())?;
        }
        OutputKind::Rbxlx => {
            // Place files don't contain an entry for the DataModel, but our
            // RbxTree representation does.

            let root_instance = tree.get_instance(root_id).unwrap();
            let top_level_ids = root_instance.children();

            rbx_xml::to_writer(&mut file, tree.inner(), top_level_ids, xml_encode_config())?;
        }
        OutputKind::Rbxm => {
            rbx_binary::encode(tree.inner(), &[root_id], &mut file)?;
        }
        OutputKind::Rbxl => {
            log::warn!("Support for building binary places (rbxl) is still experimental.");
            log::warn!("Using the XML place format (rbxlx) is recommended instead.");
            log::warn!("For more info, see https://github.com/LPGhatguy/rojo/issues/180");

            let root_instance = tree.get_instance(root_id).unwrap();
            let top_level_ids = root_instance.children();

            rbx_binary::encode(tree.inner(), top_level_ids, &mut file)?;
        }
    }

    file.flush()?;

    log::trace!("Done!");

    Ok(())
}

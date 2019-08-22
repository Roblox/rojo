//! Implements the IMFS fetcher interface for the real filesystem using Rust's
//! std::fs interface and notify as the file watcher.

use std::{
    fs,
    io,
    path::{Path, PathBuf},
    sync::mpsc,
    time::Duration,
};

use jod_thread::JoinHandle;
use crossbeam_channel::{Receiver, unbounded};
use notify::{RecursiveMode, RecommendedWatcher, Watcher};

use super::fetcher::{ImfsFetcher, FileType, ImfsEvent};

/// Workaround to disable the file watcher for processes that don't need it,
/// since notify appears hang on to mpsc Sender objects too long, causing Rojo
/// to deadlock on drop.
///
/// We can make constructing the watcher optional in order to hotfix rojo build.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchMode {
    Enabled,
    Disabled,
}

pub struct RealFetcher {
    // Drop order is relevant here!
    //
    // `watcher` must be dropped before `_converter_thread` or else joining the
    // thread will cause a deadlock.
    watcher: Option<RecommendedWatcher>,

    /// Thread handle to convert notify's mpsc channel messages into
    /// crossbeam_channel messages.
    _converter_thread: JoinHandle<()>,
    receiver: Receiver<ImfsEvent>,
}

impl RealFetcher {
    pub fn new(watch_mode: WatchMode) -> RealFetcher {
        log::trace!("Starting RealFetcher with watch mode {:?}", watch_mode);

        let (notify_sender, notify_receiver) = mpsc::channel();
        let (sender, receiver) = unbounded();

        let handle = jod_thread::Builder::new()
            .name("notify message converter".to_owned())
            .spawn(move || {
                notify_receiver
                    .into_iter()
                    .for_each(|event| { sender.send(event).unwrap() });
            })
            .expect("Could not start message converter thread");

        // TODO: Investigate why notify hangs onto notify_sender too long,
        // causing our program to deadlock. Once this is fixed, watcher no
        // longer needs to be optional, but is still maybe useful?
        let watcher = match watch_mode {
            WatchMode::Enabled => {
                Some(notify::watcher(notify_sender, Duration::from_millis(300))
                    .expect("Couldn't start 'notify' file watcher"))
            }
            WatchMode::Disabled => None,
        };

        RealFetcher {
            watcher,
            _converter_thread: handle,
            receiver,
        }
    }
}

impl ImfsFetcher for RealFetcher {
    fn file_type(&mut self, path: &Path) -> io::Result<FileType> {
        let metadata = fs::metadata(path)?;

        if metadata.is_file() {
            Ok(FileType::File)
        } else {
            Ok(FileType::Directory)
        }
    }

    fn read_children(&mut self, path: &Path) -> io::Result<Vec<PathBuf>> {
        log::trace!("Reading directory {}", path.display());

        let mut result = Vec::new();

        let iter = fs::read_dir(path)?;

        for entry in iter {
            result.push(entry?.path());
        }

        Ok(result)
    }

    fn read_contents(&mut self, path: &Path) -> io::Result<Vec<u8>> {
        log::trace!("Reading file {}", path.display());

        fs::read(path)
    }

    fn create_directory(&mut self, path: &Path) -> io::Result<()> {
        log::trace!("Creating directory {}", path.display());

        fs::create_dir(path)
    }

    fn write_file(&mut self, path: &Path, contents: &[u8]) -> io::Result<()> {
        log::trace!("Writing path {}", path.display());

        fs::write(path, contents)
    }

    fn remove(&mut self, path: &Path) -> io::Result<()> {
        log::trace!("Removing path {}", path.display());

        let metadata = fs::metadata(path)?;

        if metadata.is_file() {
            fs::remove_file(path)
        } else {
            fs::remove_dir_all(path)
        }
    }

    fn watch(&mut self, path: &Path) {
        log::trace!("Watching path {}", path.display());

        if let Some(watcher) = self.watcher.as_mut() {
            if let Err(err) = watcher.watch(path, RecursiveMode::NonRecursive) {
                log::warn!("Couldn't watch path {}: {:?}", path.display(), err);
            }
        }
    }

    fn unwatch(&mut self, path: &Path) {
        log::trace!("Stopped watching path {}", path.display());

        if let Some(watcher) = self.watcher.as_mut() {
            if let Err(err) = watcher.unwatch(path) {
                log::warn!("Couldn't unwatch path {}: {:?}", path.display(), err);
            }
        }
    }

    fn receiver(&self) -> Receiver<ImfsEvent> {
        self.receiver.clone()
    }
}
//! Local operator interface. Never listens on the player TCP port.
use crate::protocol::TableId;
use serde::{Deserialize, Serialize};
use std::io;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action", rename_all = "snake_case", deny_unknown_fields)]
pub enum AdminRequest {
    List,
    ClearInactive,
    Remove { table_id: TableId, force: bool },
}

#[cfg(unix)]
mod local {
    use super::*;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::os::unix::{
        fs::{FileTypeExt, PermissionsExt},
        net::{UnixListener, UnixStream},
    };
    use std::path::PathBuf;
    use std::time::Duration;

    pub struct ServerAdmin {
        listener: UnixListener,
        path: PathBuf,
    }

    impl ServerAdmin {
        pub fn bind(path: &Path) -> io::Result<Self> {
            let parent = path
                .parent()
                .filter(|p| !p.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            if std::fs::metadata(parent)?.permissions().mode() & 0o077 != 0 {
                return Err(io::Error::other(
                    "admin socket requires a private (0700) parent directory",
                ));
            }
            match std::fs::symlink_metadata(path) {
                Ok(metadata) => {
                    if !metadata.file_type().is_socket() {
                        return Err(io::Error::other("admin path exists and is not a socket"));
                    }
                    match UnixStream::connect(path) {
                        Ok(_) => return Err(io::Error::other("admin socket already in use")),
                        Err(error) if error.kind() == io::ErrorKind::ConnectionRefused => {
                            std::fs::remove_file(path)?
                        }
                        Err(error) => return Err(error),
                    }
                }
                Err(error) if error.kind() == io::ErrorKind::NotFound => (),
                Err(error) => return Err(error),
            }
            let listener = UnixListener::bind(path)?;
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))?;
            listener.set_nonblocking(true)?;
            Ok(Self {
                listener,
                path: path.to_path_buf(),
            })
        }

        pub fn poll(
            &self,
            mut handle: impl FnMut(AdminRequest) -> serde_json::Value,
        ) -> io::Result<()> {
            let (mut stream, _) = match self.listener.accept() {
                Ok(connection) => connection,
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => return Ok(()),
                Err(error) => return Err(error),
            };
            stream.set_read_timeout(Some(Duration::from_millis(100)))?;
            stream.set_write_timeout(Some(Duration::from_millis(100)))?;
            let mut line = String::new();
            let response = match BufReader::new((&mut stream).take(1025)).read_line(&mut line) {
                Ok(_) if line.len() <= 1024 && line.ends_with('\n') => {
                    match serde_json::from_str(&line) {
                        Ok(request) => handle(request),
                        Err(_) => serde_json::json!({"error": "invalid operator request"}),
                    }
                }
                _ => serde_json::json!({"error": "operator request is incomplete or too large"}),
            };
            // An operator disconnect must not stop the game server.
            let _ = writeln!(stream, "{response}");
            Ok(())
        }
    }

    impl Drop for ServerAdmin {
        fn drop(&mut self) {
            let _ = std::fs::remove_file(&self.path);
        }
    }

    pub fn request(path: &Path, request: &AdminRequest) -> io::Result<serde_json::Value> {
        let mut stream = UnixStream::connect(path)?;
        stream.set_read_timeout(Some(Duration::from_secs(10)))?;
        stream.set_write_timeout(Some(Duration::from_secs(2)))?;
        writeln!(stream, "{}", serde_json::to_string(request)?)?;
        let mut line = String::new();
        BufReader::new(stream.take(1_048_576)).read_line(&mut line)?;
        serde_json::from_str(&line).map_err(io::Error::other)
    }
}

#[cfg(unix)]
pub use local::{request, ServerAdmin};

#[cfg(not(unix))]
pub struct ServerAdmin;
#[cfg(not(unix))]
impl ServerAdmin {
    pub fn bind(_: &Path) -> io::Result<Self> {
        Err(io::Error::other("operator socket requires Linux or macOS"))
    }
    pub fn poll(&self, _: impl FnMut(AdminRequest) -> serde_json::Value) -> io::Result<()> {
        Ok(())
    }
}
#[cfg(not(unix))]
pub fn request(_: &Path, _: &AdminRequest) -> io::Result<serde_json::Value> {
    Err(io::Error::other(
        "run operator commands on the Linux server",
    ))
}

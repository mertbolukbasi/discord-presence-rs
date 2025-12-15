use crate::activities::Activity;
use crate::error::Error;
use serde_json::json;
use std::io::{Read, Write};
use uuid::Uuid;

/// A trait for Inter-Process Communication (IPC).
pub trait Ipc: Read + Write + Send + Sync {
    /// Connects to the IPC server.
    fn connect() -> Result<Self, Error>
    where
        Self: Sized;
}

/// A struct for Unix IPC.
#[cfg(unix)]
pub struct UnixIpc(std::os::unix::net::UnixStream);

#[cfg(unix)]
impl Ipc for UnixIpc {
    /// Connects to the Discord IPC server on Unix.
    fn connect() -> Result<Self, Error> {
        let path = std::env::var("XDG_RUNTIME_DIR")
            .or_else(|_| std::env::var("TMPDIR"))
            .map(std::path::PathBuf::from)
            .unwrap();

        for i in 0..10 {
            let sock_path = path.join(format!("discord-ipc-{}", i));
            if sock_path.exists() {
                let stream = std::os::unix::net::UnixStream::connect(sock_path)?;
                return Ok(Self(stream));
            }
        }
        Err(Error::ConnectionNotFound)
    }
}

#[cfg(unix)]
impl Read for UnixIpc {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

#[cfg(unix)]
impl Write for UnixIpc {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

/// A struct for Windows IPC.
#[cfg(windows)]
pub struct WindowsIpc(std::fs::File);

#[cfg(windows)]
impl WindowsIpc {
    fn new(file: std::fs::File) -> Self {
        Self(file)
    }
}

#[cfg(windows)]
impl Ipc for WindowsIpc {
    /// Connects to the Discord IPC server on Windows.
    fn connect() -> Result<Self, Error> {
        for i in 0..10 {
            let path = format!(r"\\.\pipe\discord-ipc-{}", i);
            match std::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(path)
            {
                Ok(file) => return Ok(Self(file)),
                Err(_) => continue,
            }
        }
        Err(Error::ConnectionNotFound)
    }
}

#[cfg(windows)]
impl Read for WindowsIpc {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }
}

#[cfg(windows)]
impl Write for WindowsIpc {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }
}

/// The main client for interacting with the Discord Gateway.
pub struct Client {
    ipc: Box<dyn Ipc>,
    /// The client ID of the application.
    pub client_id: String,
}

impl Client {
    /// Creates a new `Client`.
    pub fn new(client_id: &str) -> Result<Self, Error> {
        #[cfg(unix)]
        let ipc = Box::new(UnixIpc::connect()?);
        #[cfg(windows)]
        let ipc = Box::new(WindowsIpc::connect()?);
        #[cfg(not(any(unix, windows)))]
        return Err(Error::ConnectionNotFound);

        let mut client = Self {
            ipc,
            client_id: client_id.to_string(),
        };

        client.handshake()?;
        Ok(client)
    }

    /// Performs the handshake with the Discord IPC server.
    fn handshake(&mut self) -> Result<(), Error> {
        let payload = json!({
            "v": 1,
            "client_id": self.client_id
        });
        self.write_ipc(0, payload.to_string())?;

        let response = self.read_ipc()?;
        let response_data: serde_json::Value = serde_json::from_str(&response)?;

        if response_data["cmd"].as_str() == Some("DISPATCH")
            && response_data["evt"].as_str() == Some("READY")
        {
            Ok(())
        } else {
            Err(Error::HandshakeFailed)
        }
    }

    /// Sets the activity for the user.
    pub fn set_activity(&mut self, activity: Activity) -> Result<(), Error> {
        let payload = json!({
            "cmd": "SET_ACTIVITY",
            "args": {
                "pid": std::process::id(),
                "activity": activity
            },
            "nonce": Uuid::new_v4().to_string()
        });
        self.write_ipc(1, payload.to_string())?;
        Ok(())
    }

    /// Writes a message to the Discord IPC server.
    fn write_ipc(&mut self, opcode: u32, payload: String) -> Result<(), Error> {
        let payload_bytes = payload.as_bytes();
        let len = payload_bytes.len() as u32;

        self.ipc.write_all(&opcode.to_le_bytes())?;
        self.ipc.write_all(&len.to_le_bytes())?;
        self.ipc.write_all(payload_bytes)?;
        Ok(())
    }

    /// Reads a message from the Discord IPC server.
    fn read_ipc(&mut self) -> Result<String, Error> {
        let mut opcode_buf = [0u8; 4];
        let mut len_buf = [0u8; 4];

        self.ipc.read_exact(&mut opcode_buf)?;
        self.ipc.read_exact(&mut len_buf)?;

        let len = u32::from_le_bytes(len_buf);
        let mut payload_buf = vec![0u8; len as usize];
        self.ipc.read_exact(&mut payload_buf)?;

        Ok(String::from_utf8_lossy(&payload_buf).to_string())
    }

    /// Closes the connection to the Discord IPC server.
    pub fn close(&mut self) -> Result<(), Error> {
        self.write_ipc(2, "".to_string())?;
        Ok(())
    }
}

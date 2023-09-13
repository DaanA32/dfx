use std::{fs::{OpenOptions, File}, io::Write, writeln};

use dfx_base::session_id::SessionId;

use crate::session::{SessionSettings, LoggingOptions};

pub trait Logger: Send + std::fmt::Debug {
    fn on_incoming(&self, incoming: &str);
    fn on_outgoing(&self, outgoing: &str);
    fn on_event(&self, event: &str);
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct NoLogger;
impl Logger for NoLogger {
    fn on_incoming(&self, _incoming: &str) {}
    fn on_outgoing(&self, _outgoing: &str) {}
    fn on_event(&self, _event: &str) {}
}

#[derive(Debug, Clone)]
pub struct PrintLnLogger {
    session_id: SessionId,
}
impl Logger for PrintLnLogger {
    fn on_incoming(&self, incoming: &str) {
        println!(
            "[INCOMING] {} {}",
            self.session_id,
            incoming.replace("\x01", "|")
        );
    }
    fn on_outgoing(&self, outgoing: &str) {
        println!(
            "[OUTGOING] {} {}",
            self.session_id,
            outgoing.replace("\x01", "|")
        );
    }
    fn on_event(&self, event: &str) {
        println!("[EVENT   ] {} {}", self.session_id, event);
    }
}

pub trait LogFactory {
    type Log: Logger;
    fn create(&self, session_id: &SessionId) -> Self::Log;
}

#[derive(Debug, Clone)]
pub struct PrintlnLogFactory;
impl PrintLnLogger {
    pub fn new(session_id: &SessionId) -> Self {
        PrintLnLogger {
            session_id: session_id.clone(),
        }
    }
}
impl PrintlnLogFactory {
    pub fn new() -> Self {
        PrintlnLogFactory
    }
}

impl LogFactory for PrintlnLogFactory {
    type Log = PrintLnLogger;
    fn create(&self, session_id: &SessionId) -> Self::Log {
        PrintLnLogger::new(session_id)
    }
}

#[derive(Debug)]
pub struct FileLogger {
    messages_file: File,
    event_file: File,
}

impl FileLogger {
    pub fn new(session_id: &SessionId, options: &LoggingOptions) -> std::io::Result<Self> {
        let log_path = options.file_log_path().map(|f| f.as_str()).unwrap_or_else(|| ".");
        let prefix = session_id.prefix();
        let messages_file_name = format!("{log_path}/{prefix}.messages");
        let event_file_name = format!("{log_path}/{prefix}.event");
        let messages_file = OpenOptions::new().read(true).write(true).create(true).open(messages_file_name)?;
        let event_file = OpenOptions::new().read(true).write(true).create(true).open(event_file_name)?;
        Ok(FileLogger {
            messages_file,
            event_file
        })
    }
}

impl Logger for FileLogger {
    fn on_incoming(&self, incoming: &str) {
        let mut file = &self.messages_file;
        file.write_all(incoming.as_bytes()).unwrap();
        writeln!(file).unwrap();
    }

    fn on_outgoing(&self, outgoing: &str) {
        let mut file = &self.messages_file;
        file.write_all(outgoing.as_bytes()).unwrap();
        writeln!(file).unwrap();
    }

    fn on_event(&self, event: &str) {
        let mut file = &self.event_file;
        file.write_all(event.as_bytes()).unwrap();
        writeln!(file).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct FileLogFactory {
    settings: SessionSettings
}
impl FileLogFactory {
    pub fn new(settings: &SessionSettings) -> Self {
        FileLogFactory {
            settings: settings.clone()
        }
    }
    pub fn boxed(settings: &SessionSettings) -> Box<dyn LogFactory<Log = FileLogger>> {
        Box::new(FileLogFactory::new(settings))
    }
}

impl LogFactory for FileLogFactory {
    type Log = FileLogger;
    fn create(&self, session_id: &SessionId) -> Self::Log {
        let path = self.settings.for_session_id(session_id)
            .unwrap().logging();
        let logger = FileLogger::new(session_id, path);
        logger.unwrap()
    }
}
#[derive(Debug)]
#[cfg(feature = "log")]
pub struct MacroLogger {
    session_id: SessionId,
}

#[cfg(feature = "log")]
impl MacroLogger {
    pub fn new(session_id: &SessionId, _options: &LoggingOptions) -> Self {
        MacroLogger {
            session_id: session_id.clone()
        }
    }
}
#[cfg(feature = "log")]
use log::info;
#[cfg(feature = "log")]
impl Logger for MacroLogger {
    fn on_incoming(&self, incoming: &str) {
        let target = format!("{}.in", self.session_id.prefix());
        info!(target: &target, "{}", incoming);
    }

    fn on_outgoing(&self, outgoing: &str) {
        let target = format!("{}.out", self.session_id.prefix());
        info!(target: &target, "{}", outgoing);
    }

    fn on_event(&self, event: &str) {
        let target = format!("{}.event", self.session_id.prefix());
        info!(target: &target, "{}", event);
    }
}

#[derive(Debug, Clone)]
#[cfg(feature = "log")]
pub struct MacroLogFactory {
    settings: SessionSettings
}
#[cfg(feature = "log")]
impl MacroLogFactory {
    pub fn new(settings: &SessionSettings) -> Self {
        MacroLogFactory {
            settings: settings.clone()
        }
    }
    pub fn boxed(settings: &SessionSettings) -> Box<dyn LogFactory<Log = MacroLogger>> {
        Box::new(MacroLogFactory::new(settings))
    }
}

#[cfg(feature = "log")]
impl LogFactory for MacroLogFactory {
    type Log = MacroLogger;
    fn create(&self, session_id: &SessionId) -> Self::Log {
        let path = self.settings.for_session_id(session_id)
            .unwrap().logging();
        let logger = MacroLogger::new(session_id, path);
        logger
    }
}

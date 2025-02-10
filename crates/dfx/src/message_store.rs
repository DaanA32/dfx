use std::collections::BTreeMap;

use chrono::{DateTime, Utc};

use dfx_base::session_id::SessionId;

pub trait MessageStore: Send + std::fmt::Debug {
    fn reset(&mut self);
    fn creation_time(&self) -> Option<DateTime<Utc>>;
    fn refresh(&mut self);

    fn next_sender_msg_seq_num(&self) -> u32;
    fn set_next_sender_msg_seq_num(&mut self, seq_num: u32);
    fn incr_next_sender_msg_seq_num(&mut self);

    fn next_target_msg_seq_num(&self) -> u32;
    fn set_next_target_msg_seq_num(&mut self, seq_num: u32);
    fn incr_next_target_msg_seq_num(&mut self);

    fn set(&mut self, msg_seq_num: u32, message_string: &str);
    fn get(&self, begin_seq_num: u32, end_seq_num: u32) -> Vec<String>;
}

#[derive(Debug)]
pub(crate) struct MemoryStore {
    messages: BTreeMap<u32, String>,
    next_sender_msg_seq_num: u32,
    next_target_msg_seq_num: u32,
    creation_time: Option<DateTime<Utc>>,
}

impl MemoryStore {
    pub fn new() -> Self {
        MemoryStore {
            messages: BTreeMap::new(),
            next_sender_msg_seq_num: 1,
            next_target_msg_seq_num: 1,
            creation_time: Some(Utc::now()),
        }
    }
}

impl MessageStore for MemoryStore {
    fn reset(&mut self) {
        self.messages.clear();
        self.next_sender_msg_seq_num = 1;
        self.next_target_msg_seq_num = 1;
        self.creation_time = Some(Utc::now());
    }

    fn creation_time(&self) -> Option<DateTime<Utc>> {
        self.creation_time
    }

    fn refresh(&mut self) {}

    fn next_sender_msg_seq_num(&self) -> u32 {
        self.next_sender_msg_seq_num
    }

    fn set_next_sender_msg_seq_num(&mut self, seq_num: u32) {
        self.next_sender_msg_seq_num = seq_num;
    }

    fn incr_next_sender_msg_seq_num(&mut self) {
        self.next_sender_msg_seq_num += 1;
    }

    fn next_target_msg_seq_num(&self) -> u32 {
        self.next_target_msg_seq_num
    }

    fn set_next_target_msg_seq_num(&mut self, seq_num: u32) {
        self.next_target_msg_seq_num = seq_num;
    }

    fn incr_next_target_msg_seq_num(&mut self) {
        self.next_target_msg_seq_num += 1;
    }

    fn set(&mut self, msg_seq_num: u32, message_string: &str) {
        self.messages.insert(msg_seq_num, message_string.into());
    }

    fn get(&self, begin_seq_num: u32, end_seq_num: u32) -> Vec<String> {
        assert!(begin_seq_num <= end_seq_num);
        self.messages
            .range(begin_seq_num..=end_seq_num)
            .map(|(_, v)| v.clone())
            .collect()
    }
}

pub trait MessageStoreFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn MessageStore>;
}

#[derive(Clone, Debug)]
pub struct MemoryStoreFactory;

impl Default for MemoryStoreFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryStoreFactory {
    pub fn new() -> Self {
        MemoryStoreFactory
    }
    pub fn boxed() -> Box<dyn MessageStoreFactory> {
        Box::new(MemoryStoreFactory)
    }
}

impl MessageStoreFactory for MemoryStoreFactory {
    fn create(&self, _session_id: &SessionId) -> Box<dyn MessageStore> {
        Box::new(MemoryStore::new())
    }
}
use std::collections::HashMap;
use std::fs::{self, File, OpenOptions};
use std::io::{self, prelude::*, SeekFrom};
use std::path::{Path, PathBuf};

use crate::session::SessionSettings;

#[derive(Debug)]
pub struct FileStore {
    seq_nums_file_name: String,
    msg_file_name: String,
    header_file_name: String,
    session_file_name: String,

    seq_nums_file: Option<File>,
    msg_file: Option<File>,
    header_file: Option<File>,

    cache: MemoryStore,
    offsets: HashMap<u32, MsgDef>,
}

#[derive(Debug)]
struct MsgDef {
    index: u64,
    size: i32,
}

impl FileStore {
    pub fn new(session_id: &SessionId, path: &PathBuf) -> io::Result<Self> {
        if !Path::new(path).exists() {
            fs::create_dir(path)?;
        }

        let prefix = session_id.prefix();
        let seq_nums_file_name = format!("{}/{}.seqnums", path.as_path().display(), prefix);
        let msg_file_name = format!("{}/{}.body", path.as_path().display(), prefix);
        let header_file_name = format!("{}/{}.header", path.as_path().display(), prefix);
        let session_file_name = format!("{}/{}.session", path.as_path().display(), prefix);

        let mut store = FileStore {
            seq_nums_file_name,
            msg_file_name,
            header_file_name,
            session_file_name,
            seq_nums_file: None,
            msg_file: None,
            header_file: None,
            cache: MemoryStore::new(),
            offsets: HashMap::new(),
        };

        store.open()?;
        Ok(store)
    }

    fn open(&mut self) -> io::Result<()> {
        self.close();

        self.construct_from_file_cache()?;
        self.initialize_session_create_time()?;

        self.seq_nums_file = Some(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&self.seq_nums_file_name)?,
        );
        self.msg_file = Some(
            OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&self.msg_file_name)?,
        );
        self.header_file = Some(
            OpenOptions::new()
                .append(true)
                .create(true)
                .open(&self.header_file_name)?,
        );

        Ok(())
    }

    fn close(&mut self) {
        self.seq_nums_file = None;
        self.msg_file = None;
        self.header_file = None;
    }

    fn purge_single_file<P: AsRef<Path>>(&self, file: Option<&File>, filename: P) {
        if let Some(f) = file {
            let _ = f.sync_all();
        }
        if filename.as_ref().exists() {
            let _ = fs::remove_file(filename);
        }
    }

    fn purge_file_cache(&self) {
        self.purge_single_file(self.seq_nums_file.as_ref(), &self.seq_nums_file_name);
        self.purge_single_file(self.msg_file.as_ref(), &self.msg_file_name);
        self.purge_single_file(self.header_file.as_ref(), &self.header_file_name);
        self.purge_single_file(None::<&File>, &self.session_file_name);
    }

    fn construct_from_file_cache(&mut self) -> io::Result<()> {
        self.offsets.clear();

        if Path::new(&self.header_file_name).exists() {
            let file = File::open(&self.header_file_name)?;
            let reader = io::BufReader::new(file);
            for line in reader.lines() {
                let line = line?;
                let header_parts: Vec<&str> = line.split(',').collect();
                if header_parts.len() == 3 {
                    let seq_num = header_parts[0].parse().unwrap();
                    let index = header_parts[1].parse().unwrap();
                    let size = header_parts[2].parse().unwrap();
                    self.offsets.insert(seq_num, MsgDef { index, size });
                }
            }
        }

        if Path::new(&self.seq_nums_file_name).exists() {
            let mut contents = String::new();
            File::open(&self.seq_nums_file_name)?.read_to_string(&mut contents)?;
            let parts: Vec<&str> = contents.split(':').collect();
            if parts.len() == 2 {
                let next_sender_msg_seq_num = parts[0].trim().parse().unwrap();
                let next_target_msg_seq_num = parts[1].trim().parse().unwrap();
                self.cache.next_sender_msg_seq_num = next_sender_msg_seq_num;
                self.cache.next_target_msg_seq_num = next_target_msg_seq_num;
            }
        }

        Ok(())
    }

    fn initialize_session_create_time(&mut self) -> io::Result<()> {
        if Path::new(&self.session_file_name).exists()
            && fs::metadata(&self.session_file_name)?.len() > 0
        {
            let mut contents = String::new();
            File::open(&self.session_file_name)?.read_to_string(&mut contents)?;
            let creation_time = contents.parse::<DateTime<Utc>>().unwrap();
            self.cache.creation_time = Some(creation_time);
        } else {
            let creation_time_str = self.cache.creation_time.unwrap().to_rfc3339();
            let mut file = File::create(&self.session_file_name)?;
            file.write_all(creation_time_str.as_bytes())?;
        }

        Ok(())
    }

    pub fn get(&self, start_seq_num: u32, end_seq_num: u32) -> io::Result<Vec<String>> {
        let mut messages = Vec::with_capacity((end_seq_num - start_seq_num + 1) as usize);
        for i in start_seq_num..=end_seq_num {
            if let Some(msg_def) = self.offsets.get(&i) {
                let mut msg_bytes = vec![0; msg_def.size as usize];
                self.msg_file
                    .as_ref()
                    .unwrap()
                    .seek(SeekFrom::Start(msg_def.index))?;
                self.msg_file.as_ref().unwrap().read_exact(&mut msg_bytes)?;
                let msg = String::from_utf8_lossy(&msg_bytes).to_string();
                messages.push(msg);
            }
        }
        Ok(messages)
    }

    pub fn set(&mut self, msg_seq_num: u32, msg: &str) -> io::Result<()> {
        self.msg_file.as_mut().unwrap().seek(SeekFrom::End(0))?;
        let offset = self.msg_file.as_ref().unwrap().stream_position()?;
        let msg_bytes = msg.as_bytes();
        let size = msg_bytes.len() as i32;

        let mut header_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.header_file_name)?;
        writeln!(header_file, "{},{},{}", msg_seq_num, offset, size)?;

        self.offsets.insert(
            msg_seq_num,
            MsgDef {
                index: offset,
                size,
            },
        );

        self.msg_file.as_mut().unwrap().write_all(msg_bytes)?;

        Ok(())
    }

    pub fn set_next_sender_msg_seq_num(&mut self, value: u32) -> io::Result<()> {
        self.cache.next_sender_msg_seq_num = value;
        self.set_seq_num()
    }

    pub fn incr_next_sender_msg_seq_num(&mut self) -> io::Result<()> {
        self.cache.incr_next_sender_msg_seq_num();
        self.set_seq_num()
    }

    pub fn set_next_target_msg_seq_num(&mut self, value: u32) -> io::Result<()> {
        self.cache.next_target_msg_seq_num = value;
        self.set_seq_num()
    }

    pub fn incr_next_target_msg_seq_num(&mut self) -> io::Result<()> {
        self.cache.incr_next_target_msg_seq_num();
        self.set_seq_num()
    }

    fn set_seq_num(&mut self) -> io::Result<()> {
        let seq_nums_str = format!(
            "{:010} : {:010}  ",
            self.cache.next_sender_msg_seq_num, self.cache.next_target_msg_seq_num
        );
        let mut seq_nums_file = File::create(&self.seq_nums_file_name)?;
        seq_nums_file.write_all(seq_nums_str.as_bytes())?;
        Ok(())
    }

    pub fn reset(&mut self) -> io::Result<()> {
        self.cache.reset();
        self.purge_file_cache();
        self.open()?;
        Ok(())
    }

    pub fn refresh(&mut self) -> io::Result<()> {
        self.cache.reset();
        self.open()?;
        Ok(())
    }
}

impl MessageStore for FileStore {
    fn reset(&mut self) {
        self.reset().unwrap();
    }

    fn creation_time(&self) -> Option<DateTime<Utc>> {
        self.cache.creation_time()
    }

    fn refresh(&mut self) {
        self.refresh().unwrap();
    }

    fn next_sender_msg_seq_num(&self) -> u32 {
        self.cache.next_sender_msg_seq_num()
    }

    fn set_next_sender_msg_seq_num(&mut self, seq_num: u32) {
        self.set_next_sender_msg_seq_num(seq_num).unwrap();
    }

    fn incr_next_sender_msg_seq_num(&mut self) {
        self.incr_next_sender_msg_seq_num().unwrap();
    }

    fn next_target_msg_seq_num(&self) -> u32 {
        self.cache.next_target_msg_seq_num()
    }

    fn set_next_target_msg_seq_num(&mut self, seq_num: u32) {
        self.set_next_target_msg_seq_num(seq_num).unwrap();
    }

    fn incr_next_target_msg_seq_num(&mut self) {
        self.incr_next_target_msg_seq_num().unwrap();
    }

    fn set(&mut self, msg_seq_num: u32, message_string: &str) {
        self.set(msg_seq_num, message_string).unwrap()
    }

    fn get(&self, begin_seq_num: u32, end_seq_num: u32) -> Vec<String> {
        self.get(begin_seq_num, end_seq_num).unwrap()
    }
}

#[derive(Clone, Debug)]
pub struct FileStoreFactory {
    settings: SessionSettings,
}

impl FileStoreFactory {
    pub fn new(settings: &SessionSettings) -> Self {
        FileStoreFactory {
            settings: settings.clone(),
        }
    }
    pub fn boxed(settings: SessionSettings) -> Box<dyn MessageStoreFactory> {
        Box::new(FileStoreFactory { settings })
    }
}

impl MessageStoreFactory for FileStoreFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn MessageStore> {
        let path = self
            .settings
            .for_session_id(session_id)
            .unwrap()
            .persistence();
        let path = match path {
            crate::session::Persistence::FileStore { path } => path.clone(),
            crate::session::Persistence::Memory => Path::new(".").to_path_buf(),
            crate::session::Persistence::None => Path::new(".").to_path_buf(),
        };
        let store = FileStore::new(session_id, &path);
        Box::new(store.unwrap())
    }
}

#[derive(Clone, Debug)]
pub struct DefaultStoreFactory {
    settings: SessionSettings,
}

impl DefaultStoreFactory {
    pub fn new(settings: &SessionSettings) -> Self {
        DefaultStoreFactory {
            settings: settings.clone(),
        }
    }
    pub fn boxed(settings: &SessionSettings) -> Box<dyn MessageStoreFactory> {
        Box::new(DefaultStoreFactory::new(settings))
    }
}

impl MessageStoreFactory for DefaultStoreFactory {
    fn create(&self, session_id: &SessionId) -> Box<dyn MessageStore> {
        let path = self
            .settings
            .for_session_id(session_id)
            .unwrap()
            .persistence();
        match path {
            crate::session::Persistence::FileStore { path } => {
                Box::new(FileStore::new(session_id, path).unwrap())
            }
            crate::session::Persistence::Memory => Box::new(MemoryStore::new()),
            // REVIEW what should the default be?
            crate::session::Persistence::None => Box::new(MemoryStore::new()),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{assert_eq, path::Path};

    use dfx_base::session_id::SessionId;

    use crate::message_store::MessageStore;

    use super::FileStore;

    #[test]
    fn file_store_rw_test() {
        let session_id = SessionId::new("FIX4.4", "TEST", "", "", "STORE", "", "");
        let path = Path::new("/tmp").to_path_buf();
        {
            let store = FileStore::new(&session_id, &path);
            assert!(store.is_ok());
            let mut store: Box<dyn MessageStore> = Box::new(store.unwrap());
            store.reset();
            store.set_next_sender_msg_seq_num(4);
            store.set_next_target_msg_seq_num(5);
        }
        {
            let store = FileStore::new(&session_id, &path);
            assert!(store.is_ok());
            let store: Box<dyn MessageStore> = Box::new(store.unwrap());
            let sender_seq_num = store.next_sender_msg_seq_num();
            let target_seq_num = store.next_target_msg_seq_num();
            assert_eq!(sender_seq_num, 4);
            assert_eq!(target_seq_num, 5);
        }
    }

    #[test]
    fn file_store_time_test() {
        let session_id = SessionId::new("FIX4.4", "TEST", "", "", "TIME", "", "");
        let path = Path::new("/tmp").to_path_buf();
        let time = {
            let store = FileStore::new(&session_id, &path);
            assert!(store.is_ok());
            let mut store: Box<dyn MessageStore> = Box::new(store.unwrap());
            store.reset();
            store.creation_time()
        };
        {
            let store = FileStore::new(&session_id, &path);
            assert!(store.is_ok());
            let store: Box<dyn MessageStore> = Box::new(store.unwrap());
            assert_eq!(store.creation_time(), time);
        }
    }

    #[test]
    fn file_store_messages_test() {
        let session_id = SessionId::new("FIX4.4", "TEST", "", "", "MESSAGE", "", "");
        let path = Path::new("/tmp").to_path_buf();
        {
            let store = FileStore::new(&session_id, &path);
            assert!(store.is_ok());
            let mut store: Box<dyn MessageStore> = Box::new(store.unwrap());
            store.set(2, "ONE");
            store.set(3, "TWO");
        }
        {
            let store = FileStore::new(&session_id, &path);
            assert!(store.is_ok());
            let store: Box<dyn MessageStore> = Box::new(store.unwrap());
            let messages = store.get(2, 3);
            assert_eq!(messages.len(), 2);
            assert_eq!(messages[0], "ONE");
            assert_eq!(messages[1], "TWO");
        }
    }
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::io::BufWriter;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::Path;

// This is the header that precedes every data entry in our file.
// It helps us check data integrity.
#[derive(Serialize, Deserialize, Debug)]
pub struct EntryHeader {
    pub key_len: u32,
    pub value_len: u32,
    // In a real DB, we would add a CRC32 checksum here to detect corruption.
}

pub struct QuarkStorage {
    // The actual file on disk
    file: File,
    // The in-memory index: Key -> Byte Offset in file
    index: HashMap<String, u64>,
    // Current writing position
    current_offset: u64,
}

impl QuarkStorage {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true) // Create if not exists
            .read(true)
            .write(true)
            .append(true) // Always write at the end
            .open(path)?;

        let mut storage = QuarkStorage {
            file,
            index: HashMap::new(),
            current_offset: 0,
        };

        // When we start, we must load the index from the existing file!
        storage.load_index()?;
        Ok(storage)
    }

    // Writing Data (Append Only)
    pub fn set(&mut self, key: String, value: String) -> io::Result<()> {
        let key_bytes = key.as_bytes();
        let value_bytes = value.as_bytes();

        //  Create Header
        let header = EntryHeader {
            key_len: key_bytes.len() as u32,
            value_len: value_bytes.len() as u32,
        };

        // Serialize Header
        let encoded_header = bincode::serialize(&header).unwrap();

        // Write to disk: [Header][Key][Value]
        // We use a buffer to make it faster, but for simplicity here we write directly
        let mut writer = BufWriter::new(&self.file);
        writer.write_all(&encoded_header)?;
        writer.write_all(key_bytes)?;
        writer.write_all(value_bytes)?;
        writer.flush()?;

        // Update In-Memory Index
        // We point to the start of the header
        self.index.insert(key, self.current_offset);

        // Advance Offset
        self.current_offset += (encoded_header.len() + key_bytes.len() + value_bytes.len()) as u64;

        Ok(())
    }

    // Reading Data
    pub fn get(&mut self, key: &str) -> io::Result<Option<String>> {
        // Check if key exists in memory
        let offset = match self.index.get(key) {
            Some(&o) => o,
            None => return Ok(None),
        };

        //  Seek to the position in the file
        let mut reader = BufReader::new(&self.file);
        reader.seek(SeekFrom::Start(offset))?;

        // Read the Header to know how much to read next
        // We need to know the exact size of the encoded header.
        // For simplicity in this tutorial, let's assume a fixed header size
        // or peek, but standard bincode read works if the stream is positioned.
        let header: EntryHeader = bincode::deserialize_from(&mut reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        //Read Key (we skip it because we already know it, but we must advance the cursor)
        let mut key_buf = vec![0u8; header.key_len as usize];
        reader.read_exact(&mut key_buf)?;

        //Read Value
        let mut val_buf = vec![0u8; header.value_len as usize];
        reader.read_exact(&mut val_buf)?;

        // Convert bytes to String
        let value = String::from_utf8(val_buf)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(Some(value))
    }

    // Rebuilding Index on Startup (Crash Recovery)
    fn load_index(&mut self) -> io::Result<()> {
        // This function needs to read the file from start to finish
        // and populate self.index
        // We will implement this in the next iteration.
        Ok(())
    }
}

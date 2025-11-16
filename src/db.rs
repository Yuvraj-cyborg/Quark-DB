use std::collections::HashMap;
use std::fs;
use std::io;

pub struct CacheDB {
    store: HashMap<String, String>,
}

impl CacheDB {
    pub fn new() -> Self {
        CacheDB {
            store: HashMap::new(),
        }
    }

    // inserting data into the cache
    pub fn put(&mut self, key: String, value: String) {
        self.store.insert(key, value);
    }

    // fetching data from the cache using its key
    pub fn get(&self, key: &str) -> Option<&String> {
        self.store.get(key)
    }

    // deleting/removing data key from table and retruns true/false accordingly
    pub fn delete(&mut self, key: &str) -> bool {
        self.store.remove(key).is_some()
    }

    // how many entries are there in cache
    pub fn size(&self) -> usize {
        self.store.len()
    }

    //Save the cache to a file
    pub fn save(&self, path: &str) -> io::Result<()> {
        let json = serde_json::to_string(&self.store)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> io::Result<Self> {
        let data = fs::read_to_string(path)?;
        let store: HashMap<String, String> = serde_json::from_str(&data)?;
        Ok(Self { store })
    }
}

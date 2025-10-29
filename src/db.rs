use std::collections::HashMap;

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
        self.store.insert(key,value);
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

}



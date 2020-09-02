#[derive(Default)]
pub struct KvStore {}

impl KvStore {
    pub fn new() -> Self {
        KvStore::default()
    }

    pub fn get(&self, key: String) -> Option<String> {
        unimplemented!()
    }

    pub fn set(&mut self, key: String, value: String) -> Option<String> {
        unimplemented!()
    }

    pub fn remove(&mut self, key: String) {
        unimplemented!()
    }
}

use crate::kvs::KeyValueStore;
use crate::task::Existence;
use crate::Result;
use sled::{ConfigBuilder, Db, IVec};
use std::path::Path;

// #[derive(Debug)]
pub struct SledTree {
    tree: Db,
}
impl SledTree {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let config = ConfigBuilder::new().path(path).build();
        let tree = track_any_err!(Db::start(config))?;
        Ok(SledTree { tree })
    }
}
impl KeyValueStore for SledTree {
    type OwnedValue = IVec;

    fn put(&mut self, key: &[u8], value: &[u8]) -> Result<Existence> {
        track_any_err!(self.tree.set(key.to_vec(), value.to_vec()))?;
        Ok(Existence::unknown())
    }

    fn get(&mut self, key: &[u8]) -> Result<Option<Self::OwnedValue>> {
        let value = track_any_err!(self.tree.get(key))?;
        Ok(value)
    }

    fn delete(&mut self, key: &[u8]) -> Result<Existence> {
        let exists = track_any_err!(self.tree.del(key))?.is_some();
        Ok(Existence::new(exists))
    }
}

use curl::easy::{Handler, WriteError};
use std::ops::{Deref, DerefMut};
#[derive(Debug, Default)]
pub struct Collector(pub Vec<u8>);
impl Deref for Collector {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Collector {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl Handler for Collector {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.0.extend_from_slice(data);
        Ok(data.len())
    }
}

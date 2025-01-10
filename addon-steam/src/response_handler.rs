use curl::easy::{Handler, WriteError};
use std::ops::{Deref, DerefMut};
use std::str;
use std::str::Utf8Error;

#[derive(Debug, Clone, Default)]
pub struct ResponseHandler {
    data: Vec<u8>,
}

impl Handler for ResponseHandler {
    fn write(&mut self, data: &[u8]) -> Result<usize, WriteError> {
        self.data.extend_from_slice(data);
        Ok(data.len())
    }
}

impl Deref for ResponseHandler {
    type Target = [u8];
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl AsRef<[u8]> for ResponseHandler {
    fn as_ref(&self) -> &[u8] {
        &self.data
    }
}
impl DerefMut for ResponseHandler {
    fn deref_mut(&mut self) -> &mut [u8] {
        &mut self.data
    }
}
impl AsMut<Vec<u8>> for ResponseHandler {
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
}
#[derive(Debug)]
pub enum ResponseJsonError {
    Utf8Error(Utf8Error),
    JsonError(serde_json::error::Error),
}
impl From<Utf8Error> for ResponseJsonError {
    fn from(err: Utf8Error) -> ResponseJsonError {
        ResponseJsonError::Utf8Error(err)
    }
}
impl From<serde_json::error::Error> for ResponseJsonError {
    fn from(err: serde_json::error::Error) -> ResponseJsonError {
        ResponseJsonError::JsonError(err)
    }
}
impl ResponseHandler {
    pub fn json<T>(&self) -> Result<T, ResponseJsonError>
    where
        T: serde::de::DeserializeOwned,
    {
        let text = str::from_utf8(&self.data)?;
        log::debug!("{}", text);
        let json: T = serde_json::from_str(text)?;
        Ok(json)
    }
}

use crate::kv::ast::{KeyValue, KvToken, KvValue};
use crate::kv::lexer::KvLexer;
use async_recursion::async_recursion;
use eyre::{eyre, Result};
use std::collections::BTreeMap;
use std::io;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::Mutex;
use tokio_stream::{Stream, StreamExt};

#[async_recursion]
pub async fn parse_opt_kv<S>(raw_stream: Arc<Mutex<S>>) -> Result<Option<KeyValue>>
where
    S: Stream<Item = io::Result<KvToken>> + Unpin + Send,
{
    let mut stream = raw_stream.lock().await;
    let key_raw = if let Some(val) = stream.next().await {
        val?
    } else {
        return Ok(None);
    };
    let key = match key_raw {
        KvToken::String(s) => s,
        KvToken::OpenObject => return Err(eyre!("Unexpected open object")),
        KvToken::CloseObject => return Ok(None),
    };
    let val_raw = stream.next().await.unwrap()?;
    let val = match val_raw {
        KvToken::String(s) => KvValue::String(s),
        KvToken::OpenObject => {
            let mut items: BTreeMap<String, KvValue> = BTreeMap::new();
            drop(stream);
            while let Some(item) = parse_opt_kv(raw_stream.clone()).await? {
                items.insert(item.key, item.value);
            }
            KvValue::Object(items)
        }
        KvToken::CloseObject => return Err(eyre!("Unexpected close object")),
    };
    Ok(Some(KeyValue { key, value: val }))
}
pub async fn full_parse<R>(reader: R) -> Result<KeyValue>
where
    R: AsyncReadExt + Unpin + Send,
{
    let raw_tokens = KvLexer::new(reader).tokens();
    tokio::pin!(raw_tokens);
    let tokens = Arc::new(Mutex::new(raw_tokens));

    match parse_opt_kv(tokens.clone()).await {
        Ok(val) => Ok(val.unwrap()),
        Err(err) => Err(err),
    }
}

use crate::kv::ast::KvToken;
use async_stream::try_stream;
use std::io::ErrorKind;
use tokio::io;
use tokio::io::AsyncReadExt;
use tokio_stream::Stream;
pub struct KvLexer<R>(R);

impl<R> KvLexer<R>
where
    R: AsyncReadExt + Unpin,
{
    pub fn new(reader: R) -> KvLexer<R> {
        Self(reader)
    }

    pub fn tokens(mut self) -> impl Stream<Item = io::Result<KvToken>> {
        try_stream! {
            loop {

                let curr = (match self.0.read_u8().await {
                    Ok(c) => Ok(c),
                    Err(e) if e.kind()==ErrorKind::UnexpectedEof => break,
                    Err(e) => Err(e),
                }).unwrap();
                match curr {
                    0 => break,
                    b'{' => yield KvToken::OpenObject,
                    b'}' => yield KvToken::CloseObject,
                    b'"' => {
                        let mut str_bytes = Vec::with_capacity(16);
                        loop {
                            let curr = self.0.read_u8().await.unwrap();
                            if curr == b'"' {
                                break;
                            }
                            str_bytes.push(curr);
                        }
                        yield KvToken::String(String::from_utf8(str_bytes).unwrap());
                    },
                    b'\t' | b'\n'|b' '|b'\r' => {},
                    b => panic!("Invalid token 0x{:02x}", b),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use tokio_stream::StreamExt;

    macro_rules! assert_next_token_eq {
        ($tokens:expr, $val:expr) => {
            assert_eq!($tokens.next().await.map(|v| v.ok()), $val);
        };
    }
    macro_rules! assert_next_token_eq_val {
        ($tokens:expr, $val:expr) => {
            assert_next_token_eq!($tokens, Some(Some($val)));
        };
    }
    #[tokio::test]
    pub async fn test_empty_lex() {
        let mut raw = tokio_test::io::Builder::new().read(b"").build();
        let lexer = KvLexer::new(&mut raw);
        let tokens = lexer.tokens();
        tokio::pin!(tokens);
        assert_next_token_eq!(tokens, None);
    }
    #[tokio::test]
    async fn test_lex_text() {
        let mut raw = tokio_test::io::Builder::new().read(b"\"appid\"").build();
        let lexer = KvLexer::new(&mut raw);
        let tokens = lexer.tokens();
        tokio::pin!(tokens);
        assert_next_token_eq_val!(tokens, KvToken::String("appid".into()));
        assert_next_token_eq!(tokens, None);
    }
    #[tokio::test]
    async fn test_lex_basic_object() {
        let mut raw = tokio_test::io::Builder::new()
            .read(b"{\"appid\" \"228980\" }")
            .build();
        let lexer = KvLexer::new(&mut raw);
        let tokens = lexer.tokens();
        tokio::pin!(tokens);
        assert_next_token_eq_val!(tokens, KvToken::OpenObject);
        assert_next_token_eq_val!(tokens, KvToken::String("appid".into()));
        assert_next_token_eq_val!(tokens, KvToken::String("228980".into()));
        assert_next_token_eq_val!(tokens, KvToken::CloseObject);
        assert_next_token_eq!(tokens, None);
    }
}

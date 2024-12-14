#[rquickjs::module(rename_vars = "camelCase", rename = "fs/promises")]
pub mod fs {
    use tokio::fs::{read_dir, read_to_string};

    #[rquickjs::function]
    pub async fn readdir(path: String) -> rquickjs::Result<Vec<String>> {
        let mut ps = Vec::with_capacity(8);
        let mut iter = read_dir(path).await?;
        while let Some(entry) = iter.next_entry().await? {
            ps.push(entry.path().to_string_lossy().to_string());
        }
        Ok(ps)
    }
    #[rquickjs::function]
    pub async fn read_file(path: String) -> rquickjs::Result<String> {
        Ok( read_to_string(&path).await?)
    }
}

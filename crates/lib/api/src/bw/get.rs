use anyhow::Result;

pub async fn notes(id: &str) -> Result<String> {
    Ok(String::from_utf8(crate::bw::get(["notes", id]).await?)?)
}

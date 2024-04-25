use reqwest::{Body, Client};

pub async fn plain<T>(token: &str, message: T) -> anyhow::Result<()>
where
    T: Into<Body>,
{
    let client = Client::new();
    let url = format!("https://telepush.dev/api/inlets/plain/{}", token);
    let req = client.post(url).body(message);
    let res = req.send().await?;
    res.error_for_status()?;
    Ok(())
}

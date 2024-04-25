use anyhow::Ok;

pub async fn get_token(token: Option<&str>) -> anyhow::Result<String> {
    if let Some(token) = token {
        Ok(token.to_string())
    } else {
        Ok(api::bw::get::notes("Cloudflare").await?)
    }
}

pub async fn get_domain(name: Option<&str>) -> anyhow::Result<String> {
    let name = if let Some(name) = name {
        name.to_string()
    } else {
        format!("{}.ddns.liblaf.me", whoami::fallible::hostname()?)
    };
    Ok(name.to_lowercase())
}

pub async fn get_telepush_token(token: Option<&str>) -> anyhow::Result<String> {
    if let Some(token) = token {
        Ok(token.to_string())
    } else {
        Ok(api::bw::get::notes("TELEPUSH_TOKEN").await?)
    }
}

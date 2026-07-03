//! Web search / open URL. Made by KebiLab

use crate::command::WebOp;
use crate::error::{KebiError, Result};
use std::process::Command;

pub async fn apply(op: WebOp, query: &str) -> Result<Option<String>> {
    let q = query.trim();
    if q.is_empty() {
        return Err(KebiError::Action("Пустой запрос".into()));
    }
    let url = match op {
        WebOp::Open if q.starts_with("http://") || q.starts_with("https://") => q.to_string(),
        WebOp::Open => format!("https://{}", q),
        WebOp::Search => {
            let encoded = url_encode(q);
            format!("https://www.google.com/search?q={encoded}")
        }
    };
    Command::new("cmd")
        .args(["/C", "start", "", &url])
        .spawn()
        .map_err(|e| KebiError::Action(format!("open url: {e}")))?;
    Ok(Some(format!("Открываю: {url}")))
}

fn url_encode(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c.is_ascii_alphanumeric() || matches!(c, '-' | '_' | '.' | '~') {
            out.push(c);
        } else {
            for b in c.to_string().as_bytes() {
                out.push_str(&format!("%{:02X}", b));
            }
        }
    }
    out
}

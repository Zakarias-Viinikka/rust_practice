pub const SERVER_URL: &str = "http://127.0.0.1:3000/call";

pub fn call_server(payload: &str) -> Result<String, String> {
    match ureq::post(SERVER_URL).send_string(payload) {
        Ok(r) => r
            .into_string()
            .map_err(|e| format!("could not read response body: {}", e)),
        Err(ureq::Error::Status(code, r)) => {
            let body = r.into_string().unwrap_or_default();
            Err(format!("HTTP {}: {}", code, body))
        }
        Err(e) => Err(format!("could not reach server: {}", e)),
    }
}
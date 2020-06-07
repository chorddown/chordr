pub struct Credentials {
    pub username: String,
    pub password: String,
}

impl Credentials {
    /// Try to read the `Credentials` from the Basic Auth header
    pub fn from_headers(headers: Vec<&str>) -> Option<Credentials> {
        for header in headers {
            match Credentials::from_header(header) {
                None => { /* continue */ }
                Some(c) => return Some(c),
            }
        }

        None
    }

    /// Try to read the `Credentials` from the Basic Auth header
    pub fn from_header(header: &str) -> Option<Credentials> {
        if !header.starts_with("Basic ") {
            return None;
        }

        println!("{}", header);

        let header_chars = header.chars();
        let base64code = header_chars.into_iter().skip(6).collect::<String>();
        let decoded: String = match base64::decode(&base64code) {
            Ok(vec) => String::from_utf8_lossy(&vec).to_string(),
            Err(_) => return None,
        };

        let parts: Vec<&str> = decoded.splitn(2, ':').collect();
        if parts.len() < 2 {
            return None;
        }

        Some(Credentials {
            username: parts[0].to_string(),
            password: parts[1].to_string(),
        })
    }
}

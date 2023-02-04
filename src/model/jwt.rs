#[derive(serde::Deserialize, serde::Serialize)]
pub struct Calims {
    pub user_id: u32,
    pub exp: u64,
}

impl Calims {
    pub fn from_request_header_map(
        header_map: axum::http::HeaderMap,
    ) -> Result<Self, crate::error::AppError> {
        let token = header_map.get("token");
        match token {
            None => Err(crate::error::AppError::InvalidToken),
            Some(token) => {
                let data = jsonwebtoken::decode::<Calims>(
                    token.to_str().unwrap_or(""),
                    &crate::JWT_KEYS.decoding,
                    &jsonwebtoken::Validation::default(),
                )
                .map_err(|_| crate::error::AppError::InternalServerError)?;
                Ok(data.claims)
            }
        }
    }
}

pub struct Keys {
    pub encoding: jsonwebtoken::EncodingKey,
    pub decoding: jsonwebtoken::DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: jsonwebtoken::EncodingKey::from_secret(secret),
            decoding: jsonwebtoken::DecodingKey::from_secret(secret),
        }
    }
}

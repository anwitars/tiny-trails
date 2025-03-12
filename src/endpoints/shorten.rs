use axum::{extract::State, Json};

use crate::{
    encoding::encode_base62,
    response::TTResponse,
    return_if_error, return_if_errors,
    validation::{Error as TTError, TTInput},
};

const SHORTEN_INPUT_FIELDS: [&str; 1] = ["url"];

fn get_extra_fields_in_json(json: &serde_json::Value, fields: &[&str]) -> Option<Vec<String>> {
    let mut extra_fields = Vec::new();
    for key in json.as_object().unwrap().keys() {
        if !fields.contains(&key.as_str()) {
            extra_fields.push(key.to_string());
        }
    }

    return if extra_fields.is_empty() {
        None
    } else {
        Some(extra_fields)
    };
}

struct ShortenInput {
    url: String,
}

// TODO: maybe write a derive macro for this, or at least have helper functions
impl TTInput for ShortenInput {
    fn from_json(json: &serde_json::Value) -> Result<Self, Vec<TTError>>
    where
        Self: Sized,
    {
        if !json.is_object() {
            return Err(vec![TTError::type_mismatch(
                "object",
                vec![":all:".to_string()],
            )]);
        }

        if let Some(extra_fields) = get_extra_fields_in_json(json, &SHORTEN_INPUT_FIELDS) {
            let errors = extra_fields
                .iter()
                .map(|field| TTError::extra_not_allowed(vec![field.to_string()]))
                .collect();

            return Err(errors);
        }

        let url = json
            .get("url")
            .ok_or_else(|| vec![TTError::required_field(vec!["url".to_string()])])?;

        let url = url
            .as_str()
            .ok_or_else(|| vec![TTError::type_mismatch("string", vec!["url".to_string()])])?;

        url::Url::parse(url).map_err(|_| vec![TTError::invalid_url(vec!["url".to_string()])])?;

        Ok(Self {
            url: url.to_string(),
        })
    }
}

pub async fn shorten(
    State(pool): State<sqlx::SqlitePool>,
    Json(input_json): Json<serde_json::Value>,
) -> TTResponse<String> {
    let input = return_if_errors!(ShortenInput::from_json(&input_json));

    let mut transaction = return_if_error!(pool.begin().await);

    let result = sqlx::query!(
        r#"
        INSERT INTO trails (short, long)
        VALUES (?, ?)
        RETURNING id
        "#,
        "temp",
        input.url
    )
    .fetch_one(&mut *transaction)
    .await;

    let id = return_if_error!(result).id;
    let short = encode_base62(id as u64);

    let result = sqlx::query!(
        r#"
        UPDATE trails
        SET short = ?
        WHERE id = ?
        "#,
        short,
        id
    )
    .execute(&mut *transaction)
    .await;

    return_if_error!(result);
    return_if_error!(transaction.commit().await);

    TTResponse::data(short)
}

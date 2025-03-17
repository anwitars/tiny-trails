use axum::{extract::State, Json};

use crate::{
    encoding::encode_base62,
    response::TTResponse,
    validation::{Error as TTError, TTInput},
};

use super::common::SameErrorResult;

const SHORTEN_INPUT_FIELDS: [&str; 2] = ["url", "expiration_hours"];
const MAX_EXPIRATION_HOURS: u16 = 720;

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
    expiration_hours: Option<u16>,
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

        let expiration_hours = json.get("expiration_hours");
        let expiration_hours = match expiration_hours {
            Some(expiration_hours) => {
                let expiration_hours = expiration_hours.as_i64().ok_or_else(|| {
                    vec![TTError::type_mismatch(
                        "number",
                        vec!["expiration_hours".to_string()],
                    )]
                })?;

                if expiration_hours < 1 {
                    return Err(vec![TTError::min_exceeded(
                        1,
                        vec!["expiration_hours".to_string()],
                    )]);
                }

                if expiration_hours > MAX_EXPIRATION_HOURS as i64 {
                    return Err(vec![TTError::max_exceeded(
                        MAX_EXPIRATION_HOURS.into(),
                        vec!["expiration_hours".to_string()],
                    )]);
                }

                Some(expiration_hours as u16)
            }
            None => None,
        };

        Ok(Self {
            url: url.to_string(),
            expiration_hours,
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ShortenResponseData {
    pub trailid: String,
}

pub async fn shorten(
    State(pool): State<sqlx::PgPool>,
    Json(input_json): Json<serde_json::Value>,
) -> SameErrorResult<TTResponse<ShortenResponseData>> {
    let input = ShortenInput::from_json(&input_json)?;

    let mut transaction = pool.begin().await?;

    // TODO: somehow enforce to use database default for non-nullable Option fields
    let id = sqlx::query!(
        r#"
        INSERT INTO trails (short, long, expiration_hours)
        VALUES ($1, $2, COALESCE($3, 1))
        RETURNING id
        "#,
        "temp",
        input.url,
        input.expiration_hours.map(|x| x as i32)
    )
    .fetch_one(&mut *transaction)
    .await?
    .id;

    let short = encode_base62(id as u64);

    sqlx::query!(
        r#"
        UPDATE trails
        SET short = $1
        WHERE id = $2
        "#,
        short,
        id
    )
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Ok(TTResponse::Data(ShortenResponseData { trailid: short }))
}

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::ServiceExt;

    use crate::{
        app, encoding::encode_base62, endpoints::shorten::ShortenResponseData,
        response::TTResponse, utils::testing::BodyDeserializeJson, validation,
    };

    #[sqlx::test]
    async fn test_ok(pool: sqlx::PgPool) {
        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/shorten")
                    .body(Body::from(r#"{"url":"https://example.com"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response
            .into_body()
            .deserialize_json::<TTResponse<ShortenResponseData>>()
            .await;

        assert!(matches!(body, TTResponse::Data(_)));
        let body = body.unwrap_data();

        // this is the first trail in the database, so the expected id in db is 1
        let expected_trail_id = encode_base62(1);

        assert_eq!(body.trailid, expected_trail_id);
    }

    #[sqlx::test]
    async fn test_invalid_url(pool: sqlx::PgPool) {
        let app = app(pool);

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .header("Content-Type", "application/json")
                    .uri("/shorten")
                    .body(Body::from(r#"{"url":"invalid"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = response
            .into_body()
            .deserialize_json::<TTResponse<()>>()
            .await;

        assert!(matches!(body, TTResponse::Errors(_)));
        let errors = body.unwrap_errors();

        assert!(errors.len() == 1);

        let error = &errors[0];
        let expected_error = validation::Error::invalid_url(vec!["url".to_string()]);

        assert_eq!(error, &expected_error);
    }
}

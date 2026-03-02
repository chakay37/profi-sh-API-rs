use axum::{extract, http};
use axum::extract::path::ErrorKind::Message;
use serde::{Deserialize, Serialize};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey, errors::Error as JwtError};
use sqlx::{FromRow, PgPool};
use chrono::{Utc, Duration};
use bcrypt::{verify, hash};
use hyper::StatusCode;
use sqlx::types::Text;
const SECRET: &[u8] = b"";

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Articles {
    id: i32,
    name: String,
    date: chrono::NaiveDate,
    text: String
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Cities {
    id: i32,
    name: String,
    is_displayed: bool
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Deals {
    id: i32,
    date: chrono::NaiveDate,
    shopId: i32,
    enddate: chrono::NaiveDate,
    value: String,
    text: String,
    priority: i16
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Photos {
    id: i32,
    desc: Option<String>,
    name: String,
    url: Option<String>
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Shops {
    id: i32,
    name: String,
    Mon: String,
    Tue: String,
    Wed: String,
    Thu: String,
    Fri: String,
    Sat: String,
    mapsrc: String,
    adresa: String,
    urlname: String
}
#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Users {
    id: i64,
    email: String,
    phone: String,
    cityid: i64,
    date_registered: Option<chrono::DateTime<chrono::Utc>>,
}
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize
}

impl Articles {
    pub fn new(id: i32, name: String, date: chrono::NaiveDate, text: String) -> Articles {
        Self { id, name, date, text }
    }
}
impl Cities {
    pub fn new(id: i32, name: String, is_displayed: bool) -> Cities {
        Self { id, name, is_displayed }
    }
}
impl Deals {
    pub fn new(id: i32, date: chrono::NaiveDate, shopId: i32, enddate: chrono::NaiveDate, value: String, text: String, priority: i16) -> Deals {
        Self { id, date, shopId, enddate, value, text, priority }
    }
}
impl Photos {
    pub fn new(id: i32, desc: Option<String>, name: String, url: Option<String>) -> Photos {
        Self { id, desc, name, url }
    }
}
impl Shops {
    pub fn new(id: i32, name: String, Mon: String, Tue: String, Wed: String, Thu: String, Fri: String, Sat: String, mapsrc: String, adresa: String, urlname: String) -> Shops {
        Self {id, name, Mon, Tue, Wed, Thu, Fri, Sat, mapsrc, adresa, urlname }
    }
}
impl Users {
    pub fn new(id: i64, email: String, phone: String, cityid: i64, date_registered: Option<chrono::DateTime<chrono::Utc>>) -> Users {
        Self { id, email, phone, cityid, date_registered }
    }
}

pub async fn hello_world() -> &'static str {
    "hello world"
}

fn generate_jwt(username: &str) -> Result<String, JwtError> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET))
}

fn validate_jwt(token: &str) -> Result<Claims, JwtError> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
pub async fn get_articles(
    extract::State(pool): extract::State<PgPool>
) -> Result<axum::Json<Vec<Articles>>, String> {
    let res = sqlx::query_as::<_, Articles>(r#"SELECT * FROM "Articles""#)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(|e| e.to_string());

    res
}

pub async fn get_cities(
    extract::State(pool): extract::State<PgPool>
) -> Result<axum::Json<Vec<Cities>>, String> {
    let res = sqlx::query_as::<_, Cities>(r#"SELECT * FROM "Cities""#)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(|e| e.to_string());

    res
}

pub async fn get_deals(
    extract::State(pool): extract::State<PgPool>
) -> Result<axum::Json<Vec<Deals>>, String> {
    let res = sqlx::query_as::<_, Deals>(r#"SELECT * FROM "Deals""#)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(|e| e.to_string());

    res
}

pub async fn get_photos(
    extract::State(pool): extract::State<PgPool>
) -> Result<axum::Json<Vec<Photos>>, String> {
    let res = sqlx::query_as::<_, Photos>(r#"SELECT * FROM "Photos""#)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(|e| e.to_string());

    res
}


pub async fn get_shops(
    extract::State(pool): extract::State<PgPool>
) -> Result<axum::Json<Vec<Shops>>, String> {
    let res = sqlx::query_as::<_, Shops>(r#"SELECT * FROM "Shops""#)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(|e| e.to_string());

    res
}

pub async fn get_users(
    extract::State(pool): extract::State<PgPool>
) -> Result<axum::Json<Vec<Users>>, String> {
    let res = sqlx::query_as::<_, Users>(r#"SELECT * FROM "Users""#)
        .fetch_all(&pool)
        .await
        .map(axum::Json)
        .map_err(|e| e.to_string());

    res
}

pub async fn post_users(extract::State(pool): extract::State<PgPool>,
                          axum::Json(payload): axum::Json<Users>,
) -> Result<axum::Json<String>, http::StatusCode> {
    let user = Users::new(payload.id, payload.email, payload.phone, payload.cityid, payload.date_registered);

    let res = sqlx::query(
        r#"
        INSERT INTO "Users" (id, email, phone, cityid, date_registered)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.phone)
        .bind(&user.cityid)
        .bind(&user.date_registered)
        .execute(&pool)
        .await;

    match res {
        Ok(result) => {
            let rows = result.rows_affected();
            Ok(axum::Json(format!("{} row(s) affected", rows)))
        },
        Err(_) => Err(http::StatusCode::INTERNAL_SERVER_ERROR),
    }
}
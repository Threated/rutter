use crate::{db::Redis, responders::JsonRes};
use rocket::{
    serde::json::Json,
    http::{
        CookieJar,
        Cookie,
        Status
    },
    request::{FromRequest, Outcome},
    Request,
    outcome::IntoOutcome
};

use serde::{Deserialize, Serialize};

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};
use jsonwebtoken::{decode, encode, Validation, DecodingKey, EncodingKey, Header};
use chrono::{Utc, Duration};

#[derive(Deserialize)]
pub struct User<'r> {
    pub name: &'r str,
    pub password: &'r str,
}

impl<'r> User<'r> {
    pub fn hash_pw(&self) -> String {
        Argon2::default()
            .hash_password(
                self.password.as_bytes(),
                &SaltString::generate(&mut OsRng)
            )
            .unwrap()
            .to_string()
    }

    pub fn verify_password(&self, password: &str) -> bool {
        Argon2::default().verify_password(
            self.password.as_bytes(),
            &PasswordHash::new(password).unwrap()
        ).is_ok()
    }
}

/// Request Guard
pub struct Authenticated {
    pub name: String
}

/// jwt claims
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    name: String,
    exp: usize
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authenticated {
    type Error = String;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request.headers()
            .get("Authorization")
            .nth(0).and_then(|jwt| dbg!(jwt.split_once("Bearer ").map(|(_, jwt)| jwt)))
            .and_then(|jwt| dbg!(decode::<Claims>(jwt, &DecodingKey::from_secret(rocket::Config::SECRET_KEY.as_ref()), &Validation::default())).ok())
            .map(|token| Authenticated {name: token.claims.name})
            .or_forward(())
    }
}

#[post("/login", format="json", data="<user>")]
async fn login(user: Json<User<'_>>, mut db: Redis, jar: &CookieJar<'_>) -> JsonRes {
    let password = db.get_pw_by_name(user.name).await;
    if password.is_some() && user.verify_password(&password.unwrap()) {
        jar.add(Cookie::new("jwt", encode(
            &Header::default(),
            &Claims {
                name: user.name.to_string(),
                exp: (Utc::now() + Duration::weeks(2)).timestamp() as usize
            },
            &EncodingKey::from_secret(rocket::Config::SECRET_KEY.as_ref())
        ).unwrap()));
        JsonRes::from((Status::Ok, "Login Successfull"))
    } else {
        JsonRes::from((Status::Unauthorized, "Wrong username or password"))
    }
}

#[post("/register", format="json", data="<user>")]
async fn register(user: Json<User<'_>>, mut db: Redis) -> JsonRes {
    if db.register_user(&user).await {
        JsonRes::from((Status::Created, "Created User"))
    } else {
        JsonRes::from((Status::Conflict, "User already exists"))
    }
}

#[post("/logout")]
async fn logout<'a>(jar: &CookieJar<'_>) -> JsonRes {
    jar.remove(Cookie::named("jwt"));
    JsonRes::from((Status::ResetContent, "User logged out"))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![register, login, logout]
}

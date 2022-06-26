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

use serde::Deserialize;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};

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

pub struct Authenticated {
    pub name: String
}


#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authenticated {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        request.cookies()
            .get_private("session_id")
            .map(|c| Authenticated {name: c.value().to_string()})
            .or_forward(())
    }
}

#[post("/login", format="json", data="<user>")]
async fn login(user: Json<User<'_>>, mut db: Redis, jar: &CookieJar<'_>) -> JsonRes {
    let password = db.get_pw_by_name(user.name).await;
    if password.is_some() && user.verify_password(&password.unwrap()) {
        jar.add_private(Cookie::new("session_id", String::from(user.name)));
        (Status::Ok, "Login Successfull").into()
    } else {
        (Status::Unauthorized, "Wrong username or password").into()
    }
}

#[post("/register", format="json", data="<user>")]
async fn register(user: Json<User<'_>>, mut db: Redis) -> JsonRes {
    if db.register_user(&user).await {
        (Status::Created, "Created User").into()
    } else {
        (Status::Conflict, "User already exists").into()
    }
}

#[post("/logout")]
async fn logout<'a>(jar: &CookieJar<'_>) -> JsonRes {
    jar.remove_private(Cookie::new("session_id", ""));
    (Status::ResetContent, "User logged out").into()
}

pub fn routes() -> Vec<rocket::Route> {
    routes![register, login, logout]
}

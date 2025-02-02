use axum_extra::extract::cookie::Key;
use axum_extra::extract::cookie::{Cookie, SignedCookieJar};
use cookie::time::Duration;
use cookie::SameSite;
use serde::{Deserialize, Serialize};
use serde_json;
use uuid::Uuid;
use std::env;

#[derive(Serialize, Deserialize, Debug)]
    pub struct SessionData {
    pub user_id: Option<Uuid>,
    pub game_id: Option<Uuid>
}

pub fn get_cookie_key() -> Key {
    let secret = env::var("COOKIE_SECRET").expect("COOKIE_SECRET mancante");
    Key::from(secret.as_bytes())
}

pub fn add_session(jar: SignedCookieJar, session: SessionData) -> SignedCookieJar {
    let session_value =
        serde_json::to_string(&session).expect("Impossibile serializzare SessionData");

    let cookie: Cookie = Cookie::build(("session", session_value))
        .domain("localhost")
        .path("/")
        .secure(false) // TRUE in prod
        .max_age(Duration::days(1))
        .http_only(true)
        .same_site(SameSite::None)
        .build();

    println!("COOKIE CREATED: {:?}", cookie);

    jar.add(cookie)
}

pub fn read_session(jar: SignedCookieJar) -> Option<SessionData> {
    
    let cookie = jar.get("session")
        .and_then(|cookie| serde_json::from_str(cookie.value()).ok());

    println!("COOKIE READ: {:?}", cookie);

    cookie
}

pub fn remove_session(jar: SignedCookieJar) -> SignedCookieJar {
    jar.remove(Cookie::from("session"))
}

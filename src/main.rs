#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use chrono::prelude::*;
use rocket::response::content;
use serde_json::json;

const VERSION: &'static str = "0.1.0";

#[get("/")]
fn date() -> content::Json<String> {
    content::Json(
        json!({
            "date": Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()
        })
        .to_string(),
    )
}

#[get("/")]
fn version() -> content::Json<String> {
    content::Json(json!({"version": VERSION}).to_string())
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/date", routes![date])
        .mount("/version", routes![version])
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use super::VERSION;
    use chrono::prelude::*;
    use rocket::http::{ContentType, Status};
    use rocket::local::Client;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    struct Date {
        date: String,
    }

    #[derive(Serialize, Deserialize)]
    struct Version {
        version: String,
    }

    #[test]
    fn date() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/date").dispatch();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let d: Date = serde_json::from_str(&response.body_string().expect("nonempty response"))
            .expect("valid date object");
        assert_eq!(d.date, now);
    }

    #[test]
    fn version() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/version").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let v: Version = serde_json::from_str(&response.body_string().expect("nonempty response"))
            .expect("valid version object");
        assert_eq!(v.version, VERSION);
    }
}

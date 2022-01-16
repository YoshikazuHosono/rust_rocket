use rocket_contrib::json::Json;

use crate::models::User;

#[get("/")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/user")]
pub fn get_user() -> Json<Vec<User>> {
    Json(vec![
        User {
            user_id: 1,
            user_name: String::from("hosono"),
        },
        User {
            user_id: 2,
            user_name: String::from("yoshikazu"),
        },
    ])
}

#[get("/user/<user_id>")]
pub fn get_user_by_user_id(user_id: u32) -> Json<User> {
    Json(match user_id {
        1 => User {
            user_id: 1,
            user_name: String::from("hosono"),
        },
        2 => User {
            user_id: 2,
            user_name: String::from("yoshikazu"),
        },
        _ => User {
            user_id: 999,
            user_name: String::from("ke tu ba n"),
        },
    })
}

#[post("/user", data = "<user>")]
pub fn post_user(user: Json<User>) -> String {
    format!("{:?}", user.0)
}

#[cfg(test)]
mod test {
    extern crate rocket;

    use rocket::http::{ContentType, Status};
    use rocket::local::Client;

    use crate::routes::*;

    fn rocket() -> rocket::Rocket {
        rocket::ignite().mount(
            "/",
            routes![index, get_user, get_user_by_user_id, post_user],
        )
    }

    #[test]
    fn test_index() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("Hello, world!".into()));
    }

    #[test]
    fn test_get_user() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/user").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.body_string(), Some("[{\"user_id\":1,\"user_name\":\"hosono\"},{\"user_id\":2,\"user_name\":\"yoshikazu\"}]".into()));
    }

    #[test]
    fn test_get_user_by_user_id_1() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/user/1").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("{\"user_id\":1,\"user_name\":\"hosono\"}".into())
        );
    }

    #[test]
    fn test_get_user_by_user_id_2() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/user/2").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("{\"user_id\":2,\"user_name\":\"yoshikazu\"}".into())
        );
    }

    #[test]
    fn test_get_user_by_user_id_3() {
        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client.get("/user/3").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("{\"user_id\":999,\"user_name\":\"ke tu ba n\"}".into())
        );
    }

    #[test]
    fn test_post_user() {
        let user = r#"
            {
                "user_id": 8 ,
                "user_name": "popopo"
            }
        "#;

        let client = Client::new(rocket()).expect("valid rocket instance");
        let mut response = client
            .post("/user")
            .header(ContentType::JSON)
            .body(user)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some(r#"User { user_id: 8, user_name: "popopo" }"#.into())
        );
    }
}

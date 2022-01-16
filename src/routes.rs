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
pub fn new_todo(user: Json<User>) -> String {
    format!("{:?}", user.0)
}

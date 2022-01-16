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

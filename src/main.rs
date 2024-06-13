#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, json::Json};
use self::models::*;
use diesel::prelude::*;
use MyCloudAPI::*;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    if age > 18 {
        format!("You're a old person {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your age wait {}.", name, (18-age))
    }
}

#[get("/getuser")]
fn getuser() -> String {
    format!("Hello ?")
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index]).mount("/", routes![hello]).mount("/", routes![getuser]);
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .filter(published.eq(true))
        .limit(5)
        .select(Users::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.login_user);
        println!("-----------\n");
        println!("{}", user.password_user);
    }
}

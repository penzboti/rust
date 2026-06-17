#[macro_use]
extern crate rocket;

// password ENV
// users txt
// game txt

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/login")]
fn login() -> &'static str {
    "/error"
}

#[get("/error")]
fn error() -> &'static str {
    "Error"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,login,error])
}

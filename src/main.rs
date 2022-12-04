#[macro_use]
extern crate rocket;

#[get("/v1")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![world])
}

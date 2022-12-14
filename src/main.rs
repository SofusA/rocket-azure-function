use std::env;

#[macro_use]
extern crate rocket;

#[get("/v1")]
fn world() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    let figment = rocket::Config::figment()
        .merge(("port", port));

    let rocket = rocket::custom(figment).mount("/api", routes![world]);

    let iter = rocket.routes();

    for route in iter {
        let path = route.uri.origin.path().raw();
        println!("Routes:");
        println!(">{:?}: {:?}", route.method, path);
    };

    rocket
}
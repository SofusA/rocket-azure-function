#[get("/v1")]
fn world() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod routes_tests {
    use crate::routes::{get_routes, Route};

    use super::*;
    #[test]
    fn get_routes_test() {
        let rocket = rocket::build().mount("/api", routes![world]);

        let routes = get_routes(rocket);
        let expected_routes = vec![Route { method: crate::routes::Method::Get, route: "/api/v1".to_string() }];

        assert_eq!(routes.first().unwrap().method, expected_routes.first().unwrap().method);
    }
}   
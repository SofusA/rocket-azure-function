mod routes_test;
use rocket::{Rocket, Build};

pub fn get_routes(rocket: Rocket<Build>) -> Vec<Route> {
    let iter = rocket.routes();
    let mut routes: Vec<Route> = Vec::new();

    for route in iter {
        routes.push(Route { method: method_to_internal_method(route.method), route: route.uri.origin.path().raw().to_string() });
    };

    return routes;
}

#[derive(Debug, PartialEq)]
pub struct Route {
    pub method: Method,
    pub route: String 
}

#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Put,
    Post,
    Delete,
    Options,
    Head,
    Trace,
    Connect,
    Patch,
}

pub fn method_to_internal_method(method: rocket::http::Method) -> Method {
    return match method {
        rocket::http::Method::Get => Method::Get,
        rocket::http::Method::Put => Method::Put,
        rocket::http::Method::Post => Method::Post,
        rocket::http::Method::Delete => Method::Delete,
        rocket::http::Method::Options => Method::Options,
        rocket::http::Method::Head => Method::Head,
        rocket::http::Method::Trace => Method::Trace,
        rocket::http::Method::Connect => Method::Connect,
        rocket::http::Method::Patch => Method::Patch,
    }
}
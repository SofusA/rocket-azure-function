mod routes_test;
use std::collections::HashSet;

use rocket::{Build, Rocket};

pub fn get_routes(rocket: Rocket<Build>) -> EndPoint<'static> {
    let mut routes = EndPoint {
        route: "/".to_string(),
        methods: vec![],
        sub_routes: None,
    };

    let iter = rocket.routes();
    for route in iter {
        let method = method_to_internal_method(route.method);

        for sub_route in route.uri.origin.path().raw().split('/') {
            match  {
                
            }

        }

        routes.push(Route {
            method: method_to_internal_method(route.method),
            route: sub_routes,
        });
    }

    return routes;
}

fn create_endpoint(method: Method, route: String) -> EndPoint<'static> {
    EndPoint { route: route, methods: vec![method], sub_routes: None }
}

#[derive(Debug)]
pub struct EndPoint<'a> {
    pub route: String,
    pub methods: Vec<Method>,
    pub sub_routes: Option<Vec<&'a EndPoint<'a>>>,
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
    };
}

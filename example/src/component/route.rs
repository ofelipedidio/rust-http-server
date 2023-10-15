use std::{collections::HashMap, error::Error};

use super::{http_service::{HttpService, DeferredRouteService}, escape::unescape_str, router::Router};

pub struct RoutePathService {
    path: RoutePathObject,
    service: Box<dyn HttpService>,
}

impl RoutePathService {
    pub fn new(path: RoutePathObject, service: Box<dyn HttpService>) -> Self {
        RoutePathService { path, service }
    }

    pub(crate) fn new_with_deferred(path: RoutePathObject, router: Router) -> Self {
        let service = DeferredRouteService::new(path.clone(), router);
        RoutePathService { path, service: Box::new(service) }
    }
}

#[derive(Default)]
pub struct DefaultRoute();

// TODO: HashMap<String, String> does not model queries correctly. You can input
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoutePathObject {
    path: Vec<PathSegment>,
    query: HashMap<String, String>,
}

impl RoutePathObject {
    pub fn path(&self) -> &Vec<PathSegment> {
        &self.path
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum PathSegment {
    Literal(String),
    Wildcard,
}

impl PathSegment {
    fn from<S: Into<String>>(seg: S) -> Self {
        let seg = seg.into();
        match seg.as_str() {
            "*" => PathSegment::Wildcard,
            _ => PathSegment::Literal(seg),
        }
    }
}

impl RoutePathObject {
    pub fn new(path: Vec<PathSegment>, query: HashMap<String, String>) -> Self {
        RoutePathObject { path, query }
    }
}

impl TryInto<RoutePathObject> for &str {
    type Error = Box<dyn Error>;

    fn try_into(self) -> Result<RoutePathObject, Self::Error> {
        let mut parts = self.splitn(2, "?");

        let path = parts.next().expect("Split always has at least one segment");
        let path: Result<_, _> = path
            .split("/")
            .map(|seg| unescape_str(seg))
            .map(|seg| seg.map(|seg| PathSegment::from(seg)))
            .collect();
        let path = path?;

        let query = parts.next().unwrap_or("");
        let query: Result<_, Box<dyn Error>> = query
            .split("&")
            .filter(|s| s.len() > 0)
            .map(|x| {
                let mut x = x.splitn(2, "=");
                let first = x.next().expect("Split always has at least one segment");
                let second = x.next().unwrap_or("");
                let first = unescape_str(first)?;
                let second = unescape_str(second)?;
                Ok((first, second))
            })
            .collect();
        let query = query?;

        Ok(RoutePathObject::new(path, query))
    }
}

impl TryInto<RoutePathObject> for DefaultRoute {
    type Error = Box<dyn Error>;
    fn try_into(self) -> Result<RoutePathObject, Self::Error> {
        Ok(RoutePathObject::new(
            vec![PathSegment::Wildcard],
            HashMap::new(),
        ))
    }
}



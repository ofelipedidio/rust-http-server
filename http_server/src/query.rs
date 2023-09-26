use std::collections::HashMap;

use crate::path::Path;

#[derive(Debug)]
pub struct QueryPath<P: Path> {
    pub path: P,
    pub query: HashMap<String, String>,
}

impl <P: Path> Path for QueryPath<P> {
    type Error = ();

    fn parse<S: AsRef<str>>(path: S) -> Result<Self, Self::Error> {
        let path = path.as_ref();
        let mut split_path = path.splitn(2, "?");
        let path = split_path
            .next()
            .unwrap();
        let path = P::parse(path)
            .unwrap();
        let query = split_path
            .flat_map(|s| s.split("&"))
            .map(|s| s.splitn(2, "="))
            .map(|mut s| (s.next().unwrap(), s.next().unwrap_or("")))
            .map(|(k,v)| (k.to_string(), v.to_string()));
        let query: HashMap<String, String> = query.collect();

        Ok(QueryPath { path, query })
    }
}

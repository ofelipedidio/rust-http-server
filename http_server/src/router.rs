use std::collections::HashMap;
use std::hash::Hash;
use crate::query::QueryPath;
use crate::request::HttpRequest;
use crate::response::{HttpResponse, HttpBody};

pub type RequestHandler<P, B> = fn(HttpRequest<P>) -> HttpResponse<B>;

/*
pub enum HierarchicalKey {
    String(String),
    Variable(String),
}

pub trait HttpPath : Sized {
    fn to_hierarchical_key(self) -> HierarchicalKey;
}

impl HttpPath for String {
    fn to_hierarchical_key(self) -> HierarchicalKey {
        HierarchicalKey::String(self)
    }
}

impl HttpPath for &str {
    fn to_hierarchical_key(self) -> HierarchicalKey {
        HierarchicalKey::String(self.to_string())
    }
}

pub struct HierarchicalHashMap<K, V>(Option<V>, HashMap<K, HierarchicalHashMap<K, V>>);

impl <K : Hash + Eq, V> HierarchicalHashMap<K, V> {
    pub fn insert(&mut self, key: &[K], value: V) {
        if key.len() == 0 {
        }
        todo!()
    }
}

pub type RouterRequest = RequestHandler<QueryPath<String>, Box<dyn HttpBody>>;

pub struct Router {
    router: HierarchicalHashMap<HierarchicalKey, RouterRequest>,
}

impl Router {
    pub fn get<P: HttpPath>(&mut self, path: P, handler: RouterRequest) {
        let path = path.to_hierarchical_key();

        self.router.insert(path, handler);
    }
}
*/ 


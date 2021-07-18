use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use lazy_static::lazy_static;
use mario_core::Collector;

struct RegistryInner {
    pub collectors_by_name: HashMap<String, Box<dyn Collector>>,
}

impl RegistryInner {
    fn register(&mut self, c: Box<dyn Collector>) {
        self.collectors_by_name.entry(c.get_name()).or_insert(c);
    }

    fn unregister(&mut self, c: Box<dyn Collector>) {
        let collect_name = c.get_name();
        self.collectors_by_name.remove(&collect_name);
    }
}

impl std::fmt::Debug for RegistryInner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Registry ({} collectors)",
            self.collectors_by_name.keys().len()
        )
    }
}

#[derive(Clone, Debug)]
pub struct Registry {
    inner: Arc<RwLock<RegistryInner>>,
}

impl Default for Registry {
    fn default() -> Self {
        let r = RegistryInner { collectors_by_name: HashMap::new() };
        Registry { inner: Arc::new(RwLock::new(r)) }
    }
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&self, c: Box<dyn Collector>) {
        self.inner.write().unwrap().register(c)
    }

    pub fn unregister(&self, c: Box<dyn Collector>) {
        self.inner.write().unwrap().unregister(c)
    }
}

lazy_static! {
    static ref DEFAULT_REGISTRY: Registry = Registry::new();
}

pub fn default_registry() -> &'static Registry {
    lazy_static::initialize(&DEFAULT_REGISTRY);
    &DEFAULT_REGISTRY
}

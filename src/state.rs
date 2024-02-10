use std::collections::HashMap;
use std::any::Any;
use std::sync::RwLock;
use lazy_static::lazy_static;
use std::fmt::Debug;

pub trait CloneableAny: Any + Send + Sync {
    fn clone_any(&self) -> impl CloneableAny;
}

pub type Value = Box<dyn CloneableAny>;

pub struct State {
    pub state_map: HashMap<String, Value>
}

lazy_static!{
    pub static ref STATE: RwLock<State> = RwLock::new(State { state_map: HashMap::new() });
}

use std::collections::HashMap;
use std::any::Any;
use std::sync::RwLock;
use lazy_static::lazy_static;

pub type Value = Box<dyn Any + Send + Sync>;

pub struct State {
    pub state_map: HashMap<String, Value>
}

lazy_static!{
    pub static ref STATE: RwLock<State> = RwLock::new(State { state_map: HashMap::new() });
}

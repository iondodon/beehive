use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::{Arc, RwLock};
use lazy_static::lazy_static;


pub trait Value: Send + Sync + Debug {

}

impl<T> Value for T 
where
    T: 'static + Send + Sync + Debug
{
    
}

impl Display for dyn Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct State {
    pub store: HashMap<String, Arc<dyn Value>>
}

lazy_static!{
    pub static ref STATE: RwLock<State> = RwLock::new(State { store: HashMap::new() });
}

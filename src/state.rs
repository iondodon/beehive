use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::sync::RwLock;
use lazy_static::lazy_static;

// Define an object-safe cloning trait.
pub trait Value: Send + Sync + Debug {
    fn clone_value(&self) -> Box<dyn Value>;

    // Method to help with Display implementation
    fn as_debug(&self) -> &dyn Debug; 
}

// Implement the CloneBox trait for any type that implements Clone.
impl<T> Value for T
where
    T: Copy + Clone + Debug + Send + Sync
{
    fn clone_value(&self) -> Box<dyn Value> {
        Box::new(self)
    }

    fn as_debug(&self) -> &dyn Debug {
        self
    }
}

// Since we cannot directly implement Display for dyn Value (as it might not be meaningful for all types),
// we provide a workaround using Debug trait for demonstration.
impl Display for dyn Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.as_debug())
    }
}

pub struct State {
    pub store: HashMap<String, Box<dyn Value>>
}

lazy_static!{
    pub static ref STATE: RwLock<State> = RwLock::new(State { store: HashMap::new() });
}

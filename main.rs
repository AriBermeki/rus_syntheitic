use std::collections::HashMap;
use std::any::Any;
use std::rc::Rc;

pub struct SyntheticEvent {
    register_callback_map: HashMap<String, Rc<dyn Any>>,
}

impl SyntheticEvent {
    pub fn new() -> Self {
        SyntheticEvent {
            register_callback_map: HashMap::new(),
        }
    }

    pub fn register_callback(&mut self, func_name: String, callback: Rc<dyn Any>) {
        self.register_callback_map.insert(func_name, callback);
    }

    pub fn make_callback(&mut self, func_name: &str, args: &HashMap<String, Rc<dyn Any>>) {
        if let Some(callback) = self.register_callback_map.get(func_name) {
            // Assuming callback is a function with the signature: fn(&HashMap<String, Rc<dyn Any>>)
            if let Some(callback_fn) = callback.downcast_ref::<fn(&HashMap<String, Rc<dyn Any>>)>() {
                callback_fn(args);
            } else {
                println!("Callback function {} has a different signature", func_name);
            }
        } else {
            println!("Callback function {} is not registered", func_name);
        }
    }
}

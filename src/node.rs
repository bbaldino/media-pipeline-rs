use crate::packet_info::PacketInfo;
use std::sync::{Arc, Mutex, MutexGuard};

pub trait Node {
    fn process_packet(&mut self, packet_info: PacketInfo);
    fn attach(&mut self, next: Box<dyn Node>);
}

// Workaround for being able to clone any Node while keeping Node object safe (see
// https://stackoverflow.com/a/30353928)
// pub trait CloneNode {
//     fn clone_node(&self) -> Box<dyn Node>;
// }
//
// impl<T> CloneNode for T
// where
//     T: Node + Clone + 'static,
// {
//     fn clone_node(&self) -> Box<dyn Node> {
//         Box::new(self.clone())
//     }
// }
//
// impl Clone for Box<dyn Node> {
//     fn clone(&self) -> Self {
//         self.clone_node()
//     }
// }

/// Helper type that can be used for shared data in nodes
pub struct SharedData<T>(Arc<Mutex<T>>);

// Deriving clone for SharedData doesn't seem to get picked up correctly when trying to derive
// clone for a struct which contains a SharedData, so manually implement it here.
impl<T> Clone for SharedData<T> {
    fn clone(&self) -> Self {
        SharedData(self.0.clone())
    }
}

impl<T> Default for SharedData<T>
where
    T: Default,
{
    fn default() -> Self {
        Self(Arc::new(Mutex::new(T::default())))
    }
}

impl<T> SharedData<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(Mutex::new(value)))
    }

    pub fn get(&self) -> MutexGuard<'_, T> {
        // An interesting note
        // [here](https://github.com/dtolnay/anyhow/issues/81#issuecomment-609171265) on why anyhow
        // doesn't work with this error and why posion errors with mutexes should always panic
        self.0.lock().unwrap()
    }
}

/// Helper type to model an optional next node in the chain
#[derive(Default)]
pub struct NextNode(Option<Box<dyn Node>>);

impl NextNode {
    pub fn process_packet(&mut self, packet_info: PacketInfo) {
        if let Some(ref mut n) = self.0 {
            n.process_packet(packet_info);
        }
    }

    pub fn replace(&mut self, new: Box<dyn Node>) -> Option<Box<dyn Node>> {
        self.0.replace(new)
    }

    pub fn take(&mut self) -> Option<Box<dyn Node>> {
        self.0.take()
    }
}

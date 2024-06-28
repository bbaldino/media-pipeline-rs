use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};

/// Helper type that can be used for shared data in nodes
pub struct SharedData<T>(Arc<RwLock<T>>);

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
        Self(Arc::new(RwLock::new(T::default())))
    }
}

impl<T> SharedData<T> {
    pub fn new(value: T) -> Self {
        Self(Arc::new(RwLock::new(value)))
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        // An interesting note
        // [here](https://github.com/dtolnay/anyhow/issues/81#issuecomment-609171265) on why anyhow
        // doesn't work with this error and why posion errors with mutexes should always panic
        self.0.read().unwrap()
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        self.0.write().unwrap()
    }
}

/// [`LiveStateReader`] is a read-only view of some state that is updated elsewhere.
pub struct LiveStateReader<T>(tokio::sync::watch::Receiver<T>);

impl<T> LiveStateReader<T> {
    // pub fn value(&self) -> &T {
    //     self.0.borrow().deref()
    // }

    pub fn value(&self) -> tokio::sync::watch::Ref<'_, T> {
        self.0.borrow()
    }
}

pub struct LiveStateWriter<T> {
    inner: tokio::sync::watch::Sender<T>,
}

impl<T> LiveStateWriter<T> {
    pub fn new(initial_state: T) -> Self {
        let inner = tokio::sync::watch::Sender::new(initial_state);

        LiveStateWriter { inner }
    }

    pub fn set(&self, new_value: T) {
        // TODO: bubble up return?
        let _ = self.inner.send(new_value);
    }

    pub fn value(&self) -> tokio::sync::watch::Ref<'_, T> {
        self.inner.borrow()
    }

    pub fn modify<F>(&self, modify: F)
    where
        F: FnOnce(&mut T),
    {
        self.inner.send_modify(modify);
    }

    pub fn reader(&self) -> LiveStateReader<T> {
        LiveStateReader(self.inner.subscribe())
    }
}

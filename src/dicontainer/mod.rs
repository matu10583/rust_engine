mod dicontainer_impl {

    pub struct DiContainer {
        // map of typeid to boxed value
        map: std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any + Send + Sync>>,
    }

    impl DiContainer {
        pub fn new() -> Self {
            Self {
                map: std::collections::HashMap::new(),
            }
        }

        pub fn insert<T: 'static + Send + Sync>(&mut self, value: T) {
            self.map
                .insert(std::any::TypeId::of::<T>(), Box::new(value));
        }

        pub fn get<T: 'static + Send + Sync>(&self) -> Option<&T> {
            self.map
                .get(&std::any::TypeId::of::<T>())
                .and_then(|b| b.downcast_ref::<T>())
        }

        pub fn get_mut<T: 'static + Send + Sync>(&mut self) -> Option<&mut T> {
            self.map
                .get_mut(&std::any::TypeId::of::<T>())
                .and_then(|b| b.downcast_mut::<T>())
        }

        pub fn remove<T: 'static + Send + Sync>(&mut self) -> Option<T> {
            self.map
                .remove(&std::any::TypeId::of::<T>())
                .and_then(|b| b.downcast::<T>().ok().map(|b| *b))
        }

        pub fn clear(&mut self) {
            self.map.clear();
        }
    }
}

pub use dicontainer_impl::DiContainer;

// Example usage:
// let mut container = DiContainer::new();

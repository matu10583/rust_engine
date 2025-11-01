#[cfg(feature = "ecs-hecs")]
mod hecs_impl {
    // use super::*;
    use hecs as h;
    pub struct World(h::World);
    #[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
    pub struct Entity(h::Entity);
    pub trait Component: Send + Sync + 'static {}
    pub struct Ref<'a, T: ?Sized>(h::Ref<'a, T>);
    pub struct RefMut<'a, T: ?Sized>(h::RefMut<'a, T>);

    impl<T: Send + Sync + 'static> Component for T {}
    impl World {
        pub fn new() -> Self {
            Self(h::World::new())
        }
        pub fn spawn<T: Component>(&mut self, component: T) -> Entity {
            Entity(self.0.spawn((component,)))
        }

        pub fn despawn(&mut self, entity: Entity) -> bool {
            self.0.despawn(entity.0).is_ok()
        }

        pub fn insert<T: Component>(&mut self, entity: Entity, component: T) -> bool {
            self.0.insert(entity.0, (component,)).is_ok()
        }

        pub fn remove<T: Component>(&mut self, entity: Entity) -> Option<T> {
            self.0.remove_one::<T>(entity.0).ok()
        }

        pub fn get<'a, T: Component>(&self, entity: Entity) -> Option<Ref<'_, T>> {
            self.0.get::<&T>(entity.0).ok().map(Ref)
        }

        pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<RefMut<'_, T>> {
            self.0.get::<&mut T>(entity.0).ok().map(RefMut)
        }

        pub fn query_ref<T: Component>(&self) -> QueryRef<'_, T> {
            QueryRef {
                inner: self.0.query::<&T>(),
            }
        }

        pub fn query_mut<T: Component>(&mut self) -> QueryMut<'_, T> {
            QueryMut {
                inner: self.0.query::<&mut T>(),
            }
        }
    }

    pub struct QueryRef<'w, T: Component> {
        inner: h::QueryBorrow<'w, &'w T>,
    }

    impl<'w, T: Component> QueryRef<'w, T> {
        pub fn iter<'a>(
            &'a mut self,
        ) -> impl Iterator<Item = (Entity, &'a T)> + 'a + use<'a, 'w, T> {
            self.inner.iter().map(|(e, c)| (Entity(e), c))
        }
    }
    pub struct QueryMut<'w, T: Component> {
        inner: h::QueryBorrow<'w, &'w mut T>,
    }
    impl<'w, T: Component> QueryMut<'w, T> {
        pub fn iter(&mut self) -> impl Iterator<Item = (Entity, &'_ mut T)> + '_ + use<'_, 'w, T> {
            self.inner.iter().map(|(e, c)| (Entity(e), c))
        }
    }
}

#[cfg(feature = "ecs-hecs")]
pub use hecs_impl::*;

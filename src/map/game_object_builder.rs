use bevy::math::bounding::Aabb2d;
use bevy::prelude::*;

use super::mapdata::Boundary;

#[derive(Component, Debug)]
pub struct Collider {
    pub aabb: Aabb2d,
}

// extra is a list of functions for spawning components that
// may be optional on a given entity.
pub struct GameObject {
    pub sprite: Sprite,
    pub transform: Transform,
    pub visibility: Visibility,
    pub name: Name,
    pub collider: Option<Collider>,
    // stored components to be applied later.
    pub extra: Vec<Box<dyn FnOnce(&mut EntityCommands) + Send + Sync>>,
}

impl GameObject {

    #[cfg(feature = "client")]
    pub fn new(id: &str, sprite: Sprite, transform: Transform, visibility: Visibility) -> Self {
        Self {
            sprite,
            transform,
            visibility,
            name: Name::new(id.to_string()),
            collider: None,
            extra: vec![],
        }
    }
    #[cfg(feature = "server")]
    pub fn new(id: &str, _unit: (), transform: Transform, _unit2: ()) -> Self {
        Self {
            sprite: Sprite::default(), // unused on server
            transform,
            visibility: Visibility::Hidden,
            name: Name::new(id.to_string()),
            collider: None,
            extra: vec![],
        }
    }
    pub fn with_marker<T: Component>(self) -> Self
    where
        T: Default,
    {
        self.with_component(T::default())
    }

    pub fn with_collider(mut self, collider: Collider) -> Self {
        self.collider = Some(collider);
        self
    }

    pub fn with_component<C: Component>(mut self, component: C) -> Self {
        self.extra.push(Box::new(move |ec| {
            ec.insert(component);
        }));
        self
    }

    pub fn spawn(self, commands: &mut Commands) -> Entity {
        let mut ec = commands.spawn((self.sprite, self.transform, self.visibility, self.name));

        if let Some(collider) = self.collider {
            ec.insert(collider);
        }

        for extra in self.extra {
            extra(&mut ec);
        }

        ec.id()
    }
}

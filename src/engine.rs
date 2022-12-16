use crate::ecs::*;
use crate::{
    components::{Sprite, Transform},
    renderer::*,
    Shape,
};

pub struct Taconite {
    world: World,
}

impl Renderer for Taconite {}

impl EventHandler for Taconite {
    fn update(&mut self) {
        self.world.update();
    }

    fn draw(&self, _canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {}
}

impl Default for Taconite {
    fn default() -> Self {
        let mut taconite = Self {
            world: World::default(),
        };

        taconite.register_component::<Transform>();
        taconite.register_component::<Sprite>();
        taconite.register_component::<Shape>();

        taconite
    }
}

impl Taconite {
    pub fn create_entity(&mut self) -> usize {
        self.world.create_entity()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        self.world.remove_entity(entity_id);
    }

    pub fn register_component<T: 'static + Component>(&mut self) -> &mut World {
        self.world.register_component::<T>();
        &mut self.world
    }

    pub fn add_system<T: 'static + System>(&mut self, system: T) -> &mut World {
        self.world.add_system(system);
        &mut self.world
    }

    pub fn add_component_to_entity<T: 'static + Component>(
        &mut self,
        entity_id: usize,
        component: T,
    ) -> &mut World {
        self.world.add_component_to_entity(entity_id, component);
        &mut self.world
    }

    pub fn update(&mut self) {
        self.world.update();
    }

    pub fn start(&mut self) {
        self.start_window().expect("Failed to start the window.");
    }
}

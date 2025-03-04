mod example;

use godot::{classes::Engine, prelude::*};

use crate::example::*;

struct Extension;

const SINGLETON_NAME: &str = "MySingleton";

#[gdextension]
unsafe impl ExtensionLibrary for Extension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            engine.register_singleton(
                SINGLETON_NAME,
                &MySingleton::new_alloc(),
            );
        }
    }
    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            let mut engine = Engine::singleton();
            if let Some(singleton) = engine.get_singleton(SINGLETON_NAME) {
                engine.unregister_singleton(SINGLETON_NAME);
                singleton.free();
            } else {
                godot_error!("failed to unregister singleton: {SINGLETON_NAME}");
            }
        }
    }
}

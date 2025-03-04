use godot::classes::Label;

use crate::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct MySingleton;

#[godot_api]
impl MySingleton {
    #[func]
    pub fn example_concrete(&self, node: Gd<MyNode>, mut label: Gd<Label>) {
        label.set_text(&format!("test: {:.3}", node.bind().get_example_export()));
    }
}

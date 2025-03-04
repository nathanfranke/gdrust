use crate::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MyNode {
    #[export]
    example_export: f64,
}

#[godot_api]
impl INode for MyNode {
    fn process(&mut self, delta: f64) {
        self.example_export += delta;
    }
}

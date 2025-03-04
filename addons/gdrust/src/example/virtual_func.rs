use crate::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct MyVirtual {
    _base: Base<Node>,
}

#[godot_api]
impl MyVirtual {
    #[func(virtual)]
    fn example_virtual(&self) -> i64 {
        0
    }
}

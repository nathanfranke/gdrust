use crate::*;

#[derive(GodotClass)]
#[class(init)]
pub struct MyRefCounted {
    #[var]
    example_var: Variant,
}

#[godot_api]
impl MyRefCounted {
    #[func]
    fn example_associated(example_var: Variant) -> Gd<Self> {
        Gd::from_object(Self { example_var })
    }
}

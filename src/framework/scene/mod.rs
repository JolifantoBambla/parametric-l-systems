use crate::framework::input::Input;

pub mod transform;

pub trait Update {
    fn update(&mut self, input: &Input);
}

pub struct Scene {

}

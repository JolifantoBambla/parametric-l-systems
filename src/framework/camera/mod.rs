use glam::Mat4;

pub trait Camera {
    fn view(&self) -> Mat4;
    fn inverse_view(&self) -> Mat4 {
        self.view().inverse()
    }
    fn projection(&self) -> Mat4;
    fn inverse_projection(&self) -> Mat4 {
        self.projection().inverse()
    }
    fn resize(&mut self, width: u32, height: u32);
}

pub trait Renderer {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn rect(&self, p0:[f32;2], p1:[f32;2], color:[f32;4]);
    fn line(&self, p0:[f32;2], p1:[f32;2], color:[f32;4]);
}
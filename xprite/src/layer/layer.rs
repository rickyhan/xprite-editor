use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Layer {
    pub name: String,
    pub content: Pixels,
    pub visible: bool,
}

impl Layer {
    pub fn new(name: String) -> Self {
        let content = Pixels::new();
        let visible = true;

        Self {
            name,
            content,
            visible,
        }
    }

    pub fn with_pixels(mut self, content: &Pixels) -> Self {
        self.content = content.to_owned();
        self
    }

    pub fn pixels(&self) -> &Pixels {
        &self.content
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        &mut self.content
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
        info!("toggled {} to: {}", self.name, self.visible);
    }
}

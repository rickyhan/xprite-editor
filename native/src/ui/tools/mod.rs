use crate::prelude::*;

pub mod autoshade_panel;
pub mod colorpicker_panel;
pub mod ellipse_panel;
pub mod eraser_panel;
pub mod line_panel;
pub mod marquee_panel;
pub mod paintbucket_panel;
pub mod pencil_panel;
pub mod rect_panel;
pub mod settings_panel;
pub mod texture_panel;
pub mod vector_panel;

pub fn draw(selected: ToolType, rdr: &mut dyn Renderer, state: &mut State, ui: &Ui) {
    match selected {
        ToolType::Pencil => pencil_panel::draw(state, ui),
        ToolType::Line => line_panel::draw(state, ui),
        ToolType::PaintBucket => paintbucket_panel::draw(state, ui),
        ToolType::Vector => vector_panel::draw(state, ui),
        ToolType::ColorPicker => colorpicker_panel::draw(state, ui),
        ToolType::Eraser => eraser_panel::draw(state, ui),
        ToolType::Rect | ToolType::FilledRect => rect_panel::draw(state, ui),
        ToolType::Ellipse | ToolType::FilledEllipse => ellipse_panel::draw(state, ui),
        ToolType::Texture => texture_panel::draw(rdr, state, ui),
        ToolType::Marquee => marquee_panel::draw(rdr, state, ui),
        ToolType::Settings => settings_panel::draw(rdr, state, ui),
        ToolType::Symmetry => (),
        ToolType::AutoShade => autoshade_panel::draw(rdr, state, ui),
    }
}

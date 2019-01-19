pub mod canvas;
pub mod console;
pub mod inputs;
pub mod layers;
pub mod menu;
pub mod palette;
pub mod preview;
pub mod settings;
pub mod tool_panel;
pub mod toolbar;
pub mod tools;

use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;

/// steps:
/// 1. get dimensions
/// 2. handle mouse and keyboard input, change state
/// 3. update by calling draw method which takes in a renderer
pub fn draw(rdr: &mut ImguiRenderer, state: &mut State, ui: &Ui) -> bool {
    state.xpr.draw().unwrap();
    state.redraw_pixels(rdr).unwrap();
    state.xpr.update().unwrap();
    self::menu::draw_menu(rdr, state, ui);
    self::toolbar::draw_toolbar(state, ui);
    self::canvas::draw_canvas(rdr, state, ui);
    self::settings::draw_settings(rdr, state, ui);
    self::tool_panel::draw_tool_panel(rdr, state, ui);
    self::palette::draw_palette(rdr, state, ui);
    self::palette::draw_color_picker(rdr, state, ui);
    self::layers::draw_layers(rdr, state, ui);
    self::console::draw_console(rdr, state, ui);
    self::preview::draw_preview(rdr, state, ui);
    true
}

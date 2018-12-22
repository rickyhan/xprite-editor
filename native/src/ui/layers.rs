use crate::prelude::*;
use xprite::rendering::Renderer;

pub fn draw_layers(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui
    .window(im_str!("Layers"))
    .position((sz.0 as f32 - RIGHT_SIDE_WIDTH, (sz.1 / 2.) as f32 + 20.), ImGuiCond::Always)
    .size((RIGHT_SIDE_WIDTH, (sz.1 / 2.) as f32), ImGuiCond::Always)
    .movable(true)
    .collapsible(false)
    .resizable(false)
    .build(|| {
        if ui.button(im_str!("+"), (20.,20.)) {
            state.xpr.history.top_mut().add(None);
        }

        ui.popup_modal(im_str!("Rename Layer"))
          .inputs(true)
          .collapsible(false)
          .resizable(false)
          .movable(false)
          .build(|| {
            let name = {
                state.xpr.current_layer().unwrap().name.to_owned()
            };
            let mut im = ImString::new(name);
            ui.with_item_width(100., ||{
                if ui
                .input_text(im_str!(""), &mut im)
                .auto_select_all(true)
                .enter_returns_true(true)
                .build() {
                    let name : &str = im.as_ref();
                    info!("renaming: {}", name);
                    state.xpr.rename_layer(&im.as_ref()).unwrap();
                    ui.close_current_popup();
                }
            });
        });

        let (mut layers, selected_layer) = {
            let layer_manager = state.xpr.history.top_mut();
            if layer_manager.selected_layer_mut().is_none() {return;}
            let selected_layer = layer_manager.selected_layer_mut().unwrap().clone();
            (
                layer_manager.layers.clone(),
                selected_layer
            )
        };

        for (i, layer) in layers.iter_mut().enumerate() {
            {
                let name = layer.name.as_str();
                let is_sel = layer == &selected_layer;
                if ui.selectable(
                    im_str!("{}", name),
                    is_sel,
                    ImGuiSelectableFlags::AllowDoubleClick,
                    (50.,0.)
                ) {
                    if ui.imgui().is_mouse_double_clicked(imgui::ImMouseButton::Left) {
                        info!("double clicked");
                        ui.open_popup(im_str!("Rename Layer"));
                    }
                    state.xpr.switch_layer(i);
                }
            }

            ui.same_line(100.);
            ui.with_id(i as i32, || {
                if ui.checkbox(im_str!(""), &mut layer.visible) {
                    layer.visible = !layer.visible; // undo imgui checkbox mutation
                    state.xpr.toggle_layer_visibility(i).unwrap(); // enter history frame and toggle
                }
            });
            ui.same_line(140.);
            ui.with_id(i as i32, || {
                if ui.button(im_str!("X"), (20.,20.)) {
                    state.xpr.remove_layer(i).unwrap();
                }
            });
        }
    })
}

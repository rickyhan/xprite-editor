use crate::state::State;
use imgui::*;
use xprite::prelude::*;
use xprite::rendering::Renderer;


/// steps:
/// 1. get dimensions
/// 2. handle mouse and keyboard input, change state
/// 3. update by calling draw method which takes in a renderer
pub fn draw(rdr: &Renderer, state: &mut State, ui: &Ui) -> bool {
    main_menu_bar(rdr, state, ui);
    toolbar(state, ui);
    draw_canvas(rdr, state, ui);
    true
}

fn toolbar(state: &mut State, ui: &Ui) {
    ui.window(im_str!("toolbox")).build(|| {
        let tools: Vec<&str> = state.xpr.toolbox.tools.keys().cloned().collect();
        for (_index, name) in tools.iter().enumerate() {
            let is_sel = &state.xpr.toolbox.tool().borrow().get_name() == name;
            if ui.selectable(
                im_str!("{}", name),
                is_sel,
                ImGuiSelectableFlags::empty(),
                (0.,0.)
            ) {
                state.xpr.change_tool(name);
            }
        }
    })
}

fn main_menu_bar(_rdr: &Renderer, state: &mut State, ui: &Ui) {
    ui.main_menu_bar(|| {
        ui.menu(im_str!("File")).build(|| {
            ui.menu_item(im_str!("Load")).shortcut(im_str!("Ctrl+O")).build();
            ui.menu_item(im_str!("Save")).shortcut(im_str!("Ctrl+S")).build();
        });
        ui.menu(im_str!("Edit")).build(|| {
            if ui.menu_item(im_str!("Undo")).shortcut(im_str!("Ctrl+Z")).build() {
                state.xpr.undo();
            }
            if ui.menu_item(im_str!("Redo")).shortcut(im_str!("Ctrl+y")).build() {
                state.xpr.redo();
            }
        });
    })
}

fn draw_canvas(rdr: &Renderer, state: &mut State, ui: &Ui) {
    let sz = ui.frame_size().logical_size;
    ui.window(im_str!("canvas"))
        .position((20.0, 20.0), ImGuiCond::Appearing)
        // .size((700.0, 300.0), ImGuiCond::Appearing)
        .size((sz.0 as f32/2., sz.1 as f32/2.), ImGuiCond::Appearing)
        .resizable(true)
        .build(|| {
            // checkbox for show grid
            ui.checkbox(im_str!("grid"), &mut state.xpr.canvas.show_grid);
            ui.drag_float(im_str!("scale"), &mut state.xpr.canvas.scale)
              .min(1.)
              .max(50.)
              .speed(0.1)
              .build();

            let styles = [
                StyleVar::FramePadding(ImVec2::new(1., 1.)),
                StyleVar::WindowPadding(ImVec2::new(0., 0.)),
            ];
            let colors = [ (ImGuiCol::ChildBg, GREY) ];

            ui.with_style_and_color_vars(&styles, &colors, || {
                ui.child_frame(im_str!("scrolling_region"), (0., 0.,))
                    .show_scrollbar(false)
                    .movable(false)
                    .build(|| {
                        update_viewport(state, ui);
                        state.xpr.render(rdr);
                        // draw_grid(state, ui);
                        bind_input(state, ui);

                    });
            });

        });
}

fn update_viewport(state: &mut State, ui: &Ui) {
    let win_pos = ui.get_cursor_screen_pos();
    state.xpr.canvas.update_pos(win_pos.0, win_pos.1);

    let canvas_sz = ui.get_window_size();
    state.xpr.canvas.update_sz(canvas_sz.0, canvas_sz.1);
}

fn bind_input(state: &mut State, ui: &Ui) {
    use self::InputItem::*;
    use self::InputEvent::*;

    let wheel_delta = ui.imgui().mouse_wheel();
    let (x, y) = ui.imgui().mouse_pos();

    if state.last_mouse_pos.0 != x
    || state.last_mouse_pos.1 != y {
        info!("{} {}", x, y);
        state.xpr.mouse_move(&MouseMove{ x, y });
    }

    let left = ui.imgui().is_mouse_down(ImMouseButton::Left);
    let right = ui.imgui().is_mouse_down(ImMouseButton::Right);

    // middle key for scrolling
    if ui.is_window_hovered() && !ui.is_item_active() &&
        ui.imgui().is_mouse_dragging(ImMouseButton::Middle)
    {
        let d = ui.imgui().mouse_delta();
        state.xpr.canvas.scroll.x += d.0;
        state.xpr.canvas.scroll.y += d.1;
    }

    if ui.is_window_hovered() && !ui.is_item_active()
    {
        state.xpr.canvas.scale += wheel_delta
    }

    // left
    if state.inputs.debounce(InputItem::Left, left) {
        if left {
            trace!("mouse left down");
            state.xpr.event(&MouseDown{ x, y, button: Left });
        } else {
            trace!("mouse left up");
            state.xpr.event(&MouseUp{ x, y });
        }
    }

    // right
    if state.inputs.debounce(InputItem::Right, right) {
        if right {
            let (x, y) = ui.imgui().mouse_pos();
            state.xpr.event(&MouseDown{ x, y, button: Right });
        }
    }

    // ctrl
    let ctrl = ui.imgui().key_ctrl();
    if state.inputs.debounce(InputItem::Ctrl, ctrl) {
        if ctrl {
            trace!("ctrl down");
            state.xpr.event(&KeyDown{ key: Ctrl });
        } else {
            trace!("ctrl up");
            state.xpr.event(&KeyUp{ key: Ctrl });
        }
    }

    // shift
    let shift = ui.imgui().key_shift();
    if state.inputs.debounce(InputItem::Shift, shift) {
        if shift {
            trace!("shift down");
            state.xpr.event(&KeyDown{ key: Shift });
        } else {
            trace!("shift up");
            state.xpr.event(&KeyUp{ key: Shift });
        }
    }

    // alt
    let alt = ui.imgui().key_alt();
    if state.inputs.debounce(InputItem::Alt, alt) {
        if alt {
            trace!("alt down");
            state.xpr.event(&KeyDown{ key: Alt });
        } else {
            trace!("alt up");
            state.xpr.event(&KeyUp{ key: Alt });
        }
    }

    // z
    let key_z = ui.imgui().get_key_index(ImGuiKey::Z);
    let z = ui.imgui().is_key_down(key_z);
    if state.inputs.debounce(InputItem::Z, z) {
        if z {
            trace!("z down");
            if state.inputs.ctrl {
                state.xpr.undo();
                trace!("ctrl+z");
            }
        } else {
            trace!("z up");
        }
    }

    // y
    let key_y = ui.imgui().get_key_index(ImGuiKey::Y);
    let is_y_down = ui.imgui().is_key_down(key_y);
    if state.inputs.debounce(InputItem::Y, is_y_down) {
        if is_y_down {
            trace!("Y down");
            if state.inputs.ctrl {
                state.xpr.redo();
                trace!("ctrl+y");
            }
        } else {
            trace!("y up");
        }
    }

    // for i in 0..512 {
    //     if ui.imgui().is_key_down(i) {
    //         println!("{}", i);
    //     }
    // }

    state.update_mouse_pos(x, y);
}

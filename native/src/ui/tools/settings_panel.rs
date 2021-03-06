use crate::prelude::*;
use crate::state::preview_window::PreviewWindowMode;

pub fn draw(_rdr: &mut dyn Renderer, state: &mut State, ui: &Ui) {
    ui.text(&im_str!("{}", state.xpr().name));

    if ui.button(&im_str!("Rename Document"), [0., 0.]) {
        state.toggle_hotkeys();
        ui.open_popup(&im_str!("rename_doc"));
    }

    ui.popup(&im_str!("rename_doc"), || {
        let mut fname = ImString::with_capacity(100);
        fname.push_str(&state.xpr().name);
        if ui.input_text(&im_str!("Filename"), &mut fname).build() {
            state.xpr_mut().set_name(fname.to_str().to_owned());
        }
        if ui.button(&im_str!("done"), [0., 0.]) {
            state.toggle_hotkeys();
            ui.close_current_popup();
        }
    });

    ui.tree_node(&im_str!("Document")).default_open(true).build(|| {
        let mut aspect = [0; 2];
        aspect[0] = state.xpr_mut().canvas.art_w as i32;
        aspect[1] = state.xpr_mut().canvas.art_h as i32;
        if ui.input_int2(&im_str!("size"), &mut aspect).build() {
            state.xpr_mut().canvas.art_w = aspect[0] as f64;
            state.xpr_mut().canvas.art_h = aspect[1] as f64;
            state.xpr_mut().set_redraw(true);
        }
    });

    ui.tree_node(&im_str!("Preview")).default_open(true).build(|| {
        let modes = PreviewWindowMode::VARIANTS;
        for (_index, mode) in modes.into_iter().enumerate() {
            let is_sel = &state.preview_window_state.mode == mode;
            if Selectable::new(&im_str!("{}", mode.as_str()))
                .selected(is_sel)
                .flags(SelectableFlags::empty())
                .size([0., 0.])
                .build(&ui)
            {
                state.preview_window_state.mode = *mode;
            }
        }
    });

    ui.tree_node(&im_str!("Background Color")).default_open(true).build(|| {
        let mut sel: [f32; 4] = unsafe { state.xpr().canvas.bg.as_rgba().into() };
        let id = im_str!("##{}", "background");
        let misc_flags = {
            let mut f = ColorEditFlags::empty();
            f.set(ColorEditFlags::HDR, true);
            f.set(ColorEditFlags::ALPHA_PREVIEW, true);
            f.set(ColorEditFlags::NO_OPTIONS, false);
            f.set(ColorEditFlags::NO_INPUTS, true);
            f.set(ColorEditFlags::NO_LABEL, true);
            f.set(ColorEditFlags::NO_PICKER, false);
            f
        };
        let b = ColorEdit::new(&id, &mut sel).flags(misc_flags).alpha(false);
        if b.build(&ui) {
            state.xpr_mut().canvas.bg = sel.into();
            state.xpr_mut().set_redraw(true);
        }
    });

    ui.tree_node(&im_str!("Show grid")).default_open(true).build(|| {
        // checkbox for show grid
        ui.checkbox(&im_str!("grid"), &mut state.xpr_mut().canvas.show_grid);
        // ui.text(&im_str!("{}, {}", state.xpr().last_mouse_pos.y, state.xpr().last_mouse_pos.x));
    });
}

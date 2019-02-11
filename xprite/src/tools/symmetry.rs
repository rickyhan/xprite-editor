use crate::tools::*;
use crate::algorithms::symmetry::SymmetryMode;


#[derive(Clone, Default, Debug)]
pub struct Symmetry {
    is_mouse_down: Option<InputItem>,
    pub symms: Vec<SymmetryMode>,
    pub dirty: bool,
}

impl Symmetry {
    pub fn new() -> Self {
        Symmetry {
            is_mouse_down: None,
            symms: vec![],
            dirty: false,
        }
    }

    pub fn add_symmetry(&mut self, symm: SymmetryMode) {
        self.dirty = true;
        self.symms.push(symm);
    }

    pub fn remove_symmetry(&mut self, i: usize) {
        self.dirty = true;
        self.symms.remove(i);
    }


    /// returns reflected stroke
    pub fn process(&self, pixs: &Pixels) -> Pixels {
        if self.symms.is_empty() { return Pixels::new(); }
        let mut ret = Pixels::new();
        self.symms[0].process(pixs, &mut ret);
        for symm in &self.symms[1..] {
            symm.process(&ret.clone(), &mut ret);
            symm.process(pixs, &mut ret);
        };
        ret
    }
}

impl Tool for Symmetry {
    fn cursor(&self) -> Option<Pixels> {
        // let p = self.cursor_pos?;
        // Some(pixels!(p))
        None
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // // set current cursor_pos
        // let point = xpr.canvas.shrink_size(p);
        // let color = xpr.color();
        // if self.is_mouse_down.is_some() {
        //     self.cursor_pos = Some(Pixel { point, color });
        // }
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // let point = xpr.canvas.shrink_size(p);
        // let color = xpr.color();
        // self.cursor_pos = Some(Pixel { point, color });

        // self.is_mouse_down = None;
        // // self.start_pos = None;
        Ok(())
    }

    fn mouse_down(
        &mut self,
        xpr: &Xprite,
        p: Vec2f,
        button: InputItem,
    ) -> Result<(), String> {
        // if InputItem::Left != button {
        //     return Ok(());
        // }
        // self.is_mouse_down = Some(button);
        // let point = xpr.canvas.shrink_size(p);
        // let color = xpr.color();
        // self.start_pos = Some(Pixel { point, color });
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        // if let Some(cursor) = self.cursor() {
        //     xpr.set_cursor(&cursor);
        // }
        // if let Ok(marq) = outline_rect(self.start_pos, self.cursor_pos) {
        //     xpr.add_marquee(&marq);
        // }
        Ok(false)
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if self.dirty {
            xpr.line_buf.clear();
            let (w, h) = xpr.canvas.get_art_dimension();
            for symm in &self.symms {
                xpr.line_buf.extend(&symm.get_line(w, h));
            }
            self.dirty = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn set(
        &mut self,
        _xpr: &Xprite,
        option: &str,
        value: &str,
    ) -> Result<(), String> {
        match option {
            "ctrl" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "shift" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "alt" => {
                info!("alt pressed (unimplemented)");
            }
            _ => (),
        }
        Ok(())
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_multistep_symmetry() {
        // this test is functionally equivalent to the prevoius test
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red()),
            pixel!(1,0,Color::red())
        );
        let mut symm = Symmetry::new();
        symm.push(SymmetryMode::Horizontal(2.));
        symm.push(SymmetryMode::Vertical(1.));
        let ret = symm.process(&pixs);
        assert_eq!(ret, pixels!(
            pixel!(0,1,Color::red()),
            pixel!(1,1,Color::red()),
            pixel!(2,0,Color::red()),
            pixel!(3,0,Color::red()),
            pixel!(2,1,Color::red()),
            pixel!(3,1,Color::red())
        ));
    }
}

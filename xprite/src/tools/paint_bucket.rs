use crate::prelude::*;
use crate::algorithms;

#[derive(Clone, Default, Debug)]
pub struct PaintBucket { }

impl PaintBucket {
    pub fn new() -> Self {
        PaintBucket { }
    }

    pub fn floodfill(&self, xpr: &mut Xprite, p: Vec2D, bg_color: Option<Color>) -> Result<Pixels, String> {
        let color = xpr.color();
        let w = xpr.canvas.art_w;
        let h = xpr.canvas.art_h;
        let current_layer = xpr.current_layer_mut().unwrap();
        let pixs = &current_layer.content;
        let buffer = algorithms::floodfill::floodfill(w, h, pixs, p, bg_color, color);
        // info!{"{:#?}", buffer};
        Ok(buffer)
    }

}

impl Tool for PaintBucket {

    fn tool_type(&self) -> ToolType {
        ToolType::PaintBucket
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        xpr.set_cursor(&(Pixel {point, color}).into());
        self.draw(xpr)?;
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let (w, h) = (xpr.canvas.art_w, xpr.canvas.art_h);
        if oob(point.x, point.y, w, h) { return Ok(()); }
        let bg_color = xpr.current_layer().unwrap().get_color(point);
        let buffer = self.floodfill(xpr, point, bg_color)?;

        xpr.history.enter()?;
        xpr.history.top_mut()
            .selected_layer_mut().unwrap()
            .content
            .extend(&buffer);
        Ok(())
    }

    #[allow(unused)]
    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, _button: InputItem) -> Result<(), String> {
        // let point = xpr.canvas.shrink_size(&p);
        // let bg_color = xpr.current_layer().borrow().get_color(point);
        // let buffer = self.floodfill(xpr, point, bg_color)?;
        // if buffer.len() > MAX_CURSOR_NUM {
        //     let w = xpr.canvas.art_w;
        //     let h = xpr.canvas.art_h;
        //     xpr.set_cursor(&algorithms::perimeter::find_perimeter(w as usize, h as usize, &buffer));
        // } else {
        //     xpr.set_cursor(&buffer);
        // }

        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        // noop
        Ok(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, _value: &str) -> Result<(), String> {
        match option {
            _ => (), // noop
        }
        Ok(())
    }
}

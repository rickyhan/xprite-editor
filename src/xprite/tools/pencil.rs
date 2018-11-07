use xprite::prelude::*;
use xprite::lib::algorithms::sorter::sort_path;
use xprite::lib::algorithms::pixel_perfect::pixel_perfect;
use stdweb::web::event::MouseButton;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PencilMode {
    /// just run pixel perfect - nothing else
    PixelPerfect,
    /// convert to vector and sort everything by slope
    SimplifyAndSortWhole,
    /// convert to vector and sort each segment
    SimplifyAndSortByParts,
    /// sort each monotonic segment
    SortedMonotonic,
}

pub struct Pencil {
    is_mouse_down: Option<MouseButton>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    tolerence: f32,
    mode: PencilMode,
}
impl Pencil {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor = None;
        let cursor_pos = None;
        let brush = Brush::pixel();
        let current_polyline = Polyline::new();

        Self {
            is_mouse_down,
            current_polyline,
            cursor,
            cursor_pos,
            brush,
            tolerence: 2.,
            mode: PencilMode::PixelPerfect,
        }
    }

    pub fn draw_pixels(&mut self, xpr: &mut Xprite, pixs: &[Pixel]) {
        for &Pixel{point, ..} in pixs.iter() {
            let color = ColorOption::Set(Color::new(200, 200, 200));
            xpr.draw_pixel(point.x, point.y, color);
        }
    }

    pub fn draw_polyline(&mut self, xpr: &mut Xprite, polyline: &Polyline, sort_parts: bool, sort_whole: bool) {

        let path = polyline.interp();
        for &Pixel{point, ..} in path.rasterize(xpr, sort_parts, sort_whole).unwrap().iter() {
            let color = ColorOption::Set(Color::new(200, 200, 200));
            xpr.draw_pixel(point.x, point.y, color);
        }

        // // plot anchors
        // for &p in polyline.pos.iter() {
        //     let Point2D{x, y} = xpr.canvas.client_to_grid(p.as_i32());
        //     let color = ColorOption::Set(Color::blue());
        //     xpr.draw_pixel(x, y, color);
        // }

        // // plot control points
        // for seg in &path.segments {
        //     let CubicBezierSegment { ctrl1, ctrl2, .. } = seg;
        //     for point in vec![ctrl1, ctrl2] {
        //         let Point2D{x, y} = xpr.canvas.client_to_grid(point.as_i32());
        //         xpr.draw_pixel(x, y, ColorOption::Set(Color::red()));
        //     }
        // }

    }

    fn draw_cursor(&self, xpr: &Xprite) {
        if self.cursor.is_none() { return; }

        let cursor = self.cursor.clone().unwrap();
        for &pos in cursor.iter() {
            xpr.canvas.draw(
                pos.point.x,
                pos.point.y,
                &Color::red().to_string()
            );
        }
    }

}

impl Tool for Pencil {

    fn get_name(&self) -> &'static str {
        "pencil"
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D<i32>) {
        let pixels = xpr.canvas.to_pixels(p, &self.brush, xpr.color());
        self.cursor = pixels.clone();
        let point = xpr.canvas.client_to_grid(p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel{point, color});

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            self.draw(xpr);
            return;
        }

        self.current_polyline.push(p.as_f32());

        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Left {
            xpr.history.undo();
            xpr.history.enter();
            let line_pixs = self.current_polyline.connect_with_line(&xpr);
            let perfect = pixel_perfect(&line_pixs);
            xpr.add_pixels(&Pixels::from_slice(&perfect));
        } else if button == MouseButton::Right {
            // xpr.remove_pixels(&pixels.unwrap());
        }
        self.draw(xpr);
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D<i32>, button: MouseButton) {
        self.is_mouse_down = Some(button);
        xpr.history.enter();

        self.current_polyline.push(p.as_f32());

        let pixels = xpr.canvas.to_pixels(p, &self.brush, xpr.color());
        if let Some(pixels) = pixels {
            if button == MouseButton::Left {
                xpr.add_pixels(&pixels);
            } else {
                // xpr.remove_pixels(&pixels);
            }
        }
        self.draw(xpr);
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, _p: Point2D<i32>) -> Option<()> {
        if self.is_mouse_down.is_none() {return Some(()); }
        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Right { return Some(()); }

        xpr.history.undo();
        xpr.history.enter();
        use self::PencilMode::*;
        match self.mode {
            SimplifyAndSortByParts => {
                // simply curve then rasterize
                let simple = self.current_polyline.reumann_witkam(self.tolerence)?;
                self.draw_polyline(xpr, &simple, true, false);
            }
            SimplifyAndSortWhole => {
                // simply curve then rasterize
                let simple = self.current_polyline.reumann_witkam(self.tolerence)?;
                self.draw_polyline(xpr, &simple, false, true);
            }
            PixelPerfect => {
                let mut points = self.current_polyline.connect_with_line(xpr);
                let perfect = &pixel_perfect(&points);
                self.draw_pixels(xpr, &perfect);
            }
            SortedMonotonic => {
                let mut points = self.current_polyline.connect_with_line(xpr);
                let mut perfect = pixel_perfect(&points);
                let sorted = sort_path(&mut perfect)?;
                self.draw_pixels(xpr, &sorted);
            }
        }

        self.current_polyline.clear();
        self.is_mouse_down = None;

        self.draw(xpr);
        Some(())
    }

    fn draw(&self, xpr: &Xprite) {
        xpr.canvas.clear_all();
        for &Pixel{point, color} in xpr.pixels().iter() {
            let color = match color {
                ColorOption::Set(c) => c,
                ColorOption::Unset => xpr.color(),
            }.to_string();
            xpr.canvas.draw(point.x, point.y, &color);
        }
        self.draw_cursor(xpr);
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, value: &str) {
        match option {
            "mode" => {
                use self::PencilMode::*;
                match value {
                    "monotonic" => self.mode = SortedMonotonic,
                    "pp"        => self.mode = PixelPerfect,
                    "whole"     => self.mode = SimplifyAndSortWhole,
                    "parts"     => self.mode = SimplifyAndSortByParts,
                    _ => console!(error, "malformed value: ", value),
                };
            }
            "tolerence" => {
                if let Ok(val) = value.parse() {
                    self.tolerence = val;
                } else {
                    console!(error, "cannot parse val:", value);
                }
            }
            "brush" => {
                match value {
                    "cross" => self.brush = Brush::cross(),
                    "pixel" => self.brush = Brush::pixel(),
                    _ => console!(error, "malformed value: ", value),
                }
            }
            _ => (),
        }
    }
}

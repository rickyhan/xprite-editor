use crate::prelude::*;
use crate::algorithms::sorter::sort_path;
use crate::algorithms::pixel_perfect::pixel_perfect;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PencilMode {
    /// raw - noop
    Raw,
    /// pixel perfect - nothing else
    PixelPerfect,
    /// sort each monotonic segment
    SortedMonotonic,
}

impl PencilMode {
    pub fn as_str(&self) -> &str {
        match self {
            PencilMode::Raw => "Raw",
            PencilMode::PixelPerfect => "Pixel Perfect",
            PencilMode::SortedMonotonic => "Sorted Monotonic",
        }
    }

    pub const VARIANTS: [PencilMode; 3] = [
        PencilMode::Raw,
        PencilMode::PixelPerfect,
        PencilMode::SortedMonotonic,
    ];
}

impl FromStr for PencilMode {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Raw" => Ok(PencilMode::Raw),
            "Pixel Perfect" => Ok(PencilMode::PixelPerfect),
            "Sorted Monotonic" => Ok(PencilMode::SortedMonotonic),
            _ => Err(()),
        }
    }
}

pub struct Pencil {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    pub mode: PencilMode,
    pub brush_type: BrushType,
    buffer: Pixels,
    moved: bool,
}

impl Default for Pencil {
    fn default() -> Self {
        Self::new()
    }
}


impl Pencil {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor = None;
        let cursor_pos = None;
        let brush_type = BrushType::Pixel;
        let brush = Brush::pixel();
        let buffer = Pixels::new();
        let current_polyline = Polyline::new();

        Self {
            is_mouse_down,
            current_polyline,
            cursor,
            cursor_pos,
            brush,
            brush_type,
            mode: PencilMode::PixelPerfect,
            buffer,
            moved: false,
        }
    }

    pub fn draw_polyline(&mut self, xpr: &mut Xprite, polyline: &Polyline) -> Pixels {
        let path = polyline.interp();
        let mut rasterized = path.rasterize(xpr).unwrap();
        rasterized.set_color(&xpr.color());
        // self.buffer.extend(&pixels);
        rasterized
    }

    fn set_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        self.cursor.as_ref().map(|cursor|{
            xpr.set_cursor(cursor);
        })
    }
}

impl Tool for Pencil {

    fn tool_type(&self) -> ToolType {
        ToolType::Pencil
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        self.cursor = pixels.clone();
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel{point, color});

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            return self.draw(xpr);
        }
        self.moved = true;

        self.current_polyline.push(p);

        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Left {
            self.buffer.clear();
            let line_pixs = self.current_polyline.connect_with_line(&xpr)?;
            let pixs = if self.mode != PencilMode::Raw {
                let perfect = pixel_perfect(&line_pixs);
                Pixels::from_slice(&perfect)
            } else {
                Pixels::from_slice(&line_pixs)
            };
            let mut pixs = self.brush.follow_stroke(&pixs).unwrap();
            pixs.with_color(&xpr.color());
            self.buffer.extend(&pixs);
        } else if button == InputItem::Right {
            // xpr.remove_pixels(&pixels.unwrap());
        }

        self.draw(xpr)
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, button: InputItem) -> Result<(), String>{
        self.is_mouse_down = Some(button);

        self.current_polyline.push(p);
        self.buffer.clear();
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        // TODO:
        if let Some(pixels) = pixels {
            if button == InputItem::Left {
                self.buffer.extend(&pixels);
            } else {
                // xpr.remove_pixels(&pixels);
            }
        }
        self.draw(xpr)
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, _p: Vec2D) -> Result<(), String> {
        if self.is_mouse_down.is_none() {return Ok(()); }
        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right { return Ok(()); }

        use self::PencilMode::*;
        match self.mode {
            Raw => {
                // noop
            }
            PixelPerfect => {
                // if mousedown w/o move
                if !self.moved {
                    info!("mousedown w/o moving");
                    // noop
                } else {
                    self.buffer.clear();
                    let points = self.current_polyline.connect_with_line(xpr)?;
                    let perfect = &pixel_perfect(&points);
                    let pixs = Pixels::from_slice(&perfect);
                    let path = self.brush.follow_stroke(&pixs).unwrap();
                    self.buffer.extend(&path);
                }
            }
            SortedMonotonic => {
                self.buffer.clear();
                let points = self.current_polyline.connect_with_line(xpr)?;
                let mut perfect = pixel_perfect(&points);
                let sorted = sort_path(&mut perfect)?;
                let pixs = Pixels::from_slice(&sorted);
                let path = self.brush.follow_stroke(&pixs).unwrap();
                self.buffer.extend(&path);
            }
        }

        self.buffer.set_color(&xpr.color());

        xpr.history.enter()?;
        xpr.current_layer_mut().ok_or("Layer doesn't exist.".to_owned())
            .content
            .extend(&self.buffer);

        self.current_polyline.clear();
        self.buffer.clear();
        self.is_mouse_down = None;
        self.moved = false;

        self.draw(xpr)?;
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr);
        self.buffer.set_color(&xpr.color());
        xpr.add_pixels(&self.buffer);

        Ok(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "mode" => {
                use self::PencilMode::*;
                match PencilMode::from_str(value) {
                    Ok(Raw)             => self.mode = Raw,
                    Ok(SortedMonotonic) => self.mode = SortedMonotonic,
                    Ok(PixelPerfect)    => self.mode = PixelPerfect,
                    _ => (),
                };
            }
            "brush" => {
                match value {
                    "+" => self.brush = Brush::cross(),
                    "." => self.brush = Brush::pixel(),
                    _ => error!("malformed value: {}", value),
                }
            }
            _ => (),
        }
        Ok(())
    }
}

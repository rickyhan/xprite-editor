use crate::prelude::*;
use crate::algorithms::{
    pixel_perfect::pixel_perfect,
    sorter::sort_path,
    connected_components::connected_components,
    perimeter::find_perimeter,
};
use std::ops::{Index, Sub};
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter, Error};
use indexmap::{IndexSet, set::Iter};
use img::GenericImageView;

#[cfg(feature = "python-scripting")]
#[pyclass]
#[derive(Copy, Clone, Eq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Pixel {
    pub point: Vec2D,
    pub color: Color,
}

#[cfg(feature = "python-scripting")]
use pyo3::class::basic::PyObjectProtocol;
#[cfg(feature = "python-scripting")]
#[pyproto]
impl PyObjectProtocol for Pixel {
    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!("Pixel<({:?}), ({:?})>", self.point, self.color))
    }
}

#[cfg(feature = "python-scripting")]
#[pymethods]
impl Pixel {
    #[new]
    fn __new__(obj: &PyRawObject, point: Vec2D, color: Color) -> PyResult<()> {
        obj.init(|_| {
            Pixel { point, color }
        })
    }
}

impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state)
    }
}

impl Ord for Pixel {
    fn cmp(&self, other: &Pixel) -> Ordering {
        self.point.cmp(&other.point)
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        self.point == other.point
    }
}

impl Pixel {
    pub fn with_color(&self, col: Color) -> Self {
        let mut self_ = self.clone();
        self_.color = col;
        self_
    }
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "@({},{})", self.point.x, self.point.y)
    }
}

macro_rules! pixel {
    ($i:expr, $j: expr, $k: expr) => {
        Pixel {
            point: Vec2D::new(($i) as f32, ($j) as f32),
            color: $k,
        }
    };
}


macro_rules! pixels {
    ($($i: expr),*) => {
        {
            let mut pixs = Pixels::new();
            $(
                pixs.push($i);
            )*
            pixs
        }
    };
}



#[derive(Clone, Eq, Serialize, Deserialize, Default)]
pub struct Pixels(pub IndexSet<Pixel>);

impl Hash for Pixels {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.0.iter() {
            i.point.hash(state);
            i.color.hash(state);
        }
    }
}

impl PartialEq for Pixels {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Pixels {

    pub fn new() -> Self {
        Pixels(IndexSet::new())
    }

    pub fn from_slice(slice: &[Pixel]) -> Self {
        let mut set = IndexSet::new();
        for p in slice.iter() {
            if set.contains(p) {
                continue;
            }
            set.insert(*p);
        }
        Pixels(set)
    }

    pub fn extend(&mut self, other: &Pixels) {
        for i in other.0.iter() {
            self.0.replace(*i);
        }
    }

    pub fn sub(&mut self, other: &Pixels) {
        self.0 = self.0.sub(&other.0)
    }

    pub fn intersection(&mut self, other: &Pixels) -> Pixels {
        let common: Vec<_> = self.0.intersection(&other.0).cloned().collect();
        Pixels::from_slice(&common)
    }

    pub fn push(&mut self, px: Pixel) {
        if !self.0.contains(&px) {
            self.0.insert(px);
        }
    }

    pub fn contains(&mut self, px: &Pixel) -> bool {
        self.0.contains(px)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> Iter<Pixel> {
        self.0.iter()
    }

    pub fn set_color(&mut self, color: &Color) {
        let color = *color;
        self.0 = self.0
            .iter()
            .map(|Pixel {point,..}| { Pixel{ point: *point, color } })
            .collect();
    }

    pub fn with_color(&mut self, color: &Color) -> &Self {
        let color = *color;
        self.0 = self.0
            .iter()
            .map(|Pixel {point,..}| { Pixel{ point: *point, color } })
            .collect();
        self
    }

    pub fn pixel_perfect(&mut self) {
        *self = pixel_perfect(self);
    }

    pub fn monotonic_sort(&mut self) {
        *self = sort_path(self).unwrap();
    }

    pub fn connected_components(&self, w:usize, h: usize) -> Vec<Pixels> {
        connected_components(self, w, h)
    }

    pub fn perimeter(&self, w:usize, h: usize) -> Pixels {
        find_perimeter(w, h, self)
    }


}

impl Index<usize> for Pixels {
    type Output = Pixel;
    fn index(&self, idx: usize) -> &Pixel {
        self.0.get_index(idx).unwrap()
    }
}


impl Debug for Pixels {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.0.len() {
            0 => write!(f, "Pixels[0](empty)"),
            1 => write!(f, "Pixels[1]([{:?}])", self.0.iter().next().unwrap()),
            // _ => write!(f, "Pixels[{}]([{:?}..{:?}])", self.0.len(), self.0[0], self.0.last().unwrap()),
            _ => write!(f, "Pixels[{}]([{:?}])", self.0.len(), self.0),
        }
    }
}

impl From<Pixel> for Pixels {
    fn from(p: Pixel) -> Pixels {
        let mut pix = Pixels::new();
        pix.push(p);
        pix
    }
}

impl From<img::DynamicImage> for Pixels {
    fn from(im: img::DynamicImage) -> Pixels {
        let mut pixs = Pixels::new();
        for p in im.pixels() {
            pixs.push(pixel!(p.0, p.1, p.2.into()))
        }
        pixs
    }
}


impl Pixels {
    pub fn as_bool_mat(&self, w: usize, h: usize) -> Vec<Vec<bool>> {
        let mut arr = vec![vec![false;w];h];
        for p in self.0.iter() {
            let Pixel{point, ..} = p;
            let Vec2D {x, y} = point;
            arr[*x as usize][*y as usize] = true;
        }
        arr
    }

    pub fn as_mat(&self, w: usize, h: usize) -> Vec<Vec<Option<Pixel>>> {
        let mut arr = vec![vec![None;w];h];
        for p in self.0.iter() {
            let Pixel{point, ..} = p;
            let Vec2D {x, y} = point;
            if oob(*x, *y, w as f32, h as f32) { continue; }
            arr[*x as usize][*y as usize] = Some(p.clone());
        }
        arr
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_image(&self, w: f32, h: f32, origin: (f32, f32)) -> img::DynamicImage {
        let mut rdr = ImageRenderer::new(w, h);
        for pix in &self.0 {
            let Pixel{point:Vec2D{x,y}, color} = pix;
            if oob(*x - origin.0, *y - origin.1, w as f32, h as f32) { continue; }
            rdr.rect([*x - origin.0,*y - origin.1], [0.,0.,], (*color).into(), true);
        }
        rdr.render();
        rdr.image
    }
}



mod tests {

    #[test]
    fn test_extend() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0.,0., Color::blue()),
            pixel!(0.,1., Color::blue())]
        );
        let v2 = Pixels::from_slice(&vec![
            pixel!(0.,1., Color::blue())
        ]);
        v1.extend(&v2);
        let mut expected = IndexSet::new();
        expected.insert(pixel!(0.,0., Color::blue()));
        expected.insert(pixel!(0.,1., Color::blue()));
        assert_eq!(expected, v1.0);
    }

    #[test]
    fn test_extend_dup() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0.,0., Color::red()),
            pixel!(0.,1., Color::red())]
        );
        let v2 = Pixels::from_slice(&vec![
            pixel!(0.,1., Color::blue())
        ]);
        v1.extend(&v2);
        let mut expected = IndexSet::new();
        expected.insert(pixel!(0.,0., Color::red()));
        expected.insert(pixel!(0.,1., Color::blue()));
        assert_eq!(expected, v1.0);
    }

    #[test]
    fn test_bool_mat() {
        use super::*;
        let pixs = pixels!{
            pixel!(0,0,Color::red()),
            pixel!(0,1,Color::red()),
            // pixel!(1,0,Color::red()),
            pixel!(1,1,Color::red())
        };

        assert_eq!(
            pixs.as_bool_mat(2,2),
            vec![
                vec![true, true],
                vec![false, true]
            ]
        );
    }

    #[test]
    fn test_sub() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0.,0., Color::red()),
            pixel!(0.,1., Color::red())
        ]);
        v1.sub(&Pixels::from_slice(&vec![
            pixel!(0.,1., Color::blue())
        ]));
        assert_eq!(Pixels::from_slice(&vec![pixel!(0.,0., Color::red())]), v1);
    }


    #[test]
    fn test_intersection() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0.,0., Color::red()),
            pixel!(0.,1., Color::red())
        ]);
        let intersection = v1.intersection(&Pixels::from_slice(&vec![
            pixel!(0.,1., Color::blue())
        ]));
        assert_eq!(Pixels::from_slice(&vec![
            pixel!(0.,1., Color::red())
        ]), intersection);
    }
}
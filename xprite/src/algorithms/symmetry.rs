use crate::prelude::*;

#[derive(Clone, Debug)]
pub enum SymmetryMode {
    /// -
    Horizontal(f64),
    /// |
    Vertical(f64),
    /// / parameterized by the y intercept
    AntiDiagonal(f64),
    /// \
    Diagonal(f64),
    /// (horizontal, vertical)
    Quad(f64, f64),
}

impl SymmetryMode {
    pub fn as_str(&self) -> &str {
        match self {
            SymmetryMode::Horizontal(_) => "Horizontal",
            SymmetryMode::Vertical(_) => "Vertical",
            SymmetryMode::Diagonal(_) => "Diagonal",
            SymmetryMode::AntiDiagonal(_) => "AntiDiagonal",
            SymmetryMode::Quad(_,_) => "Quad",
        }
    }

    pub const VARIANTS: [SymmetryMode; 5] =
        [
            SymmetryMode::Horizontal(0.),
            SymmetryMode::Vertical(0.),
            SymmetryMode::Quad(0., 0.),
            SymmetryMode::Diagonal(0.),
            SymmetryMode::AntiDiagonal(0.),
        ];

    pub fn process(&self, pixs: &Pixels, ret: &mut Pixels) {
        match self {
            SymmetryMode::Horizontal(m) => {
                let adjust = 1.; // ...
                for Pixel{point: Vec2f{x,y}, color} in pixs.iter() {
                    ret.push(pixel_xy!(*x, m - (y - m + adjust), *color));
                }
            }
            SymmetryMode::Vertical(m) => {
                let adjust = 1.; // ...
                for Pixel{point: Vec2f{x,y}, color} in pixs.iter() {
                    ret.push(pixel_xy!(m - (x - m + adjust), *y, *color));
                }
            }
            SymmetryMode::Quad(m1, m2) => {
                SymmetryMode::Horizontal(*m1).process(pixs, ret);
                SymmetryMode::Vertical(*m2).process(&ret.clone(), ret); // ...
                SymmetryMode::Vertical(*m2).process(pixs, ret);
            }
            SymmetryMode::AntiDiagonal(y) => {
                let pivot = vec2f!(0, *y);
                SymmetryMode::Vertical(*y).process(pixs, ret);
                *ret = ret.rotate(pivot, -PI/2.);
            }
            SymmetryMode::Diagonal(y) => {
                let pivot = vec2f!(0, *y);
                SymmetryMode::Horizontal(*y).process(pixs, ret);
                *ret = ret.rotate(pivot, -PI/2.);
            }
        }
    }

    pub fn get_line(&self, w: f64, h: f64) -> Vec<Rect> {
        match *self {
            SymmetryMode::Horizontal(m) => {
                vec![Rect(vec2f!(m, 0), vec2f!(m, w))]
            }
            SymmetryMode::Vertical(m) => {
                vec![Rect(vec2f!(0, m), vec2f!(h, m))]
            }
            SymmetryMode::Quad(m, n) => {
                vec![
                    Rect(vec2f!(m, 0), vec2f!(m, w)),
                    Rect(vec2f!(0, n), vec2f!(h, n))
                ]
            }
            SymmetryMode::Diagonal(m) => {
                let p1 = if h <= w {
                    vec2f!(m+w, w)
                } else {
                    vec2f!(h, h-m)
                };
                vec![Rect(vec2f!(m, 0), p1)]
            }
            SymmetryMode::AntiDiagonal(m) => {
                let p1 = if w <= h {
                    vec2f!(m-w, w)
                } else {
                    vec2f!(0, m)
                };
                vec![Rect(vec2f!(m, 0), p1)]
            }
        }
    }
}

impl Default for SymmetryMode {
    fn default() -> Self {
        SymmetryMode::Vertical(0.)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_process_symmetry_vertial() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red()),
            pixel!(1,0,Color::red()),
            pixel!(2,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Vertical(1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(0,1,Color::red()),
            pixel!(1,1,Color::red()),
            pixel!(2,1,Color::red())
        ));
    }

    #[test]
    fn test_process_symmetry_horizontal() {
        use super::*;
        let pixs = pixels!(
            pixel_xy!(0,0,Color::red()),
            pixel_xy!(1,0,Color::red()),
            pixel_xy!(2,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Horizontal(1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel_xy!(0,1,Color::red()),
            pixel_xy!(1,1,Color::red()),
            pixel_xy!(2,1,Color::red())
        ));
    }

    #[test]
    fn test_process_symmetry_quad() {
        use super::*;
        let pixs = pixels!(
            pixel_xy!(0,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Quad(1., 1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel_xy!(0,1,Color::red()),
            pixel_xy!(1,1,Color::red()),
            pixel_xy!(1,0,Color::red())
        ));
    }

    #[test]
    fn test_process_symmetry_quad2() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red()),
            pixel!(1,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Quad(2., 1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(0,1,Color::red()),
            pixel!(1,1,Color::red()),
            pixel!(2,0,Color::red()),
            pixel!(3,0,Color::red()),
            pixel!(2,1,Color::red()),
            pixel!(3,1,Color::red())
        ));
    }


    #[test]
    fn test_antidiagonal_symmetry() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::AntiDiagonal(2.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(1,1,Color::red())
        ));
    }

    #[test]
    fn test_antidiagonal_symmetry2() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::AntiDiagonal(3.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(2,2,Color::red())
        ));
    }

    #[test]
    fn test_antidiagonal_symmetry3() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,0,Color::red()),
            pixel!(1,0,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::AntiDiagonal(3.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(2,1,Color::red()),
            pixel!(2,2,Color::red())
        ));
    }

    #[test]
    fn test_diagonal_symmetry() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,1,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Diagonal(0.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(1,0,Color::red())
        ));
    }

    #[test]
    fn test_diagonal_symmetry1() {
        use super::*;
        let pixs = pixels!(
            pixel!(0,1,Color::red())
        );
        let mut ret = Pixels::new();
        let symm = SymmetryMode::Diagonal(1.);
        symm.process(&pixs, &mut ret);
        assert_eq!(ret, pixels!(
            pixel!(0,-1,Color::red())
        ));
    }

}
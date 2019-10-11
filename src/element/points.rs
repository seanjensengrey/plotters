use super::*;
use super::{Drawable, PointCollection};
use crate::drawing::backend::{BackendCoord, DrawingBackend, DrawingErrorKind};
use crate::style::ShapeStyle;

/// The element that used to describe a point
pub trait PointElement<Coord> {
    fn make_point(pos: Coord, size: u32, style: ShapeStyle) -> Self;
}

/// Describe a cross
pub struct Cross<Coord> {
    center: Coord,
    size: u32,
    style: ShapeStyle,
}

impl<Coord> Cross<Coord> {
    pub fn new<T: Into<ShapeStyle>>(coord: Coord, size: u32, style: T) -> Self {
        Self {
            center: coord,
            size,
            style: style.into(),
        }
    }
}

impl<'a, Coord: 'a> PointCollection<'a, Coord> for &'a Cross<Coord> {
    type Borrow = &'a Coord;
    type IntoIter = std::iter::Once<&'a Coord>;
    fn point_iter(self) -> std::iter::Once<&'a Coord> {
        std::iter::once(&self.center)
    }
}

impl<Coord, DB: DrawingBackend> Drawable<DB> for Cross<Coord> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        if let Some((x, y)) = points.next() {
            let size = self.size as i32;
            let (x0, y0) = (x - size, y - size);
            let (x1, y1) = (x + size, y + size);
            backend.draw_line((x0, y0), (x1, y1), &self.style.color)?;
            backend.draw_line((x0, y1), (x1, y0), &self.style.color)?;
        }
        Ok(())
    }
}

/// Describe a triangle marker
pub struct TriangleMarker<Coord> {
    center: Coord,
    size: u32,
    style: ShapeStyle,
}

impl<Coord> TriangleMarker<Coord> {
    pub fn new<T: Into<ShapeStyle>>(coord: Coord, size: u32, style: T) -> Self {
        Self {
            center: coord,
            size,
            style: style.into(),
        }
    }
}

impl<'a, Coord: 'a> PointCollection<'a, Coord> for &'a TriangleMarker<Coord> {
    type Borrow = &'a Coord;
    type IntoIter = std::iter::Once<&'a Coord>;
    fn point_iter(self) -> std::iter::Once<&'a Coord> {
        std::iter::once(&self.center)
    }
}

impl<Coord, DB: DrawingBackend> Drawable<DB> for TriangleMarker<Coord> {
    fn draw<I: Iterator<Item = BackendCoord>>(
        &self,
        mut points: I,
        backend: &mut DB,
        _: (u32, u32),
    ) -> Result<(), DrawingErrorKind<DB::ErrorType>> {
        if let Some((x, y)) = points.next() {
            let points = [-90, -210, -330]
                .iter()
                .map(|deg| f64::from(*deg) * std::f64::consts::PI / 180.0)
                .map(|rad| {
                    (
                        (rad.cos() * f64::from(self.size) + f64::from(x)).ceil() as i32,
                        (rad.sin() * f64::from(self.size) + f64::from(y)).ceil() as i32,
                    )
                });
            backend.fill_polygon(points, &self.style.color)?;
        }
        Ok(())
    }
}

impl<Coord> PointElement<Coord> for Cross<Coord> {
    fn make_point(pos: Coord, size: u32, style: ShapeStyle) -> Self {
        Self::new(pos, size, style)
    }
}

impl<Coord> PointElement<Coord> for TriangleMarker<Coord> {
    fn make_point(pos: Coord, size: u32, style: ShapeStyle) -> Self {
        Self::new(pos, size, style)
    }
}

impl<Coord> PointElement<Coord> for Circle<Coord> {
    fn make_point(pos: Coord, size: u32, style: ShapeStyle) -> Self {
        Self::new(pos, size, style)
    }
}

impl<Coord> PointElement<Coord> for Pixel<Coord> {
    fn make_point(pos: Coord, _: u32, style: ShapeStyle) -> Self {
        Self::new(pos, style)
    }
}

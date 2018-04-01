use ::*;

// A rect with sides parralel to x and y axis.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Rect<T: Num + Copy> {
    pub bottom_left: Vec2<T>,
    pub top_right: Vec2<T>,
}

impl<T: Num + Copy + PartialOrd> Rect<T> {
    /// Construct a rect from corner points.
    pub fn from_corners(p1: Vec2<T>, p2: Vec2<T>) -> Self {
        let (min_x, max_x) = min_max(p1.x, p2.x);
        let (min_y, max_y) = min_max(p1.y, p2.y);
        Self {
            bottom_left: vec2(min_x, min_y),
            top_right: vec2(max_x, max_y),
        }
    }

    /// Get rect's width.
    pub fn width(&self) -> T {
        self.top_right.x - self.bottom_left.x
    }

    /// Get rect's height.
    pub fn height(&self) -> T {
        self.top_right.y - self.bottom_left.x
    }

    /// Get rect's size.
    pub fn size(&self) -> Vec2<T> {
        vec2(self.width(), self.height())
    }

    /// Check if a point is inside the rect.
    ///
    /// # Examples
    /// ```
    /// use geom::*;
    /// let rect = Rect::from_corners(vec2(1, 2), vec2(3, 4));
    /// assert!(rect.contains(vec2(2, 3)));
    /// assert!(!rect.contains(vec2(5, 5)));
    /// ```
    pub fn contains(&self, point: Vec2<T>) -> bool {
        self.bottom_left.x <= point.x && point.x < self.top_right.x && self.bottom_left.y <= point.y
            && point.y < self.top_right.y
    }
}

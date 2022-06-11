// use gdnative::api::*;
use gdnative::prelude::*;
use std::fmt;
pub mod logger;

// use crate::input::Sense;
pub fn switch_visible(owner: &Node, idx: i64) {
    let node = unsafe { owner.get_child(idx).expect("Missing node").assume_safe() }
        .cast::<CanvasItem>()
        .expect("Node should cast to CanvasItem");
    node.set_visible(!node.is_visible());
}

pub type Vec2 = Vector2;

pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub center_x: f32,
    pub center_y: f32,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    /// Creates a new rectangle from its top-left corner, width and height.
    ///
    /// # Arguments:
    ///   * `x` - x-coordinate of the top-left corner.
    ///   * `y` - y-coordinate of the top-left corner.
    ///   * `w` - width of the `Rect`, going to the right.
    ///   * `h` - height of the `Rect`, going down.
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect {
            center_x: x + (w / 2.),
            center_y: y + (h / 2.),
            x,
            y,
            w,
            h,
        }
    }

    /// Returns the top-left corner of the `Rect`.
    pub fn point(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    /// Returns the size (width and height) of the `Rect`.
    pub fn size(&self) -> Vec2 {
        vec2(self.w, self.h)
    }

    /// Returns the left edge of the `Rect`
    pub fn left(&self) -> f32 {
        self.x
    }

    /// Returns the right edge of the `Rect`
    pub fn right(&self) -> f32 {
        self.x + self.w
    }

    /// Returns the top edge of the `Rect`
    pub fn top(&self) -> f32 {
        self.y
    }

    /// Returns the bottom edge of the `Rect`
    pub fn bottom(&self) -> f32 {
        self.y + self.h
    }

    /// Moves the `Rect`'s origin to (x, y)
    pub fn move_to(&mut self, destination: Vec2) {
        self.x = destination.x;
        self.y = destination.y;
    }

    /// Scales the `Rect` by a factor of (sx, sy),
    /// growing towards the bottom-left
    pub fn scale(&mut self, sx: f32, sy: f32) {
        self.w *= sx;
        self.h *= sy;
    }

    /// Checks whether the `Rect` contains a `Point`
    pub fn contains(&self, point: Vec2) -> bool {
        point.x >= self.left()
            && point.x < self.right()
            && point.y < self.bottom()
            && point.y >= self.top()
    }
    // pub fn contains_sense(&self, sense: &Sense) -> bool {
    //     sense.mouse_x >= self.left()
    //         && sense.mouse_x < self.right()
    //         && sense.mouse_y < self.bottom()
    //         && sense.mouse_x >= self.top()
    // }
    /// Checks whether the `Rect` overlaps another `Rect`
    pub fn overlaps(&self, other: &Rect) -> bool {
        self.left() <= other.right()
            && self.right() >= other.left()
            && self.top() <= other.bottom()
            && self.bottom() >= other.top()
    }

    // /// Returns a new `Rect` that includes all points of these two `Rect`s.
    // pub fn combine_with(self, other: Rect) -> Rect {
    //     let x = f32::min(self.x, other.x);
    //     let y = f32::min(self.y, other.y);
    //     let w = f32::max(self.right(), other.right()) - x;
    //     let h = f32::max(self.bottom(), other.bottom()) - y;
    //     Rect { x, y, w, h }
    // }

    // /// Returns an intersection rect there is any intersection
    // pub fn intersect(&self, other: Rect) -> Option<Rect> {
    //     let left = self.x.max(other.x);
    //     let top = self.y.max(other.y);
    //     let right = self.right().min(other.right());
    //     let bottom = self.bottom().min(other.bottom());

    //     if right < left || bottom < top {
    //         return None;
    //     }

    //     Some(Rect {
    //         x: left,
    //         y: top,
    //         w: right - left,
    //         h: bottom - top,
    //     })
    // }

    /// Translate rect origin be `offset` vector
    // pub fn offset(self, offset: Vec2) -> Rect {
    //     Rect::new(self.x + offset.x, self.y + offset.y, self.w, self.h)
    // }

    pub fn up_split_side(&self) -> Self {
        Rect::new(self.x, self.y, self.w, self.h / 2.)
    }
    pub fn down_split_side(&self) -> Self {
        let part_h = self.h / 2.;
        Rect::new(self.x, self.y + part_h, self.w, part_h)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(
            f,
            "x: {}, y: {}, w: {}, h: {} /// center_x: {} center_y: {}",
            self.x, self.y, self.w, self.h, self.center_x, self.center_y,
        )
    }
}

extern crate sfml;

use sfml::system::Vector2f;
use sfml::graphics::{RenderWindow, RenderTarget, RenderStates, Vertex};
use std::slice::Iter;
use std::iter::FromIterator;

pub fn mid(f: &Vector2f, s: &Vector2f) -> Vector2f {
    Vector2f {
        x: (f.x + s.x) / 2f32,
        y: (f.y + s.y) / 2f32,
    }
}
pub fn div_vec(f: &Vector2f, s: &Vector2f, of: f32, n: f32) -> Vector2f {
    Vector2f {
        x: (s.x - f.x) * (of / n) + f.x,
        y: (s.y - f.y) * (of / n) + f.y,
    }
}

pub trait Draw {
    fn draw(&self, &mut RenderWindow, &mut RenderStates);
}

#[derive(Clone)]
pub struct Shape {
    points: Vec<Vector2f>,
}

impl Shape {
    pub fn gons(&self) -> usize {
        self.points.len()
    }
    pub fn center(&self) -> Vector2f {
        let mut c = Vector2f { x: 0f32, y: 0f32 };
        for v in self.points.iter() {
            c.x = c.x + v.x;
            c.y = c.y + v.y;
        }
        c.x = c.x / self.points.len() as f32;
        c.y = c.y / self.points.len() as f32;
        c
    }
}
impl From<Vec<Vector2f>> for Shape {
    fn from(points: Vec<Vector2f>) -> Shape {
        Shape { points: points }
    }
}
impl FromIterator<Vector2f> for Shape {
    fn from_iter<I: IntoIterator<Item = Vector2f>>(i: I) -> Self {
        let mut v = Vec::new();
        v.extend(i);
        Shape { points: v }
    }
}


impl Draw for Shape {
    fn draw(&self, window: &mut RenderWindow, rs: &mut RenderStates) {
        use sfml::graphics::PrimitiveType::sfLines as Lines;
        use sfml::graphics::PrimitiveType::sfPoints as Points;
        let mut seg: [Vertex; 2];
        if self.points.len() == 1 {
            seg = [Vertex::new_with_pos(&self.points[0]),
                   Vertex::new_with_pos(&self.points[0])];
            window.draw_primitives(&seg[0..1], Points, rs);
            return;
        }
        for i in 0..self.points.len() - 1 {
            seg = [Vertex::new_with_pos(&self.points[i]),
                   Vertex::new_with_pos(&self.points[i + 1])];
            window.draw_primitives(&seg[..], Lines, rs);
        }
        if self.points.len() > 2 {
            seg = [Vertex::new_with_pos(&self.points[self.points.len() - 1]),
                   Vertex::new_with_pos(&self.points[0])];
            window.draw_primitives(&seg[..], Lines, rs);
        }
    }
}
impl Shape {
    pub fn iter(&self) -> Iter<Vector2f> {
        self.points.iter()
    }
}
impl<T> Draw for Vec<T> where T: Draw
{
    fn draw(&self, window: &mut RenderWindow, rs: &mut RenderStates) {
        for shape in self.iter() {
            shape.draw(window, rs);
        }
    }
}

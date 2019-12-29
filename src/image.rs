use crate::utils::Vec2U;
use std::mem;
use std::slice::{Iter, IterMut};

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct Desc {
    pub size: Vec2U,
    pub channels: usize,
}

impl Desc {
    pub fn new(size: Vec2U, channels: usize) -> Self {
        Self { size, channels }
    }
}

#[derive(Clone, Debug)]
pub struct Image {
    desc: Desc,
    pixels: Vec<f32>,
}

impl Image {
    pub fn from_desc(desc: Desc) -> Self {
        let count = desc.size.x * desc.size.y * desc.channels;
        let mut pixels = Vec::with_capacity(count);
        pixels.resize(count, 0.0);
        Self { desc, pixels }
    }

    pub fn desc(&self) -> Desc {
        self.desc
    }

    pub fn pixels(&self) -> PixelIter {
        PixelIter::new(self.desc.channels, self.pixels.as_slice())
    }

    pub fn pixels_mut(&mut self) -> PixelIterMut {
        PixelIterMut::new(self.desc.channels, self.pixels.as_mut_slice())
    }

    pub fn lines(&self) -> LineIter {
        LineIter::new(self.desc.size.x, self.desc.channels, self.pixels.as_slice())
    }

    pub fn lines_mut(&mut self) -> LineIterMut {
        LineIterMut::new(
            self.desc.size.x,
            self.desc.channels,
            self.pixels.as_mut_slice(),
        )
    }

    pub fn elements(&self) -> Iter<f32> {
        self.pixels.iter()
    }

    pub fn elements_mut(&mut self) -> IterMut<f32> {
        self.pixels.iter_mut()
    }

    pub fn set_pixel(&mut self, pos: Vec2U, value: &[f32]) {
        let (start, end) = self.pixel_bounds(pos);
        for (out, element) in self.pixels[start..end].iter_mut().zip(value) {
            *out = *element;
        }
    }

    pub fn pixel(&self, pos: Vec2U) -> &[f32] {
        let (start, end) = self.pixel_bounds(pos);
        &self.pixels[start..end]
    }

    fn pixel_bounds(&self, pos: Vec2U) -> (usize, usize) {
        let pixels = pos.y * self.desc().size.x + pos.x;
        let channels = self.desc().channels;
        let start = pixels * channels;
        let end = start + channels;
        (start, end)
    }
}

pub struct LineIter<'a> {
    channels: usize,
    line_length: usize,
    remaining: &'a [f32],
}

impl<'a> Iterator for LineIter<'a> {
    type Item = PixelIter<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }
        let (next, remaining) = self.remaining.split_at(self.line_length);
        self.remaining = remaining;
        Some(PixelIter::new(self.channels, next))
    }
}

impl<'a> LineIter<'a> {
    pub fn new(width: usize, channels: usize, elements: &'a [f32]) -> Self {
        Self {
            channels,
            line_length: channels * width,
            remaining: elements,
        }
    }
}

pub struct LineIterMut<'a> {
    channels: usize,
    line_length: usize,
    remaining: &'a mut [f32],
}

impl<'a> Iterator for LineIterMut<'a> {
    type Item = PixelIterMut<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let slice = mem::replace(&mut self.remaining, &mut []);
        if slice.is_empty() {
            return None;
        }
        let (next, remaining) = slice.split_at_mut(self.line_length);
        self.remaining = remaining;
        Some(PixelIterMut::new(self.channels, next))
    }
}

impl<'a> LineIterMut<'a> {
    pub fn new(width: usize, channels: usize, elements: &'a mut [f32]) -> Self {
        Self {
            channels,
            line_length: channels * width,
            remaining: elements,
        }
    }
}

pub struct PixelIter<'a> {
    channels: usize,
    remaining: &'a [f32],
}

impl<'a> PixelIter<'a> {
    pub fn new(channels: usize, elements: &'a [f32]) -> Self {
        Self {
            channels,
            remaining: elements,
        }
    }
}

impl<'a> Iterator for PixelIter<'a> {
    type Item = &'a [f32];

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }
        let (next, remaining) = self.remaining.split_at(self.channels);
        self.remaining = remaining;
        Some(next)
    }
}

pub struct PixelIterMut<'a> {
    channels: usize,
    remaining: &'a mut [f32],
}

impl<'a> PixelIterMut<'a> {
    pub fn new(channels: usize, elements: &'a mut [f32]) -> Self {
        Self {
            channels,
            remaining: elements,
        }
    }
}

impl<'a> Iterator for PixelIterMut<'a> {
    type Item = &'a mut [f32];

    fn next(&mut self) -> Option<Self::Item> {
        let slice = mem::replace(&mut self.remaining, &mut []);
        if slice.is_empty() {
            return None;
        }

        let (next, remaining) = slice.split_at_mut(self.channels);
        self.remaining = remaining;
        Some(next)
    }
}

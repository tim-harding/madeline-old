use crate::utils::{Vec2I, Vec2U};
use std::mem;
use std::ops::{Index, IndexMut};
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

#[derive(Debug, Clone)]
pub struct Channel {
    size: Vec2U,
    pixels: Vec<f32>,
}

impl Channel {
    pub fn new(size: Vec2U) -> Self {
        let count = size.area();
        let mut pixels = Vec::with_capacity(count);
        pixels.resize(count, 0.0);
        Self { size, pixels }
    }

    pub fn size(&self) -> Vec2U {
        self.size
    }

    pub fn elements(&self) -> Iter<f32> {
        self.pixels.iter()
    }

    pub fn elements_mut(&mut self) -> IterMut<f32> {
        self.pixels.iter_mut()
    }

    pub fn element(&self, pos: Vec2I) -> f32 {
        match self.index_of(pos) {
            Some(i) => self.pixels[i],
            None => 0.0,
        }
    }

    pub fn set_element(&mut self, pos: Vec2I, value: f32) {
        if let Some(i) = self.index_of(pos) {
            self.pixels[i] = value;
        }
    }

    fn index_of(&self, pos: Vec2I) -> Option<usize> {
        let pos_u: Vec2U = pos.into();
        if pos.x > 0 && pos.y > 0 && pos_u.x < self.size.x && pos_u.y < self.size.y {
            Some(pos_u.y * self.size.x + pos_u.x)
        } else {
            None
        }
    }

    pub fn raw(&self) -> &[f32] {
        self.pixels.as_slice()
    }

    pub fn raw_mut(&mut self) -> &mut [f32] {
        self.pixels.as_mut_slice()
    }

    pub fn lines(&self) -> LineIter {
        LineIter::new(self)
    }

    pub fn lines_mut(&mut self) -> LineIterMut {
        LineIterMut::new(self)
    }
}

impl Index<usize> for Channel {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.pixels[i]
    }
}

impl IndexMut<usize> for Channel {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.pixels[i]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct LineIter<'a> {
    remaining: &'a [f32],
    line_length: usize,
}

impl<'a> LineIter<'a> {
    pub fn new(channel: &'a Channel) -> Self {
        Self {
            remaining: channel.raw(),
            line_length: channel.size().x,
        }
    }
}

impl<'a> Iterator for LineIter<'a> {
    type Item = Iter<'a, f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }
        let (head, tail) = self.remaining.split_at(self.line_length);
        self.remaining = tail;
        Some(head.iter())
    }
}

pub struct LineIterMut<'a> {
    remaining: &'a mut [f32],
    line_length: usize,
}

impl<'a> LineIterMut<'a> {
    pub fn new(channel: &'a mut Channel) -> Self {
        Self {
            line_length: channel.size().x,
            remaining: channel.raw_mut(),
        }
    }
}

impl<'a> Iterator for LineIterMut<'a> {
    type Item = IterMut<'a, f32>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }

        let remaining = mem::replace(&mut self.remaining, &mut []);
        let (head, tail) = remaining.split_at_mut(self.line_length);
        self.remaining = tail;
        Some(head.iter_mut())
    }
}

#[derive(Debug, Clone, Default)]
pub struct Image {
    channels: Vec<Channel>,
}

impl Image {
    pub fn from_desc(desc: Desc) -> Self {
        let mut channels = Vec::with_capacity(desc.channels);
        for _ in 0..desc.channels {
            channels.push(Channel::new(desc.size));
        }
        Self { channels }
    }

    pub fn size(&self) -> Vec2U {
        self.channels[0].size()
    }

    pub fn channel_count(&self) -> usize {
        self.channels.len()
    }

    pub fn desc(&self) -> Desc {
        Desc::new(self.size(), self.channel_count())
    }

    pub fn channels(&self) -> Iter<Channel> {
        self.channels.iter()
    }

    pub fn channels_mut(&mut self) -> IterMut<Channel> {
        self.channels.iter_mut()
    }

    pub fn push(&mut self, channel: Channel) {
        self.channels.push(channel)
    }
}

impl Index<usize> for Image {
    type Output = Channel;

    fn index(&self, i: usize) -> &Self::Output {
        &self.channels[i]
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.channels[i]
    }
}

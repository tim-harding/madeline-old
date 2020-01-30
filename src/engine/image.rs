use crate::utils::{Vec2I, Vec2U};
use rayon::{
    prelude::*,
    slice::{Iter as ParIter, IterMut as ParIterMut},
};
use std::{
    ops::{Index, IndexMut},
    slice::{Iter as StdIter, IterMut as StdIterMut},
};

type StdLines<'a> = std::slice::ChunksExact<'a, f32>;
type StdLinesMut<'a> = std::slice::ChunksExactMut<'a, f32>;
type ParLines<'a> = rayon::slice::Chunks<'a, f32>;
type ParLinesMut<'a> = rayon::slice::ChunksMut<'a, f32>;

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
    elements: Vec<f32>,
}

impl Channel {
    pub fn black(size: Vec2U) -> Self {
        Self {
            size,
            elements: vec![0.0; size.area()],
        }
    }

    pub fn size(&self) -> Vec2U {
        self.size
    }

    pub fn elements(&self) -> StdIter<f32> {
        self.elements.iter()
    }

    pub fn elements_mut(&mut self) -> StdIterMut<f32> {
        self.elements.iter_mut()
    }

    // Callers should probably use .chunks() with this
    // or the step size will be too small for parallelism
    // to be an effective tool
    pub fn par_elements(&self) -> ParIter<f32> {
        self.elements.par_iter()
    }

    // Samesies
    pub fn par_elements_mut(&mut self) -> ParIterMut<f32> {
        self.elements.par_iter_mut()
    }

    pub fn index_of(&self, pos: Vec2I) -> Option<usize> {
        let pos_u: Vec2U = pos.into();
        if pos.x >= 0 && pos.y >= 0 && pos_u.x < self.size.x && pos_u.y < self.size.y {
            Some(pos_u.y * self.size.x + pos_u.x)
        } else {
            None
        }
    }

    pub fn lines(&self) -> StdLines {
        self.elements.chunks_exact(self.size.x)
    }

    pub fn lines_mut(&mut self) -> StdLinesMut {
        self.elements.chunks_exact_mut(self.size.x)
    }

    pub fn par_lines(&self) -> ParLines {
        self.elements.par_chunks(self.size.x)
    }

    pub fn par_lines_mut(&mut self) -> ParLinesMut {
        self.elements.par_chunks_mut(self.size.x)
    }
}

impl Index<usize> for Channel {
    type Output = f32;

    fn index(&self, i: usize) -> &Self::Output {
        &self.elements[i]
    }
}

impl IndexMut<usize> for Channel {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.elements[i]
    }
}

pub struct ChannelBuilder {
    elements: Vec<f32>,
}

impl ChannelBuilder {
    pub fn build(self, size: Vec2U) -> Channel {
        assert!(size.area() == self.elements.len());
        Channel {
            elements: self.elements,
            size,
        }
    }
}

impl std::iter::FromIterator<f32> for ChannelBuilder {
    fn from_iter<I>(src: I) -> Self
    where
        I: IntoIterator<Item = f32>,
    {
        Self {
            elements: src.into_iter().collect::<Vec<_>>(),
        }
    }
}

impl rayon::iter::FromParallelIterator<f32> for ChannelBuilder {
    fn from_par_iter<I>(src: I) -> Self
    where
        I: IntoParallelIterator<Item = f32>,
    {
        Self {
            elements: src.into_par_iter().collect::<Vec<_>>(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Image {
    channels: Vec<Channel>,
}

impl Image {
    pub fn from_desc(desc: Desc) -> Self {
        assert!(desc.channels > 0);
        Self {
            channels: vec![Channel::black(desc.size); desc.channels],
        }
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

    pub fn channels(&self) -> StdIter<Channel> {
        self.channels.iter()
    }

    pub fn channels_mut(&mut self) -> StdIterMut<Channel> {
        self.channels.iter_mut()
    }

    pub fn par_channels(&self) -> ParIter<Channel> {
        self.channels.par_iter()
    }

    pub fn par_channels_mut(&mut self) -> ParIterMut<Channel> {
        self.channels.par_iter_mut()
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

impl std::iter::FromIterator<Channel> for Image {
    fn from_iter<I>(src: I) -> Self
    where
        I: IntoIterator<Item = Channel>,
    {
        let channels = src.into_iter().collect::<Vec<_>>();
        assert!(channels_are_valid(&channels));
        Self { channels }
    }
}

impl rayon::iter::FromParallelIterator<Channel> for Image {
    fn from_par_iter<I>(src: I) -> Self
    where
        I: IntoParallelIterator<Item = Channel>,
    {
        let channels = src.into_par_iter().collect::<Vec<_>>();
        assert!(channels_are_valid(&channels));
        Self { channels }
    }
}

fn channels_are_valid(channels: &[Channel]) -> bool {
    let mut iter = channels.iter();
    let first = match iter.next() {
        Some(first) => first,
        // Should have at least one channel
        None => return false,
    };
    // All channels should use the same format
    channels.iter().all(|c| c.size() == first.size())
}

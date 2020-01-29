#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Globals {
    pub screen_size: Vec2,
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct PassVert {
    pub offset: Vec2,
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct PassFrag {
    pub color: Vec3,
}

pub fn buffer<T>(device: &wgpu::Device) -> wgpu::Buffer {
    device.create_buffer(&wgpu::BufferDescriptor {
        size: std::mem::size_of::<T>() as u64,
        usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
    })
}

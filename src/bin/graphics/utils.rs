pub fn buffer_with_data<T>(
    device: &wgpu::Device,
    data: &[T],
    usage: wgpu::BufferUsage,
) -> wgpu::Buffer {
    let data = unsafe {
        std::slice::from_raw_parts(
            data.as_ptr() as *const u8,
            data.len() * std::mem::size_of::<T>(),
        )
    };
    device
        .create_buffer_mapped(data.len(), usage)
        .fill_from_slice(data)
}

#[repr(C)]
#[derive(Default, Copy, Clone, Debug)]
pub struct Locals {
    pub brightness: f32,
}

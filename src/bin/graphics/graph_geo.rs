use lyon::{
    path::{
        builder::{Build, FlatPathBuilder, PathBuilder, SvgPathBuilder},
        math::{point, vector, Angle, Point},
        Path,
    },
    tessellation,
};

const TOLERANCE: f32 = 0.5;

type IndexFormat = u16;
type Geometry = tessellation::VertexBuffers<Point, IndexFormat>;

pub struct Mesh {
    pub vbo: wgpu::Buffer,
    pub ibo: wgpu::Buffer,
    pub indices: usize,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, geo: Geometry) -> Self {
        Self {
            vbo: device
                .create_buffer_mapped(geo.vertices.len(), wgpu::BufferUsage::VERTEX)
                .fill_from_slice(&geo.vertices),

            ibo: device
                .create_buffer_mapped(geo.indices.len(), wgpu::BufferUsage::INDEX)
                .fill_from_slice(&geo.indices),

            indices: geo.indices.len(),
        }
    }
}

pub struct GraphGeo {
    pub rect: Mesh,
    pub rect_outline: Mesh,
    pub trapezoid: Mesh,
    pub slot: Mesh,
}

impl GraphGeo {
    pub fn new(device: &wgpu::Device) -> Result<Self, &'static str> {
        Ok(Self {
            rect: build(device, |b| rounded_rect(b, 90.0, 27.0, 5.0))?,
            rect_outline: build(device, |b| rounded_rect(b, 92.0, 29.0, 11.0))?,
            trapezoid: build(device, |b| trapezoid(b, 100.0, 100.0, 25.0))?,
            slot: build(device, |b| circle(b, 10.0))?,
        })
    }
}

type Builder = lyon::path::builder::SvgPathBuilder<lyon::path::Builder>;

fn build<C>(device: &wgpu::Device, cb: C) -> Result<Mesh, &'static str>
where
    C: Fn(&mut Builder),
{
    let mut builder = SvgPathBuilder::new(Path::builder());
    cb(&mut builder);
    let path = builder.build();
    let mut geo: Geometry = Geometry::new();
    tessellation::FillTessellator::new()
        .tessellate_path(
            &path,
            &tessellation::FillOptions::tolerance(TOLERANCE),
            &mut lyon::tessellation::geometry_builder::simple_builder(&mut geo),
        )
        .map_err(|_| "Failed to tesselate path")?;

    Ok(Mesh::new(device, geo))
}

fn trapezoid(builder: &mut Builder, w: f32, h: f32, shift: f32) {
    builder.line_to(point(shift, h));
    builder.line_to(point(shift + w, h));
    builder.line_to(point(w, 0.0));
    builder.close();
}

fn circle(builder: &mut Builder, r: f32) {
    builder.move_to(point(0.0, r));
    builder.arc(point(r, r), vector(r, r), Angle::two_pi(), Angle::zero());
}

fn rounded_rect(builder: &mut Builder, w: f32, h: f32, r: f32) {
    builder.move_to(point(0.0, r));
    corner(builder, r, r, r);
    builder.line_to(point(w - r, 0.0));
    corner(builder, w - r, r, r);
    builder.line_to(point(w, h - r));
    corner(builder, w - r, h - r, r);
    builder.line_to(point(r, h));
    corner(builder, r, h - r, r);
    builder.close();
}

fn corner(builder: &mut Builder, x: f32, y: f32, r: f32) {
    builder.arc(
        point(x, y),
        vector(r, r),
        Angle::frac_pi_2(),
        // x-rotation has something to do with start angle,
        // but it doesn't seem to have any effect here
        Angle::zero(),
    );
}

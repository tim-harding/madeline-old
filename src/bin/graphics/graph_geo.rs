use lyon::{
    path::{
        builder::*,
        math::{Angle, Point, Vector},
        Path,
    },
    tessellation::{self, geometry_builder::FillVertexConstructor},
};

const TOLERANCE: f32 = 0.02;

type Geometry = tessellation::VertexBuffers<Position, IndexFormat>;
type IndexFormat = u16;

pub struct GraphGeo {
    pub geometry: Geometry,
}

impl GraphGeo {
    pub fn new() -> Result<Self, &'static str> {
        let mut builder = SvgPathBuilder::new(Path::builder());
        builder.move_to(Point::new(0.0, 0.1));
        builder.arc(
            Point::new(0.1, 0.1),
            Vector::new(0.1, 0.1),
            Angle::frac_pi_2(),
            Angle::frac_pi_2(),
        );
        builder.line_to(Point::new(0.4, 0.0));
        builder.arc(
            Point::new(0.4, 0.1),
            Vector::new(0.1, 0.1),
            Angle::frac_pi_2(),
            Angle::frac_pi_2(),
        );
        builder.line_to(Point::new(0.5, 0.4));
        builder.arc(
            Point::new(0.4, 0.4),
            Vector::new(0.1, 0.1),
            Angle::frac_pi_2(),
            Angle::frac_pi_2(),
        );
        builder.line_to(Point::new(0.1, 0.5));
        builder.arc(
            Point::new(0.1, 0.4),
            Vector::new(0.1, 0.1),
            Angle::frac_pi_2(),
            Angle::frac_pi_2(),
        );
        builder.close();
        let path = builder.build();

        let mut geometry: Geometry = Geometry::new();

        let _fill_count = tessellation::FillTessellator::new()
            .tessellate_path(
                &path,
                &tessellation::FillOptions::tolerance(TOLERANCE),
                &mut tessellation::BuffersBuilder::new(&mut geometry, PositionBuilder::default()),
            )
            .map_err(|_| "Failed to tesselate path")?;

        Ok(Self { geometry })
    }
}

#[derive(Default)]
struct PositionBuilder {}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// Border can also be done using an offset path
// rendered behind the solid fill.
// For this, storing the normals along with position
// may be useful.
impl FillVertexConstructor<Position> for PositionBuilder {
    fn new_vertex(
        &mut self,
        position: Point,
        _attributes: tessellation::FillAttributes,
    ) -> Position {
        debug_assert!(!position.x.is_nan());
        debug_assert!(!position.y.is_nan());
        Position {
            x: position.x,
            y: position.y,
        }
    }
}

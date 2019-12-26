use crate::utils::Id;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Color {
    Grayscale(Id),
    Rg(Id, Id),
    Rgb(Id, Id, Id),
    Rgba(Id, Id, Id, Id),
}

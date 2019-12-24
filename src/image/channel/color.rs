use crate::utils::Id;

pub enum Color {
    Grayscale(Id),
    Rg(Id, Id),
    Rgb(Id, Id, Id),
    Rgba(Id, Id, Id, Id),
}

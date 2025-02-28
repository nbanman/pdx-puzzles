pub mod grid_adjacent;
pub mod grid_constructors;
pub mod grid_errors;
pub mod grid_index;
pub mod grid_iterators;
pub mod grid_methods;
pub mod grid_rotation;

#[allow(unused_imports)]
pub use grid_adjacent::*;
pub use grid_constructors::*;
pub use grid_errors::*;
pub use grid_index::*;
#[allow(unused_imports)]
pub use grid_iterators::*;
#[allow(unused_imports)]
pub use grid_methods::*;
#[allow(unused_imports)]
pub use grid_rotation::*;

pub type Grid2<T> = Grid<T, 2>;

#[derive(Debug, PartialEq, Eq)]
pub struct Grid<T, const N: usize> {
    data: Vec<T>,
    pub dimensions: [usize; N],
}

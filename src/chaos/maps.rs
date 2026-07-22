pub mod polynomial_maps;



pub trait Map<T, const D: usize>
{
    fn apply(&self, input: &nalgebra::SVector<T, D>) -> nalgebra::SVector<T, D>;
}

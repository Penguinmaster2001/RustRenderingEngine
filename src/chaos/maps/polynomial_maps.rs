use crate::chaos::maps::Map;
use nalgebra::RealField;



pub struct PolynomialTerm<T, const D: usize>
{
    pub exponents: [usize; D],
    pub coefficient: nalgebra::SVector<T, D>,
}



pub struct PolynomialMap<T, const D: usize>
{
    pub terms: Vec<PolynomialTerm<T, D>>,
}



impl<T: RealField, const D: usize> Map<T, D> for PolynomialMap<T, D>
{
    fn apply(&self, input: &nalgebra::SVector<T, D>) -> nalgebra::SVector<T, D>
    {
        let mut output = nalgebra::SVector::<T, D>::zeros();

        for term in &self.terms
        {
            let monomial = input
                .iter()
                .zip(term.exponents.iter())
                .fold(T::one(), |acc, (x, e)| acc * x.clone().powi(*e as i32));

            output += &term.coefficient * monomial;
        }

        output
    }
}

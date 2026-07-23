use std::println;

use crate::chaos::maps::Map;
use nalgebra::RealField;
use rand::Rng;



#[derive(Debug)]
pub struct PolynomialTerm<T, const D: usize>
{
    pub exponents: [usize; D],
    pub coefficient: nalgebra::SVector<T, D>,
}



pub struct PolynomialMap<T, const D: usize>
{
    pub terms: Vec<PolynomialTerm<T, D>>,
}



impl<T, const D: usize> PolynomialMap<T, D>
where
    T: RealField,
    rand::distr::StandardUniform: rand::distr::Distribution<T>,
{
    pub fn new_random<F>(mut rng: F, max_order: usize) -> Self
    where
        F: FnMut() -> T,
    {
        let mut terms = vec![];
        let mut exponents: Vec<usize> = vec![0usize; D + 1];
        exponents[0] = max_order;
        loop
        {
            let mut coefficient = nalgebra::SVector::zeros();

            for c in 0..D
            {
                coefficient[c] = rng();
            }

            terms.push(PolynomialTerm {
                exponents: *exponents[1..(D + 1)].as_array().unwrap(),
                coefficient,
            });

            let mut i = (D - 1) as i32;
            while i >= 0 && exponents[i as usize] == 0
            {
                i -= 1;
            }

            if i < 0
            {
                break;
            }

            let mut right_sum = 0;
            for j in (i as usize + 1)..exponents.len()
            {
                right_sum += exponents[j];
            }

            exponents[i as usize] -= 1;
            exponents[i as usize + 1] = right_sum + 1;

            for j in (i as usize + 2)..exponents.len()
            {
                exponents[j] = 0;
            }
        }
        let mut coefficient = nalgebra::SVector::zeros();

        for c in 0..D
        {
            coefficient[c] = rng();
        }

        terms.push(PolynomialTerm {
            exponents: *exponents[1..(D + 1)].as_array().unwrap(),
            coefficient,
        });
        println!("{:?}", terms);

        Self { terms }
    }
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

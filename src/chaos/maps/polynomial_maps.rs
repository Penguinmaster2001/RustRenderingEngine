use crate::chaos::{
    ChaoticPoints,
    maps::Map,
};
use nalgebra::RealField;
use rand::{
    Rng,
    rngs::ThreadRng,
};
use std::println;



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

        let mut coefficient = nalgebra::SVector::zeros();

        if rng().abs() < T::from_usize(max_order).unwrap().recip()
        {
            for c in 0..D
            {
                coefficient[c] = rng();
            }

            terms.push(PolynomialTerm {
                exponents: *exponents[1..(D + 1)].as_array().unwrap(),
                coefficient,
            });
        }

        exponents[0] = max_order;
        loop
        {
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
            ((i as usize + 1)..exponents.len()).for_each(|j| {
                right_sum += exponents[j];
            });

            exponents[i as usize] -= 1;
            exponents[i as usize + 1] = right_sum + 1;

            ((i as usize + 2)..exponents.len()).for_each(|j| {
                exponents[j] = 0;
            });

            let current_order = max_order - exponents[0];

            let mut coefficient = nalgebra::SVector::zeros();
            for c in 0..D
            {
                if rng().abs() < T::from_usize(current_order).unwrap().recip()
                {
                    coefficient[c] = rng() / (T::one() + T::one()).powi((current_order - 1) as _);
                }
            }

            terms.push(PolynomialTerm {
                exponents: *exponents[1..(D + 1)].as_array().unwrap(),
                coefficient,
            });
        }
        println!("{:?}\n\n", terms);

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



pub type ChaoticPolynomialMap<T, const D: usize> = ChaoticPoints<T, PolynomialMap<T, D>, D>;


impl<const D: usize> ChaoticPolynomialMap<f32, D>
{
    pub fn new_chaotic_polynomial_map() -> Self
    {
        let mut rng = ThreadRng::default();
        let map = PolynomialMap::new_random(|| rng.random_range(-1.5..1.5), 4);

        ChaoticPoints::from_point_grid(1.0f32, 10, map)
    }
}

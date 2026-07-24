use nalgebra::RealField;
use std::ops::Range;



pub fn not_divergent_or_collapsed<T, const D: usize>(
    points: &[nalgebra::SVector<T, D>],
    bounds: Range<T>,
    threshold: T,
) -> bool
where
    T: RealField + Copy,
{
    let mut center = nalgebra::SVector::zeros();
    let mut bounded: Vec<_> = vec![];
    let count = points
        .iter()
        .filter(|p| {
            if p.magnitude_squared() > bounds.end
            {
                true
            }
            else
            {
                center += *p;
                bounded.push(*p);
                false
            }
        })
        .count();

    let total = T::from_usize(points.len()).unwrap();
    center /= total;

    T::from_usize(count).unwrap() / total < threshold
        && T::from_usize(
            bounded
                .iter()
                .filter(|p| p.metric_distance(&center) < bounds.start)
                .count(),
        )
        .unwrap()
            / total
            < threshold
}

use crate::chaos::maps::Map;
use nalgebra::RealField;



pub mod chaos_thread;
pub mod heuristics;
pub mod maps;



#[derive(Clone)]
pub struct ChaosConfig<T, M, F, G, const D: usize>
where
    F: FnMut() -> ChaoticPoints<T, M, D>,
    G: FnMut(&Vec<nalgebra::SVector<T, D>>) -> bool,
{
    pub max_iterations: Option<u32>,
    pub points_generator: F,
    pub chaos_heuristic: G,
}



pub struct ChaoticPoints<T, M, const D: usize>
{
    pub points: Vec<nalgebra::SVector<T, D>>,
    pub map: M,
    iter_num: u32,
}



impl<T, M, const D: usize> ChaoticPoints<T, M, D>
{
    pub fn new(points: Vec<nalgebra::SVector<T, D>>, map: M) -> Self
    {
        Self {
            points,
            map,
            iter_num: 0,
        }
    }



    pub fn copy_from(&mut self, other: ChaoticPoints<T, M, D>)
    {
        self.points = other.points;
        self.map = other.map;
        self.iter_num = other.iter_num;
    }
}



fn gen_points<T, const D: usize>(
    start: nalgebra::SVector<T, D>,
    dim: usize,
    edge_num: u16,
    spacing: T,
) -> Vec<nalgebra::SVector<T, D>>
where
    T: RealField + Copy,
{
    if dim >= D
    {
        return vec![start];
    }

    let mut start = start;
    let mut points = vec![];

    for _ in 0..(2 * edge_num)
    {
        start[dim] += spacing;
        points.extend(gen_points(start, dim + 1, edge_num, spacing));
    }

    points
}


impl<T, M, const D: usize> ChaoticPoints<T, M, D>
{
    pub fn get_iter_num(&self) -> u32
    {
        self.iter_num
    }
}



impl<T, M, const D: usize> ChaoticPoints<T, M, D>
where
    T: RealField + From<u16> + Copy,
{
    pub fn from_point_grid(size: T, edge_num: u16, map: M) -> Self
    {
        let spacing = size / edge_num.into();

        let points = gen_points(nalgebra::SVector::from_element(-size), 0, edge_num, spacing);

        Self {
            points,
            map,
            iter_num: 0,
        }
    }
}



impl<T, M, const D: usize> ChaoticPoints<T, M, D>
where
    M: Map<T, D>,
{
    pub fn step(&mut self)
    {
        for i in 0..self.points.len()
        {
            self.points[i] = self.map.apply(&self.points[i]);
        }

        self.iter_num += 1;
    }
}

use crate::chunking::blocks::{
    Block,
    BlockType,
};
use noise::{
    Fbm,
    MultiFractal,
    NoiseFn,
    Perlin,
    Worley,
    core::worley::distance_functions,
};



pub struct HeightNoise
{
    scale: f64,
    height_scale: f64,
    base_noise: Box<dyn NoiseFn<f64, 2>>,
}



impl HeightNoise
{
    pub fn new() -> Self
    {
        // let noise = Perlin::new(0);
        let noise = Fbm::<Perlin>::new(0);

        Self {
            scale: 0.01,
            height_scale: 20.0,
            base_noise: Box::new(noise),
        }
    }



    pub fn get(&self, x: i64, z: i64) -> i64
    {
        (self.height_scale
            * (1.0
                + self
                    .base_noise
                    .get([x as f64 * self.scale, z as f64 * self.scale]))) as i64
    }
}



pub struct PlanetNoise
{
    scale: f64,
    base_noise: Box<dyn NoiseFn<f64, 3>>,
    _layer_noise: Box<dyn NoiseFn<f64, 3>>,
}



impl PlanetNoise
{
    pub fn new() -> Self
    {
        // let noise = Perlin::new(0);
        // let noise = Fbm::<Perlin>::new(0);
        let base_noise = Worley::new(0)
            .set_distance_function(distance_functions::euclidean_squared)
            .set_return_type(noise::core::worley::ReturnType::Distance);

        // let layer_noise = Fbm::<Perlin>::new(0).set_octaves(6);
        let layer_noise = Perlin::new(0);

        Self {
            scale: 0.01,
            base_noise: Box::new(base_noise),
            _layer_noise: Box::new(layer_noise),
        }
    }



    pub fn get<P: Into<(i64, i64, i64)>>(&self, point: P) -> BlockType
    {
        let (x, y, z) = point.into();
        let sample_point = [
            x as f64 * self.scale,
            y as f64 * self.scale,
            z as f64 * self.scale,
        ];
        let mut noise_value = 0.5 * (1.0 - self.base_noise.get(sample_point));

        // noise_value += 0.05 * self.layer_noise.get(sample_point);

        noise_value = noise_value.clamp(0.0, 1.0);

        if noise_value < 0.97
        {
            return BlockType::Empty;
        }

        noise_value *= BlockType::Max as u8 as f64;

        (noise_value as u8).try_into().unwrap_or(BlockType::Empty)
    }
}

use noise::{
    Fbm,
    NoiseFn,
    Perlin,
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

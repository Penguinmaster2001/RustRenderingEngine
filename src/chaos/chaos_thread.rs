use crate::{
    chaos::{
        ChaosConfig,
        ChaoticPoints,
        maps::Map,
    },
    threading::{
        WorkerHandle,
        WorkerThread,
    },
};
use std::{
    thread::sleep,
    time::Duration,
};



pub enum ChaosCommand
{
    CreateNew,
}



pub type ChaosHandle<T, M, const D: usize> = WorkerHandle<ChaosCommand, ChaoticPoints<T, M, D>>;
pub type ChaosThread<T, M, F, const D: usize> =
    WorkerThread<ChaosConfig<T, M, F, D>, ChaosCommand, ChaoticPoints<T, M, D>, ()>;



impl<T, M, const D: usize> ChaosHandle<T, M, D>
where
    T: 'static + Send + Sync,
    M: 'static + Send + Sync + Map<T, D>,
{
    pub fn new_chaos_thread<F>(config: ChaosConfig<T, M, F, D>) -> Self
    where
        F: 'static + Send + FnMut() -> ChaoticPoints<T, M, D>,
    {
        let mut config = config;
        let points = (config.points_generator)();
        WorkerHandle::new(config, points, (), run)
    }
}



fn run<T, M, F, const D: usize>(chaos_thread: ChaosThread<T, M, F, D>)
where
    M: Map<T, D>,
    F: FnMut() -> ChaoticPoints<T, M, D>,
{
    let mut chaos_thread = chaos_thread;

    loop
    {
        if let Ok(mut points) = chaos_thread.state.write()
        {
            for event in chaos_thread.input_rx.try_iter()
            {
                match event
                {
                    ChaosCommand::CreateNew =>
                    {
                        points.copy_from((chaos_thread.config.points_generator)())
                    }
                };
            }

            if points.get_iter_num() < chaos_thread.config.iterations
            {
                points.step();
            }
        }
        sleep(Duration::from_micros(100));
    }
}

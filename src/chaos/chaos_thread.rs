use crate::{
    chaos::{
        ChaosConfig,
        ChaoticPoints,
        chaos_thread::RunState::{
            Displaying,
            Searching,
        },
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
pub type ChaosThread<T, M, F, G, const D: usize> =
    WorkerThread<ChaosConfig<T, M, F, G, D>, ChaosCommand, ChaoticPoints<T, M, D>, RunState>;



impl<T, M, const D: usize> ChaosHandle<T, M, D>
where
    T: 'static + Send + Sync,
    M: 'static + Send + Sync + Map<T, D>,
{
    pub fn new_chaos_thread<F, G>(config: ChaosConfig<T, M, F, G, D>) -> Self
    where
        F: 'static + Send + FnMut() -> ChaoticPoints<T, M, D>,
        G: 'static + Send + FnMut(&Vec<nalgebra::SVector<T, D>>) -> bool,
    {
        let mut config = config;
        let points = (config.points_generator)();
        WorkerHandle::new(config, points, RunState::Searching(0), run)
    }
}



pub enum RunState
{
    Searching(u32),
    Displaying,
}



fn run<T, M, F, G, const D: usize>(chaos_thread: ChaosThread<T, M, F, G, D>)
where
    M: Map<T, D>,
    F: FnMut() -> ChaoticPoints<T, M, D>,
    G: FnMut(&Vec<nalgebra::SVector<T, D>>) -> bool,
{
    let mut chaos_thread = chaos_thread;

    loop
    {
        if let Ok(mut points) = chaos_thread.state.write()
        {
            if chaos_thread
                .config
                .max_iterations
                .is_none_or(|i| i < points.get_iter_num())
            {
                points.step();
            }

            match chaos_thread.internal_state
            {
                RunState::Searching(iter_count) =>
                {
                    chaos_thread.internal_state = Searching(iter_count + 1);

                    if !(chaos_thread.config.chaos_heuristic)(&points.points)
                    {
                        chaos_thread.internal_state = Searching(0);
                        points.copy_from((chaos_thread.config.points_generator)())
                    }
                    else if iter_count > 200
                    {
                        chaos_thread.internal_state = Displaying;
                        points.points = (chaos_thread.config.points_generator)().points;
                        chaos_thread.input_rx.try_iter();
                    }
                }
                RunState::Displaying =>
                {
                    for event in chaos_thread.input_rx.try_iter()
                    {
                        match event
                        {
                            ChaosCommand::CreateNew =>
                            {
                                chaos_thread.internal_state = Searching(0);
                                points.copy_from((chaos_thread.config.points_generator)())
                            }
                        };
                    }
                }
            }
        }
        sleep(Duration::from_micros(100));
    }
}

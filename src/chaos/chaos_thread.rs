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
    RestartCurrent,
    Pause,
    Step,
}



pub type ChaosHandle<T, M, const D: usize> = WorkerHandle<ChaosCommand, ChaoticPoints<T, M, D>>;
pub type ChaosThread<T, M, F, G, P, const D: usize> =
    WorkerThread<ChaosConfig<T, M, F, G, P, D>, ChaosCommand, ChaoticPoints<T, M, D>, RunState>;



impl<T, M, const D: usize> ChaosHandle<T, M, D>
where
    T: 'static + Send + Sync,
    M: 'static + Send + Sync + Map<T, D>,
{
    pub fn new_chaos_thread<F, G, P>(config: ChaosConfig<T, M, F, G, P, D>) -> Self
    where
        P: 'static + Send + Sync + Copy,
        F: 'static + Send + FnMut(&P) -> ChaoticPoints<T, M, D>,
        G: 'static + Send + FnMut(&Vec<nalgebra::SVector<T, D>>) -> bool,
    {
        let mut config = config;
        let points = (config.points_generator)(&config.search_config);
        WorkerHandle::new(config, points, RunState::Searching(0), run)
    }
}



#[derive(PartialEq)]
pub enum RunState
{
    Searching(u32),
    Displaying,
    Paused,
    Step,
}



fn run<T, M, F, G, P, const D: usize>(chaos_thread: ChaosThread<T, M, F, G, P, D>)
where
    M: Map<T, D>,
    F: FnMut(&P) -> ChaoticPoints<T, M, D>,
    G: FnMut(&Vec<nalgebra::SVector<T, D>>) -> bool,
{
    let mut chaos_thread = chaos_thread;

    loop
    {
        if let Ok(mut points) = chaos_thread.state.write()
        {
            if chaos_thread.internal_state != RunState::Paused
                && chaos_thread
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
                        points.copy_from((chaos_thread.config.points_generator)(
                            &chaos_thread.config.search_config,
                        ))
                    }
                    else if iter_count > 500
                    {
                        chaos_thread.internal_state = Displaying;
                        points.points = (chaos_thread.config.points_generator)(
                            &chaos_thread.config.display_config,
                        )
                        .points
                    }
                }
                RunState::Displaying => (),
                RunState::Paused => (),
                RunState::Step => chaos_thread.internal_state = RunState::Paused,
            }

            for event in chaos_thread.input_rx.try_iter()
            {
                match event
                {
                    ChaosCommand::CreateNew =>
                    {
                        chaos_thread.internal_state = Searching(0);
                        points.copy_from((chaos_thread.config.points_generator)(
                            &chaos_thread.config.search_config,
                        ))
                    }
                    ChaosCommand::RestartCurrent =>
                    {
                        points.points = (chaos_thread.config.points_generator)(
                            &chaos_thread.config.display_config,
                        )
                        .points
                    }
                    ChaosCommand::Pause =>
                    {
                        if chaos_thread.internal_state == RunState::Paused
                        {
                            chaos_thread.internal_state = RunState::Displaying
                        }
                        else
                        {
                            chaos_thread.internal_state = RunState::Paused
                        }
                    }
                    ChaosCommand::Step => chaos_thread.internal_state = RunState::Step,
                };
            }
        }
        sleep(Duration::from_micros(100));
    }
}

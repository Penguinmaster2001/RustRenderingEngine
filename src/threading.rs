use std::{
    sync::{
        Arc,
        RwLock,
        RwLockReadGuard,
        mpsc,
    },
    thread,
};



pub struct WorkerHandle<C, T, S>
{
    pub config: C,
    input_tx: mpsc::Sender<T>,
    handle: thread::JoinHandle<()>,
    state: Arc<RwLock<S>>,
}



impl<'w: 'static, C: Send + Copy + 'w, T: Send + 'w, S: Send + Sync + 'w> WorkerHandle<C, T, S>
{
    pub fn new<I: Send + 'w, F>(config: C, state: S, internal_state: I, run: F) -> Self
    where
        F: FnOnce(WorkerThread<C, T, S, I>) + Send + 'w,
    {
        let state = Arc::new(RwLock::new(state));
        let (input_tx, input_rx) = mpsc::channel();
        let worker_thread = WorkerThread::new(config, state.clone(), internal_state, input_rx);
        Self {
            config,
            input_tx,
            handle: thread::spawn(move || {
                run(worker_thread);
            }),
            state,
        }
    }



    pub fn send(&self, event: T)
    {
        self.input_tx
            .send(event)
            .expect("Should be able to send input event.");
    }



    pub fn get_state(&'_ self) -> Option<RwLockReadGuard<'_, S>>
    {
        self.state.read().ok()
    }
}



pub struct WorkerThread<C, T, S, I>
{
    pub config: C,
    pub state: Arc<RwLock<S>>,
    pub internal_state: I,
    pub input_rx: mpsc::Receiver<T>,
}



impl<C, T, S, I> WorkerThread<C, T, S, I>
{
    pub fn new(
        config: C,
        state: Arc<RwLock<S>>,
        internal_state: I,
        input_rx: mpsc::Receiver<T>,
    ) -> Self
    {
        Self {
            config,
            internal_state,
            state,
            input_rx,
        }
    }
}

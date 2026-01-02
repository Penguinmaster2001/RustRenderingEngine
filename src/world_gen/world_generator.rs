use crate::chunking::{
    chunk::{
        Chunk,
        ChunkContainer,
    },
    chunk_mesh::ChunkMeshData,
};
use nalgebra::{
    Point3,
    Vector3,
};
use std::{
    collections::HashSet,
    sync::{
        Arc,
        Mutex,
        mpsc,
    },
    thread,
};



#[derive(Clone)]
pub struct ChunkJob
{
    pub pos: Point3<i64>,
}



pub struct WorldGenerator
{
    pub load_radius: i64,
    job_tx: mpsc::Sender<ChunkJob>,
    result_rx: mpsc::Receiver<(Chunk, ChunkMeshData)>,
    _worker_handles: Vec<thread::JoinHandle<()>>,
    generated_chunks: HashSet<Point3<i64>>,
}



impl WorldGenerator
{
    pub fn new(load_radius: i64, num_workers: usize) -> Self
    {
        let (job_tx, job_rx) = mpsc::channel::<ChunkJob>();
        let (result_tx, result_rx) = mpsc::channel::<(Chunk, ChunkMeshData)>();

        let job_rx = Arc::new(Mutex::new(job_rx));
        let result_tx = Arc::new(result_tx);

        let mut worker_handles = Vec::new();

        for id in 0..num_workers
        {
            let job_rx = job_rx.clone();
            let result_tx = result_tx.clone();

            let handle = thread::spawn(move || {
                let worker_id = id;
                loop
                {
                    let job = {
                        let rx = job_rx.lock().expect("Bad lock {worker_id}");
                        rx.recv()
                    };

                    let job = match job
                    {
                        Ok(j) => j,
                        Err(_) =>
                        {
                            println!("Thread exit {worker_id}");
                            break;
                        }
                    };

                    // let chunk = Chunk::from_offset(&job.pos);
                    let chunk = Chunk::generate_planets(&job.pos);
                    let mesh_data = ChunkMeshData::from_chunk(&chunk);

                    let _ = result_tx.send((chunk, mesh_data));
                }
            });

            worker_handles.push(handle);
        }

        Self {
            load_radius,
            job_tx,
            result_rx,
            _worker_handles: worker_handles,
            generated_chunks: HashSet::new(),
        }
    }



    pub fn generate_chunks(&mut self, center_pos: &Point3<f32>, num: u32)
    {
        let mut remaining = num;
        let center_chunk = ChunkContainer::world_to_chunk(center_pos);

        'outer: for radius in 0..self.load_radius
        {
            for x in -radius..=radius
            {
                for z in -radius..=radius
                {
                    if x != -radius && x != radius && z != -radius && z != radius
                    {
                        continue;
                    }
                    for mut y in 0..=(2 * self.load_radius)
                    {
                        if y % 2 == 0
                        {
                            y /= 2;
                        }
                        else
                        {
                            y = -(y + 1) / 2;
                        }

                        let pos = center_chunk + Vector3::new(x, y, z);
                        if self.generated_chunks.insert(pos)
                        {
                            let _ = self.job_tx.send(ChunkJob { pos });

                            remaining -= 1;
                            if remaining == 0
                            {
                                break 'outer;
                            }
                        }
                    }
                }
            }
        }
    }



    pub fn drain_results(&mut self) -> Vec<(Chunk, ChunkMeshData)>
    {
        let mut out = Vec::new();

        while let Ok(result) = self.result_rx.try_recv()
        {
            out.push(result);
        }

        out
    }
}

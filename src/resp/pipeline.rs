#[derive(Clone)]
pub struct Pipeline {
    pub count: u64,
    pub pipeline: redis::Pipeline,
}

impl Pipeline {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            count: 0,
            pipeline: redis::Pipeline::with_capacity(capacity),
        }
    }

    pub fn len(&self) -> u64 {
        self.count
    }

    pub fn add_command(&mut self, cmd: redis::Cmd) {
        self.pipeline.add_command(cmd).ignore();
        self.count += 1;
    }

    pub async fn execute(&self, conn: &mut impl redis::aio::ConnectionLike) {
        let _: () = self.pipeline.query_async(conn).await.unwrap();
    }
}

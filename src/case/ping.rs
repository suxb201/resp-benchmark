use crate::resp;
use crate::Case;
use async_trait::async_trait;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Ping {
    name: String,
    connections: u64,
    pipeline: u64,
    requests: u64,
}

#[async_trait]
impl Case for Ping {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn connections(&self) -> u64 {
        return self.connections;
    }

    fn requests(&self) -> u64 {
        return self.requests;
    }

    async fn run(&self, mut c: resp::Client) {
        let mut pipeline = resp::Pipeline::new();
        let cmd = redis::cmd("PING");
        for _ in 0..self.pipeline {
            pipeline.add_command(cmd.clone());
        }

        while c.run_cmd(&pipeline).await {}
        return;
    }
}

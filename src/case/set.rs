use crate::resp;
use crate::Case;
use async_trait::async_trait;
use rand::SeedableRng;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
struct Requests {
    requests_count: u64,
    key_distribution: String,
    key_range: u64,
    value_size: u64,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Set {
    name: String,
    connections: u64,
    pipeline: u64,
    requests: Requests,
}

#[async_trait]
impl Case for Set {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn connections(&self) -> u64 {
        self.connections
    }

    fn requests(&self) -> u64 {
        self.requests.requests_count
    }

    async fn run(&self, mut c: resp::Client) {
        if self.requests.key_range == 0 || self.requests.key_range >= 100_0000_0000 {
            println!("Invalid key range: {}, must be in (0, 100_0000_0000)", self.requests.key_range);
            return;
        }
        if self.requests.value_size == 0 || self.requests.value_size >= 100_0000_0000 {
            println!("Invalid value size: {}, must be in (0, 100_0000_0000)", self.requests.value_size);
            return;
        }

        let mut rng = {
            let rng = rand::thread_rng();
            rand::rngs::StdRng::from_rng(rng).unwrap()
        };

        let distribution = crate::case::DistributionEnum::from_str(&self.requests.key_distribution, self.requests.key_range);
        let value = crate::random::gen_random_string(self.requests.value_size);

        loop {
            let mut pipeline = resp::Pipeline::new();
            for _ in 0..self.pipeline {
                let key = format!("{:0>8}", distribution.sample(&mut rng));
                let value = value.clone();
                let mut cmd = redis::cmd("SET");
                cmd.arg(key).arg(value);
                pipeline.add_command(cmd);
            }
            if !c.run_cmd(&pipeline).await {
                break;
            }
        }
    }
}

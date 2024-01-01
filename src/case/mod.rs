pub mod ping;
pub mod set;

use crate::resp;
use async_trait::async_trait;
pub use ping::Ping;
use rand::distributions::Distribution;
pub use set::Set;

#[async_trait]
pub trait Case: Clone + Send + 'static {
    fn name(&self) -> String;
    fn connections(&self) -> u64;
    fn requests(&self) -> u64;
    async fn run(&self, c: resp::Client);
}

enum DistributionEnum {
    Uniform(rand::distributions::Uniform<u64>),
    Zipfian(zipf::ZipfDistribution),
}

impl DistributionEnum {
    fn from_str(s: &str, range: u64) -> Self {
        match s {
            "uniform" => Self::Uniform(rand::distributions::Uniform::new(0, range)),
            "zipfian" => Self::Zipfian(zipf::ZipfDistribution::new(range as usize, 1.03).unwrap()),
            _ => panic!("Unknown distribution"),
        }
    }
    fn sample(&self, rng: &mut impl rand::Rng) -> u64 {
        match self {
            Self::Uniform(d) => d.sample(rng),
            Self::Zipfian(d) => d.sample(rng) as u64,
        }
    }
}

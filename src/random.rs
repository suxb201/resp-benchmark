use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn gen_random_string(n: u64) -> String {
    let rng = thread_rng();
    let chars: String = rng.sample_iter(&Alphanumeric).take(n as usize).map(char::from).collect();
    chars
}

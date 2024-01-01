use enum_dispatch::enum_dispatch;
use serde::Deserialize;
use std::sync::atomic::AtomicU64;
use toml::Table;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
struct Config {
    cluster: bool,
    ping: Option<Vec<Ping>>,
    set: Option<Vec<Set>>,
}

#[derive(Deserialize, Debug)]
struct Ping {
    name: String,
}

#[derive(Deserialize, Debug)]
struct Set {
    name: String,
}

#[enum_dispatch]
trait Run {
    fn run(&self);
}

impl Run for Ping {
    fn run(&self) {
        println!("ping");
    }
}

impl Run for Set {
    fn run(&self) {
        println!("get");
    }
}

#[derive(Deserialize, Debug)]
#[enum_dispatch(Run)]
enum Cmd {
    ping(Ping),
    set(Set),
}

fn main() {
    // load /Users/suxb201/gits/resp-benchmark/workloads/test.toml file to toml::Table
    let s = std::fs::read_to_string("/Users/suxb201/gits/resp-benchmark/workloads/test.toml").unwrap();
    let table = toml::from_str::<Table>(&s).unwrap();
    println!("{:?}", table);

    let config: Config = toml::from_str(&s).unwrap();
    println!("{:?}", config);
    // for case in config.ping.unwrap() {
    //     case.run();
    // }
    // for case in config.set.unwrap() {
    //     case.run();
    // }
    if let Some(ping) = config.ping {
        for cmd in ping {
            cmd.run();
        }
    }
    if let Some(set) = config.set {
        for cmd in set {
            cmd.run();
        }
    }
}

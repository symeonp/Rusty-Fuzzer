extern crate chrono;
use chrono::{DateTime, Utc};
use std::collections::hash_map::DefaultHasher;
use std::collections::VecDeque;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::time::{Duration, Instant, SystemTime};

#[derive(Debug, Clone)]
pub struct SeedConfig {
    arg_count: usize,
    id: usize,
    pub seed: Vec<String>,
    time: ExecTime,
    parents: Vec<u32>,
    exit_stat: Stat,
    fitness: u8,
}

impl SeedConfig {
        pub fn new(seed: Vec<String>,id: usize) -> Self {
        
        Self {
            arg_count: seed.len(),
            seed,
            time: ExecTime {
                limit: 0,
                total: [].to_vec(),
            },
            id,
            exit_stat: Stat::NONE,
            fitness: 0,
            parents: [].to_vec(),
        }
    }

    pub fn update() -> std::io::Result<()> {
        Ok(())
    }

    pub fn init_queue(seedfile: &str) -> std::io::Result<VecDeque<SeedConfig>> {
        let mut config_queue: VecDeque<SeedConfig> = VecDeque::new();
        for (id,path) in fs::read_dir(seedfile)?.enumerate() {
            let file = path?;
            let f = File::open(file.path())?;
            let f = BufReader::new(f);
            let mut seedv: Vec<String> = Vec::new();
            for line in f.lines() {
                seedv.push(line.unwrap());
            }
            let conf = SeedConfig::new(seedv.clone(),id);
            config_queue.push_back(conf);
        }
        Ok(config_queue)
    }
}
#[derive(Debug, Clone)]
struct ExecTime {
    limit: u32,
    total: Vec<Duration>,
}
#[derive(Debug, Clone)]
struct CrashHash {
    headhash: DefaultHasher,
    tailhash: DefaultHasher,
    fullhash: DefaultHasher,
}
#[derive(Debug, Clone)]
enum Stat {
    NONE,
    SUCCESS,
    CRASH(CrashHash),
    HANG,
}
#[derive(Debug, Clone)]
pub struct ProgConfig {
    inputpath: String,
    outputdir: String,
    timeout: u32,
}

impl ProgConfig {
    pub fn init(inputpath: String, outputdir: String, limit: u32) -> Self {
        Self {
            inputpath,
            outputdir,
            timeout: limit,
        }
    }
}

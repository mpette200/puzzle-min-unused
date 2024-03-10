use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::HashMap;
use std::hint::black_box;
use std::time::{Duration, Instant};
use RangeDef::{EndVal, MidRange};

const RANDOM_SEED: u64 = 96251;

#[cfg(test)]
mod test;

fn main() {
    let t1 = vec![4, 7, 6, 5, 2];
    get_min_not_in_set(t1);

    let t2 = vec![4, 7, 6, 5, 2, 1, 0];
    get_min_not_in_set(t2);

    let mut ran_gen = RandomGen::new(RANDOM_SEED);

    let results: Vec<ComputeTime> = (1_000_000..9_000_000)
        .step_by(1_000_000)
        .map(|size| compute_time(ran_gen.make_vec(size)))
        .collect();

    println!("{:?}", results);
    println!("{} -> {}", results[0].size, results[0].duration.as_millis());
}

fn get_min_not_in_set(vals: Vec<u32>) -> u32 {
    let mut curr_min: u32 = u32::MAX;
    let mut consecutives: HashMap<u32, RangeDef> = HashMap::new();
    for val in vals {
        if curr_min > val {
            curr_min = val;
        }
        if !consecutives.contains_key(&val) {
            let u_bound = match consecutives.get(&(val + 1)) {
                Some(range_def) => match range_def {
                    MidRange => val,
                    EndVal(r) => r.high,
                },
                None => val,
            };

            let l_bound = if val == 0 {
                0
            } else {
                match consecutives.get(&(val - 1)) {
                    Some(range_def) => match range_def {
                        MidRange => val,
                        EndVal(r) => r.low,
                    },
                    None => val,
                }
            };

            consecutives.insert(u_bound, EndVal(RangeInclusive::new(l_bound, u_bound)));
            consecutives.insert(l_bound, EndVal(RangeInclusive::new(l_bound, u_bound)));

            if val + 1 < u_bound {
                consecutives.insert(val + 1, MidRange);
            }

            if val > 0 && val - 1 > l_bound {
                consecutives.insert(val - 1, MidRange);
            }

            if val > l_bound && val < u_bound {
                consecutives.insert(val, MidRange);
            }
        }
    }
    println!("{:#?}", consecutives);
    println!("curr_min: {}", curr_min);

    if curr_min > 0 {
        0
    } else {
        match &consecutives[&0] {
            MidRange => panic!("Zero cannot be midrange in consecutive numbers"),
            EndVal(r) => r.high + 1,
        }
    }
}

#[derive(Debug)]
enum RangeDef {
    MidRange,
    EndVal(RangeInclusive),
}

#[derive(Debug)]
struct RangeInclusive {
    low: u32,
    high: u32,
}

impl RangeInclusive {
    fn new(low: u32, high: u32) -> Self {
        RangeInclusive { low, high }
    }
}

fn compute_time(nums: Vec<u32>) -> ComputeTime {
    // black_box hints to the compiler to prevent optimizer
    // re-ordering and messing up the timings
    let time_inst = black_box(Instant::now());
    black_box(nums.iter().min());
    let duration = black_box(time_inst.elapsed());
    ComputeTime {
        size: nums.len().try_into().unwrap(),
        duration,
    }
}

#[derive(Debug)]
struct ComputeTime {
    size: u32,
    duration: Duration,
}

#[derive(Debug)]
struct RandomGen {
    rng: StdRng,
}

impl RandomGen {
    fn new(seed: u64) -> Self {
        RandomGen {
            rng: StdRng::seed_from_u64(seed),
        }
    }

    fn make_vec(&mut self, length: usize) -> Vec<u32> {
        let i_length: u32 = length.try_into().unwrap();
        let distr = Uniform::from(0u32..i_length / 2);
        (&mut self.rng).sample_iter(distr).take(length).collect()
    }
}

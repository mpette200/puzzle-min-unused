use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use RangeDef::{EdgeVal, MidRange};

const RANDOM_SEED: u64 = 96251;

#[cfg(test)]
mod test;

fn main() {
    let mut ran_gen = RandomGen::new(RANDOM_SEED);

    let random_lists: Vec<Vec<u32>> = (200_000..1_700_000)
        .step_by(200_000)
        .map(|size| ran_gen.make_vec(size))
        .collect();

    let results_1: Vec<ComputeTime> = random_lists
        .iter()
        .map(|val_set| compute_time(val_set, get_min_not_in_list_via_sort))
        .collect();
    println!("{:?}", results_1);

    println!();
    let results_2: Vec<ComputeTime> = random_lists
        .iter()
        .map(|val_set| compute_time(val_set, get_min_not_in_list_via_hash))
        .collect();
    println!("{:?}", results_2);

    // avoid unused warnings
    results_1[0].size;
    results_1[0].duration;
}

fn get_min_not_in_list_via_sort(vals: &Vec<u32>) -> u32 {
    let mut sorted = vals.clone();
    sorted.sort_unstable();
    if sorted.is_empty() || sorted[0] > 0 {
        0
    } else {
        (0..sorted.len())
            .skip_while(|i| i + 1 < sorted.len() && sorted[i + 1] - sorted[*i] <= 1)
            .next()
            .map(|i| sorted[i] + 1)
            .unwrap()
    }
}

fn get_min_not_in_list_via_hash(vals: &Vec<u32>) -> u32 {
    let mut curr_min: u32 = u32::MAX;
    let mut consecutives: HashMap<u32, RangeDef> = HashMap::with_capacity(vals.len());
    for val in vals {
        if curr_min > *val {
            curr_min = *val;
        }
        if !consecutives.contains_key(&val) {
            let u_bound = match consecutives.get(&(val + 1)) {
                Some(range_def) => match range_def {
                    MidRange => *val,
                    EdgeVal(r) => r.high,
                },
                None => *val,
            };

            let l_bound = if *val == 0 {
                0
            } else {
                match consecutives.get(&(val - 1)) {
                    Some(range_def) => match range_def {
                        MidRange => *val,
                        EdgeVal(r) => r.low,
                    },
                    None => *val,
                }
            };

            consecutives.insert(u_bound, EdgeVal(RangeInclusive::new(l_bound, u_bound)));
            consecutives.insert(l_bound, EdgeVal(RangeInclusive::new(l_bound, u_bound)));

            if val + 1 < u_bound {
                consecutives.insert(val + 1, MidRange);
            }

            if *val > 0 && val - 1 > l_bound {
                consecutives.insert(val - 1, MidRange);
            }

            if *val > l_bound && *val < u_bound {
                consecutives.insert(*val, MidRange);
            }
        }
    }

    if curr_min > 0 {
        0
    } else {
        match &consecutives[&0] {
            MidRange => panic!("Zero cannot be midrange in consecutive numbers"),
            EdgeVal(r) => r.high + 1,
        }
    }
}

#[derive(Debug)]
enum RangeDef {
    MidRange,
    EdgeVal(RangeInclusive),
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

fn compute_time(nums: &Vec<u32>, func_to_call: fn(&Vec<u32>) -> u32) -> ComputeTime {
    // Need to prevent the compiler from re-ordering statements.
    // Arranged so result depends on time_inst
    // and duration depends on result.
    let time_inst = Instant::now();
    let result = (time_inst, func_to_call(nums));
    let duration = result.0.elapsed();
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

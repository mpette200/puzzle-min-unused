use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::ops::Range;
use std::time::{Duration, Instant};
use RangeDef::{EdgeVal, MidRange};

const PATH_IMAGES: &str = "images/";
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

    plot_compute_time(
        &concat_str(PATH_IMAGES, "chart_01_via_sort.png"),
        &results_1,
        "Algorithm Based on Sort",
    )
    .unwrap();

    plot_compute_time(
        &concat_str(PATH_IMAGES, "chart_02_via_hash.png"),
        &results_2,
        "Algorithm Based on Hash Table",
    )
    .unwrap();

}

fn get_min_not_in_list_via_sort(vals: &Vec<u32>) -> u32 {
    let mut sorted = vals.clone();
    sorted.sort();
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

fn compute_time<F>(nums: &Vec<u32>, func_to_call: F) -> ComputeTime
where
    F: Fn(&Vec<u32>) -> u32,
{
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

#[derive(Debug, Clone, Copy)]
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

fn concat_str(a: &str, b: &str) -> String {
    let mut out = String::from(a);
    out.push_str(b);
    out
}

fn plot_compute_time(
    filename: &str,
    data: &Vec<ComputeTime>,
    title: &str,
) -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new(filename, (640, 480)).into_drawing_area();
    root_area.fill(&WHITE)?;
    let root_area = root_area.titled(title, ("sans-serif", 24))?;

    let x_range = range_by_key(&data, |x| x.size).expect("Got empty data");
    let y_range = range_by_key(&data, |x| x.duration.as_millis()).expect("Got empty data");

    let mut cc = ChartBuilder::on(&root_area)
        .margin(5)
        .set_all_label_area_size(50)
        .build_cartesian_2d(x_range, y_range)?;

    cc.configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .max_light_lines(5)
        .x_desc("Length of List")
        .y_desc("Milliseconds")
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    cc.draw_series(PointSeries::of_element(
        data.iter().map(|d| (d.size, d.duration.as_millis())),
        5i32,
        RED.filled(),
        &Circle::new,
    ))?;

    root_area
        .present()
        .expect(&format!("Unable to write result: {}.", filename));
    println!("Result has been saved to {}", filename);

    Ok(())
}

fn range_by_key<B, T, F>(data: &[T], mut f: F) -> Option<Range<B>>
where
    B: Ord + Copy,
    F: FnMut(&T) -> B,
{
    let vals: Vec<B> = data.iter().map(&mut f).collect();
    vals.iter()
        .min()
        .and_then(|min_val| vals.iter().max().map(|max_val| *min_val..*max_val))
}

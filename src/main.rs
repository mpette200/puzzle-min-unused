use plotters::prelude::*;
use rand::distributions::Uniform;
use rand::prelude::*;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use RangeDef::{EdgeVal, MidRange};

const CHART_FILE_NAME: &str = "chart_01_hello_world.png";
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

    let _ = make_plot();

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

fn make_plot() -> Result<(), Box<dyn Error>> {
    let root_area = BitMapBackend::new(CHART_FILE_NAME, (1024, 768)).into_drawing_area();

    root_area.fill(&WHITE)?;

    let root_area = root_area.titled("Image Title", ("sans-serif", 60))?;

    let (upper, lower) = root_area.split_vertically(512);

    let x_axis = (-3.4f32..3.4).step(0.1);

    let mut cc = ChartBuilder::on(&upper)
        .margin(5)
        .set_all_label_area_size(50)
        .caption("Sine and Cosine", ("sans-serif", 40))
        .build_cartesian_2d(-3.4f32..3.4, -1.2f32..1.2f32)?;

    cc.configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .disable_mesh()
        .x_label_formatter(&|v| format!("{:.1}", v))
        .y_label_formatter(&|v| format!("{:.1}", v))
        .draw()?;

    cc.draw_series(LineSeries::new(x_axis.values().map(|x| (x, x.sin())), &RED))?
        .label("Sine")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], RED));

    cc.draw_series(LineSeries::new(
        x_axis.values().map(|x| (x, x.cos())),
        &BLUE,
    ))?
    .label("Cosine")
    .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], BLUE));

    cc.configure_series_labels().border_style(BLACK).draw()?;

    /*
    // It's possible to use a existing pointing element
     cc.draw_series(PointSeries::<_, _, Circle<_>>::new(
        (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
        5,
        Into::<ShapeStyle>::into(&RGBColor(255,0,0)).filled(),
    ))?;*/

    // Otherwise you can use a function to construct your pointing element yourself
    cc.draw_series(PointSeries::of_element(
        (-3.0f32..2.1f32).step(1.0).values().map(|x| (x, x.sin())),
        5,
        ShapeStyle::from(&RED).filled(),
        &|coord, size, style| {
            EmptyElement::at(coord)
                + Circle::new((0, 0), size, style)
                + Text::new(format!("{:?}", coord), (0, 15), ("sans-serif", 15))
        },
    ))?;

    let drawing_areas = lower.split_evenly((1, 2));

    for (drawing_area, idx) in drawing_areas.iter().zip(1..) {
        let mut cc = ChartBuilder::on(drawing_area)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .margin_right(20)
            .caption(format!("y = x^{}", 1 + 2 * idx), ("sans-serif", 40))
            .build_cartesian_2d(-1f32..1f32, -1f32..1f32)?;
        cc.configure_mesh()
            .x_labels(5)
            .y_labels(3)
            .max_light_lines(4)
            .draw()?;

        cc.draw_series(LineSeries::new(
            (-1f32..1f32)
                .step(0.01)
                .values()
                .map(|x| (x, x.powf(idx as f32 * 2.0 + 1.0))),
            &BLUE,
        ))?;
    }

    // To avoid the IO failure being ignored silently, we manually call the present function
    root_area.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", CHART_FILE_NAME);
    Ok(())
}

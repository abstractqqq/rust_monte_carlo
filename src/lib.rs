use ndarray::{Array2, Axis, Array1};
use rand::thread_rng;
use rand::seq::SliceRandom;
use rayon::prelude::*; 

fn generate_sample(
    total_features:usize
    , subsample_amt:usize
    , n_estimators:usize
) -> Array2<u32>{
    let mut sample_pool:Array2<u32> = Array2::from_elem((n_estimators, total_features), 0);
    
    for mut v in sample_pool.axis_iter_mut(Axis(0)) {
        let mut vec:Vec<usize> = (0..total_features).collect();
        vec.shuffle(&mut thread_rng());
        for i in 0..total_features {
            v[i] = (vec[i] < subsample_amt) as u32
        }
    }
    sample_pool
}

fn generate_sample_2(
    total_features:usize
    , subsample_amt:usize
    , n_estimators:usize
) -> Array2<u32>{
    let mut sample_pool:Array2<u32> = Array2::from_elem((n_estimators, total_features), 0);

    sample_pool.axis_iter_mut(Axis(0)).par_bridge().for_each(|mut v|{
        let mut vec:Vec<usize> = (0..total_features).collect();
        vec.shuffle(&mut thread_rng());
        for i in 0..total_features {
            v[i] = (vec[i] < subsample_amt) as u32
        }
    });
    sample_pool
}

fn monte_carlo_prob_est(
    n_trials:usize
    , total_features:usize
    , subsample_amt:usize
    , n_estimators:usize
    , n_times:u32
) -> f32 {
    let mut count:u32 = 0;
    for _ in 0..n_trials {
        let times:Array1<u32> = generate_sample(total_features, subsample_amt, n_estimators)
            .sum_axis(Axis(0));
        
        let mut success:u32 = 1;
        for x in times.into_iter() {
            if x < n_times {
                success = 0;
                break
            }
        }
        count += success;
    }
    return count as f32 / n_trials as f32
}

#[inline]
fn _one_trial(
    total_features:usize
    , subsample_amt:usize
    , n_estimators:usize
    , n_times:u32
) -> u32 {
    let times:Array1<u32> = generate_sample(total_features, subsample_amt, n_estimators)
    .sum_axis(Axis(0));
    let mut success:u32 = 1;
    for x in times.into_iter() {
        if x < n_times {
            success = 0;
            break
        }
    }
    success
}

fn par_monte_carlo_prob_est(n_trials:usize
    , total_features:usize
    , subsample_amt:usize
    , n_estimators:usize
    , n_times:u32
) -> f32 {

    // let mut results:Vec<u32> = Vec::with_capacity(n_trials);
    let success_count:u32 = (0..n_trials).into_par_iter()
        .map(|_| _one_trial(total_features, subsample_amt, n_estimators, n_times))
        .reduce(|| 0, |a,b| a + b);
    //collect_into_vec(&mut results);

    success_count as f32 / n_trials as f32
}

pub fn run_experiment(
    target_times:u32
    , total_features:usize
    , subsample_amt:usize
    , n_trials:usize
    , threshold:f32
) -> usize {

    let mut n_estimators:usize = 0;
    let mut p:f32 = 0.;
    while p < threshold {
        n_estimators += 1;
        p = par_monte_carlo_prob_est(n_trials, total_features, subsample_amt, n_estimators, target_times);
    }

    n_estimators

}

use rust_monte_carlo::run_experiment;

fn main(){
    let target_times:u32 = 6;
    let total_features:usize = 100;
    let subsample_amt:usize = 60;
    let n_trials:usize = 500;
    let threshold:f32 = 0.95;
    let n:usize = run_experiment(
        target_times
        , total_features
        , subsample_amt
        , n_trials
        , threshold
    );
    println!("Experiment set up:\n total_features = {},\n subsample_amt = {},\n threshold: {}"
    , total_features, subsample_amt, threshold);
    println!(" Monte carlo trials to estimate probability: {}", n_trials);
    println!("Result: we need {} estimators to have 95% confidence that all features are considered at least {} times.", n, target_times)
}


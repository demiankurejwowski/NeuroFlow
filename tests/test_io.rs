extern crate neuroflow;
extern crate time;
extern crate rand;

use std::path::Path;
use std::fs::remove_file;

use neuroflow::FeedForward;
use neuroflow::data::{DataSet, Extractable};

use rand::distributions::IndependentSample;
use rand::distributions::range::Range;
use rand::distributions::normal::Normal;

use neuroflow::activators;
use neuroflow::estimators;

use neuroflow::io::save;


#[test]
fn saving_of_neural_net(){
    const ALLOWED_ERROR: f64 = 0.1; // Max allowed error is 10%
    let mut nn = FeedForward::new(&[2, 2, 1]);
    let mut data = DataSet::new();

    data.push(&[0f64, 0f64], &[0f64]);
    data.push(&[1f64, 0f64], &[1f64]);
    data.push(&[0f64, 1f64], &[1f64]);
    data.push(&[1f64, 1f64], &[0f64]);

    nn.activation(activators::Type::Tanh)
        .learning_rate(0.05)
        .momentum(0.15)
        .train(&data, 5_000);

    save(&nn, "test.nn");

    let p = Path::new("test.nn");
    assert!(p.exists());
    if p.exists(){
        remove_file(p);
    }
}
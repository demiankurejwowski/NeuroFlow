mod network;

use network::NeuralNet;

extern crate time;

fn main() {
    println!("Application starts!");

    let mut nn = NeuralNet::new(vec![2, 1], 0.1, 0.1);

    let sc = &[
        (&[0f64, 0f64], &[0f64]),
        (&[1f64, 0f64], &[1f64]),
        (&[0f64, 1f64], &[1f64]),
        (&[1f64, 1f64], &[0f64]),
    ];
    let mut k = 0;

    nn.print(network::Type::InducedField);
    nn.print(network::Type::Y);
    nn.print(network::Type::Deltas);
    nn.print(network::Type::Weights);

    let prev = time::now_utc();
    for _ in 0..100{
        if k == 4{
            k = 0;
        }
        nn.fit(sc[k].0, sc[k].1);
        k += 1;
    }

    nn.print(network::Type::Deltas);
    nn.print(network::Type::Weights);

    println!("Res 1: [{}, {}], [{}] -> [{}]", sc[0].0[0], sc[0].0[1], sc[0].1[0], nn.calc(sc[0].0)[0]);
    println!("Res 2: [{}, {}], [{}] -> [{}]", sc[1].0[0], sc[1].0[1], sc[1].1[0], nn.calc(sc[1].0)[0]);
    println!("Res 3: [{}, {}], [{}] -> [{}]", sc[2].0[0], sc[2].0[1], sc[2].1[0], nn.calc(sc[2].0)[0]);
    println!("Res 4: [{}, {}], [{}] -> [{}]", sc[3].0[0], sc[3].0[1], sc[3].1[0], nn.calc(sc[3].0)[0]);

    println!("\nSpend time: {}", (time::now_utc() - prev));
    println!("Application stops!");
}

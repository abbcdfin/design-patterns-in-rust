extern crate abstract_factory;

use std::env;
use abstract_factory::Config;
use abstract_factory::run;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::parse(&args).unwrap();

    run(config);
}

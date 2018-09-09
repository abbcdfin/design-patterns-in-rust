pub struct Config<'a> {
    factory: &'a str,
}

impl <'a> Config <'a> {
    
    pub fn parse(args: &'a Vec<String>) -> Result<Config<'a>, &'static str> {
        if args.len() < 2 {
            return Err("Not enough parameters");
        }

        Ok(
            Config {
                factory : &args[1],
            }
        )
    }
}

pub fn run(config: Config) {
    println!("Setup environment with product from {}", config.factory);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

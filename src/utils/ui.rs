use colored::*;

pub const BANNER: &str = "
                                         
 _   _     _   _                         
| |_| |___| |_| |_ ___ ___       ___ ___ 
| . | | . | . | . | -_|  _|     |  _|_ -|
|___|_|___|___|___|___|_|  _____|_| |___|
                          |_____|        
          Rust Version - 1.0.0
";

pub fn banner() {
    println!("{}", BANNER.blue().bold());
}
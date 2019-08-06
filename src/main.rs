use std::error::Error;

use duplicate_finder::run;

fn main() -> Result<(), Box<dyn Error>> {
    run(vec!["./"])
}

use std::{error::Error, time::Instant};

use runner::Runner;

mod parsing;
mod q01;
mod q02;
mod q03;
mod q04;
mod q05;
mod q06;
mod q07;
mod q08;
mod q09;
mod q10;
mod runner;

macro_rules! run {
    ($runner:ident $mod:ident) => {
        let name = stringify!($mod);
        $runner.run_test(&format!("{name}::a"), $mod::a, &format!("{name}.real"));
        $runner.run_test(&format!("{name}::b"), $mod::b, &format!("{name}.real"));
    };
}

fn main() -> Result<(), Box<dyn Error>> {
    let runner = Runner::with_data_dir("data")?;
    let start = Instant::now();
    run!(runner q01);
    run!(runner q02);
    run!(runner q03);
    run!(runner q04);
    run!(runner q05);
    run!(runner q06);
    run!(runner q07);
    run!(runner q08);
    run!(runner q09);
    run!(runner q10);
    // runner.run_test("test", q10::b, "test");
    let elapsed = start.elapsed();
    println!("Ran all puzzles in {}ms", elapsed.as_millis());
    Ok(())
}

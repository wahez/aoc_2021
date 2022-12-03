use std::error::Error;

use runner::Runner;

mod parsing;
mod q01;
mod q02;
mod q03;
mod runner;

fn main() -> Result<(), Box<dyn Error>> {
    let runner = Runner::with_data_dir("data")?;
    runner.run_test("q01::a", q01::a, "q01.real");
    runner.run_test("q01::b", q01::b, "q01.real");
    runner.run_test("q02::a", q02::a, "q02.real");
    runner.run_test("q02::b", q02::b, "q02.real");
    runner.run_test("q03::a", q03::a, "q03.real");
    runner.run_test("q03::b", q03::b, "q03.real");
    Ok(())
}

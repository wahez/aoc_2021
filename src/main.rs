use std::{fmt::Display, fs::File, io::BufReader, path::Path};

mod parsing;
mod q01;
mod q02;

pub fn run_test<R, E, F>(name: &str, func: F, input_path: impl AsRef<Path>)
where
    R: Display,
    E: Display,
    F: FnOnce(BufReader<File>) -> Result<R, E>,
{
    let Ok(input_file) = File::open(&input_path) else {
        eprintln!("Could not open file {}", input_path.as_ref().display());
        return;
    };
    let input_buf = BufReader::new(input_file);
    match func(input_buf) {
        Err(e) => println!("Error solving puzzle {name}: {e}"),
        Ok(r) => println!("{name} {} {r}", input_path.as_ref().display()),
    }
}

fn main() {
    run_test("q01::a", q01::a, "data/q01.real");
    run_test("q01::b", q01::b, "data/q01.real");
    run_test("q02::a", q02::a, "data/q02.real");
    run_test("q02::b", q02::b, "data/q02.real");
}

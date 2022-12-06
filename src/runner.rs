use itertools::Itertools;
use std::{
    collections::HashMap,
    error::Error,
    fmt::Display,
    fs::File,
    io::BufRead,
    io::BufReader,
    path::{Path, PathBuf},
    time::Instant,
};

pub struct Runner {
    data_dir: PathBuf,
    answers: HashMap<String, String>,
}

impl Runner {
    pub fn with_data_dir(data_dir: impl AsRef<Path>) -> Result<Runner, Box<dyn Error>> {
        let data_dir = data_dir.as_ref().to_path_buf();
        let path = data_dir.join("answers");
        let buf_read = BufReader::new(File::open(path)?);
        let answers = buf_read
            .lines()
            .filter_ok(|line| !line.is_empty())
            .map_ok(|line| {
                let (func, input, answer) = line.splitn(3, ' ').tuples().next().unwrap();
                (format!("{func} {input}"), answer.to_string())
            })
            .try_collect()?;
        Ok(Runner { data_dir, answers })
    }

    pub fn run_test<R, E, F>(&self, name: &str, func: F, filename: impl AsRef<Path>)
    where
        R: Display,
        E: Display,
        F: FnOnce(BufReader<File>) -> Result<R, E>,
    {
        let full_path = self.data_dir.join(&filename);
        let Ok(input_file) = File::open(&full_path) else {
            eprintln!("Could not open file {}", full_path.display());
            return;
        };
        let input = BufReader::new(input_file);
        let start = Instant::now();
        let result = func(input);
        let elapsed = start.elapsed();
        let name = format!("{name} {}", full_path.display());
        match result {
            Err(e) => eprintln!("{name} had an error: {e}"),
            Ok(r) => {
                let result = format!("{r}");
                match self.answers.get(&name) {
                    None => println!("{name} {result}"),
                    Some(a) if *a == result => {
                        println!("{name} solved in {}us", elapsed.as_micros())
                    }
                    Some(a) => eprintln!("{name} computed {result}, expected {a}"),
                }
            }
        }
    }
}

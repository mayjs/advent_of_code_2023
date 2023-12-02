use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::marker::PhantomData;
use std::path::Path;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum InputError<T> {
    #[error("IO error")]
    IoError(io::Error),
    #[error("Conversion error")]
    ConversionError(#[from] T),
}

pub fn stream_items<I, T>(input: I) -> impl Iterator<Item = Result<T, InputError<T::Err>>>
where
    I: Read,
    T: FromStr,
{
    BufReader::new(input).lines().map(|maybe_line| {
        maybe_line
            .map_err(|e| InputError::IoError(e))
            .and_then(|l| Ok(l.parse()?))
    })
}

pub fn stream_items_from_file<P: AsRef<Path>, T: FromStr>(
    path: P,
) -> std::io::Result<impl Iterator<Item = Result<T, InputError<T::Err>>>> {
    Ok(stream_items(File::open(path)?))
}

pub struct BlockCollector<T, I, F> {
    input: T,
    predicate: F,
    _phantom: PhantomData<I>,
}

impl<T, I, F> BlockCollector<T, I, F> {
    fn new(input: T, predicate: F) -> Self {
        BlockCollector {
            input,
            predicate,
            _phantom: PhantomData,
        }
    }
}

impl<T, I, F> Iterator for BlockCollector<T, I, F>
where
    T: Iterator<Item = I>,
    F: FnMut(&I) -> bool,
{
    type Item = Vec<I>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut group = Vec::new();
        loop {
            match self.input.next() {
                Some(line) => {
                    if (self.predicate)(&line) {
                        return Some(group);
                    } else {
                        group.push(line);
                    }
                }
                None => {
                    if group.len() > 0 {
                        return Some(group);
                    } else {
                        return None;
                    }
                }
            }
        }
    }
}

pub fn stream_file_blocks<P: AsRef<Path>>(
    path: P,
) -> std::io::Result<impl Iterator<Item = Vec<String>>> {
    let file = File::open(path)?;
    let lines = BufReader::new(file).lines().filter_map(Result::ok);
    Ok(BlockCollector::new(lines, |line: &String| line.len() == 0))
}

pub mod test_helpers {
    use std::{fmt::Display, fs::File, io::Write, path::Path};
    use tempfile::{tempdir, TempDir};

    pub fn create_line_file<T: Display, I: Iterator<Item = T>>(
        inp: I,
        dir: Option<TempDir>,
    ) -> (TempDir, impl AsRef<Path>) {
        let dir = dir.unwrap_or_else(|| tempdir().expect("Failed to create tempdir"));
        let filepath = dir.path().join("tempinput.txt");
        let mut file = File::create(&filepath).expect("Could not create file");
        inp.for_each(|item| writeln!(file, "{}", item).expect("Could not write to file"));
        (dir, filepath)
    }

    pub fn create_example_file(inp: &str, dir: Option<TempDir>) -> (TempDir, impl AsRef<Path>) {
        let dir = dir.unwrap_or_else(|| tempdir().expect("Failed to create tempdir"));
        let filepath = dir.path().join("tempinput.txt");
        let mut file = File::create(&filepath).expect("Could not create file");
        write!(file, "{}", inp).expect("Could not write to file");
        (dir, filepath)
    }
}

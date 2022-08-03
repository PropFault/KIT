use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

pub trait Resource{
    fn read(&self) -> Box<dyn Read>;
    fn write(&mut self) -> Box<dyn Write>;
}

pub struct FileResource{
    pub path: Box<Path>
}

impl Resource for FileResource{
    fn read(&self) -> Box<dyn Read> {
        Box::new(BufReader::new(File::open(self.path.clone()).unwrap()))
    }

    fn write(&mut self) -> Box<dyn Write> {
        Box::new(BufWriter::new(File::open(self.path.clone()).unwrap()))
    }
}

impl FileResource{
    pub fn new(path: Box<Path>) -> FileResource{
        FileResource{ path }
    }
}
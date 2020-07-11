use std::io;

pub mod ssh;

pub trait Transport {
    fn read_xml(&mut self) -> io::Result<String>;
    fn write_xml(&mut self, data: &str) -> io::Result<()>;
}

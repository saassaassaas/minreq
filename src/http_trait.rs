use std::io;

pub trait  ReadWrite:  io::Read + io::Write + Send + Sync {}

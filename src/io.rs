use std::cell::RefCell;
use std::fs::File;
use std::io::*;
use std::io::BufWriter;

pub trait IoPort {

	/// Reads one byte from the port
	fn read(&self) -> u8;

	/// Writes one byte to the port
	fn write(&self, value: u8) -> ();

}

pub struct FdWriteIoPort {
	writer: RefCell<BufWriter<File>>
}


impl FdWriteIoPort {

	pub fn new(desc: File) -> FdWriteIoPort {

		let bw = BufWriter::new(desc);
		FdWriteIoPort {
			writer: RefCell::new(bw)
		}

	}

}

impl IoPort for FdWriteIoPort {

	fn read(&self) -> u8 {
		0 // Do nothing.
	}

	fn write(&self, value: u8) -> () {
		match self.writer.borrow_mut().write(&[value]) {
			Ok(_) => (), Err(_) => ()
		}
	}

}

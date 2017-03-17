use std::fs::File;
use std::io::*;
use std::io::BufWriter;
use std::io::prelude::*;

pub trait IoPort {

	/// Reads one byte from the port
	fn read(&self) -> u8;

	/// Writes one byte to the port
	fn write(&self, value: u8) -> ();

}

struct FdWriteIoPort<'a> {
	writer: &'a mut BufWriter<File>
}

impl<'a> FdWriteIoPort<'a> {

	pub fn new<'b>(desc: File) -> FdWriteIoPort<'b> {

		let mut bw = BufWriter::new(desc);
		FdWriteIoPort {
			writer: panic!("this doesn't work") // FIXME Lifetimes suck.
		}

	}

}

impl<'a> IoPort for FdWriteIoPort<'a> {

	fn read(&self) -> u8 {
		0 // Do nothing.
	}

	fn write(&self, value: u8) -> () {
		// FIXME self.writer.write(&[value]);
	}

}

/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate v4l;

use std::fs::File;
use std::io::prelude::*;

use v4l::device::List;
use v4l::prelude::*;

fn main() {
	let list = List::new();

	if list.count() == 0 {
		println!("No available device !");
		return;
	}

	let list = List::new();

	println!("Available devices :");
	for dev in list {
		println!("    {}: {}", dev.index().unwrap(), dev.name().unwrap());
	}
	println!("");

	let mut dev = CaptureDevice::new(0).unwrap();
	let fmt = dev.format().unwrap();

	println!("Using format :\n{}", fmt);

	let mut stream = MmapStream::with_buffers(&mut dev, 4).expect("Failed to create buffer stream");

	loop {
		let frame = stream.next().unwrap();
		println!(
			"Buffer size: {}, seq: {}, timestamp: {}",
			frame.len(),
			frame.meta().sequence,
			frame.meta().timestamp
		);

		let mut file = File::create(format!("frame.jpg")).unwrap();
		file.write_all(frame.data()).unwrap();
	}
}

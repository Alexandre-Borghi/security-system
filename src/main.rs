/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate opencv;

use opencv::core::Mat;
use opencv::core::MatExprTrait;
use opencv::videoio::prelude::*;
use opencv::videoio::VideoCapture;

fn main() {
	let mut dev = VideoCapture::from_file("/dev/video0", opencv::videoio::CAP_ANY)
		.expect("Unable to get camera");

	let width = dev
		.get(opencv::videoio::VideoCaptureProperties::CAP_PROP_FRAME_WIDTH as i32)
		.expect("Unable to get camera width") as i32;
	let height = dev
		.get(opencv::videoio::VideoCaptureProperties::CAP_PROP_FRAME_HEIGHT as i32)
		.expect("Unable to get camera height") as i32;

	println!("Available camera:");
	println!("    Width: {}", width);
	println!("    Height: {}", height);

	let mut last_img = Mat::zeros(height, width, opencv::core::CV_8UC3)
		.unwrap()
		.to_mat()
		.unwrap();

	loop {
		let is_ready = dev.grab().expect("Unable to get camera status");

		if !is_ready {
			continue;
		}

		let mut img = Mat::default().unwrap();
		dev.retrieve(&mut img, 0)
			.expect("Unable to get frame from camera");
		let mut diff = Mat::zeros(height, width, opencv::core::CV_8UC3)
			.unwrap()
			.to_mat()
			.unwrap();

		opencv::core::absdiff(&img, &last_img, &mut diff).unwrap();

		opencv::imgcodecs::imwrite("frame.jpg", &diff, &opencv::core::Vector::default()).unwrap();

		last_img = Mat::copy(&img).unwrap();
	}
}

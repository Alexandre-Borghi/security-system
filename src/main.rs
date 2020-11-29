/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate opencv;

use opencv::core::Mat;
use opencv::core::MatExprTrait;
use opencv::videoio::prelude::*;
use opencv::videoio::VideoCapture;

fn main() {
	// Get camera

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

	// Create window

	opencv::highgui::named_window("test", opencv::highgui::WINDOW_AUTOSIZE).unwrap();

	// Loop

	let mut last_img = Mat::zeros(height, width, opencv::core::CV_8UC1)
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

		let mut gray = Mat::zeros(height, width, opencv::core::CV_8UC3)
			.unwrap()
			.to_mat()
			.unwrap();

		let mut diff = Mat::zeros(height, width, opencv::core::CV_8UC3)
			.unwrap()
			.to_mat()
			.unwrap();

		let mut thresh_diff = Mat::zeros(height, width, opencv::core::CV_8UC3)
			.unwrap()
			.to_mat()
			.unwrap();

		opencv::imgproc::cvt_color(
			&img,
			&mut gray,
			opencv::imgproc::ColorConversionCodes::COLOR_BGR2GRAY as i32,
			0,
		)
		.unwrap();

		opencv::core::absdiff(&gray, &last_img, &mut diff).unwrap();
		opencv::imgproc::threshold(
			&diff,
			&mut thresh_diff,
			127.,
			255.,
			opencv::imgproc::ThresholdTypes::THRESH_BINARY as i32,
		)
		.unwrap();

		opencv::imgcodecs::imwrite("frame.jpg", &thresh_diff, &opencv::core::Vector::default())
			.unwrap();

		last_img = Mat::copy(&gray).unwrap();
	}
}

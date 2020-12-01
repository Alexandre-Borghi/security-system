/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

extern crate lettre;
extern crate opencv;
extern crate serde;
extern crate serde_derive;
extern crate toml;

use std::fs::read_to_string;

use toml::value::Value;

use opencv::core::Mat;
use opencv::core::MatExprTrait;
use opencv::videoio::prelude::*;
use opencv::videoio::VideoCapture;

use lettre::smtp::ConnectionReuseParameters;
use lettre::{
	smtp::authentication::{Credentials, Mechanism},
	SendableEmail, SmtpClient, Transport,
};
use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Config {
	sender: Sender,
	contacts: Option<toml::value::Array>,
}

#[derive(Deserialize)]
struct Sender {
	email: String,
	password: String,
}

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

	// Create window and timer

	opencv::highgui::named_window("test", opencv::highgui::WINDOW_AUTOSIZE).unwrap();
	let mut start_time = std::time::Instant::now();
	let mut is_in_alert = false;
	let mut alert_frames = 0;

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

		if start_time.elapsed() < std::time::Duration::from_secs(1) {
			continue;
		}

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

		let white_pixels = opencv::core::sum_elems(&thresh_diff).unwrap()[0] / 255.;
		let white_pixels = white_pixels as i32;

		if !is_in_alert && white_pixels > 20 {
			println!("Movement detected with {} pixels !", white_pixels);
			is_in_alert = true
		} else if is_in_alert {
			if white_pixels > 20 {
				alert_frames += 1;
			} else {
				println!("Alert lasted {} seconds", alert_frames);
				is_in_alert = false;
				alert_frames = 0;
			}
		}

		// Send alerts

		if alert_frames == 4 {
			send_alert_email();
		}

		last_img = Mat::copy(&gray).unwrap();

		start_time = std::time::Instant::now();
	}
}

fn send_alert_email() {
	let config = read_to_string("config.toml").unwrap();
	let config: Config = toml::de::from_str(&config).unwrap();

	let mut email_addresses = vec![];

	if let Some(addresses) = config.contacts {
		for address in addresses {
			if let Value::String(str_addr) = address {
				email_addresses.push(str_addr);
			}
		}
	} else {
		println!("[INFO] No contacts given, not sending e-mails");
		return;
	}

	let body = format!(
		"
Alert !
Movement has been detected from your camera !
"
	);

	let mut email = lettre_email::EmailBuilder::new()
		.from((config.sender.email.clone(), "Security System"))
		.subject("Security alert !")
		.text(body);

	for addr in email_addresses {
		println!("[INFO] Sending email to : {}", addr); // TODO: Show that only when loading config file
		email = email.to(addr);
	}

	let email: SendableEmail = email.build().unwrap().into();

	// Connect to a remote server on a custom port
	let mut mailer = SmtpClient::new_simple("smtp.gmail.com")
		.unwrap()
		.credentials(Credentials::new(
			config.sender.email,
			config.sender.password,
		))
		.smtp_utf8(true)
		.authentication_mechanism(Mechanism::Plain)
		.connection_reuse(ConnectionReuseParameters::ReuseUnlimited)
		.transport();

	mailer.send(email).unwrap();
	mailer.close();
}

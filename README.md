# Security System

This program takes input from cameras, detects motion and sends an e-mail to given e-mail addresses when it detects motion.

It serves a web server and a web app used to change the settings and view informations.

---

It uses Rust because I want to learn Rust and I want the software to run fast.

It is published under the MPL 2.0 license.

It only works on Linux because it uses [v4l2](https://en.wikipedia.org/wiki/Video4Linux) to capture images from cameras.

---

## Building

To build this program, please install the following dependencies

### opencv

You will need the opencv development libraries to build this project. If your OS uses the APT package manager, you can install all opencv libs like that :

```shell
	$ sudo apt-get install libopencv-*
```

### libclang and clang

You will need to install a libclang package and clang binaries. I personally did this with APT :

```shell
	$ sudo apt-get install libclang-*
	$ sudo apt-get install clang
```

---

## How does it work ?

### November 29th, 2020

It continually compares the image from the camera and the last image it got from the camera. It computes the difference and applies a threshold, giving this type of images :

![](docs/gifs/gray-thresh-diff.gif)
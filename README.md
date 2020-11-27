# Security System

This program takes input from cameras, detects motion and sends an e-mail to given e-mail addresses when it detects motion.

It serves a web server and a web app used to change the settings and view informations.

---

It uses Rust because I want to learn Rust and I want the software to run fast.

It is published under the MPL 2.0 license.

It only works on Linux because it uses the [v4l](https://crates.io/crates/v4l) crate which uses [v4l2](https://en.wikipedia.org/wiki/Video4Linux).

---

## Building

To build this program, please install the following dependencies

### v4l2

You will need the v4l2 development library to build this prject. If your OS uses the APT package manager, you can install v4l2 like that :

```shell
	$ sudo apt install libv4l-dev
```

### libclang

You will need to install a libclang package. I personally did this with APT :

```shell
	$ sudo apt-get install libclang-*
```
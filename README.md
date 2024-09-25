<h1 align="center">filmweb-csv</h1>

[![Rust CI](https://github.com/wedkarz02/filmweb-csv/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/wedkarz02/filmweb-csv/actions/workflows/rust-ci.yml)
![GitHub License](https://img.shields.io/github/license/wedkarz02/filmweb-csv)

**Please note** that this project is under active development. It's mostly usable but may contain bugs, missing features, etc.

This README is not complete yet.

## Overview

TBA

## Table of Contents

1. [Requirements](#requirements)
2. [Download](#download)
3. [Getting Started](#getting-started)\
    3.1. [Getting the Cookie header](#getting-the-cookie-header)\
    3.2. [Setting up the enviroment](#setting-up-the-enviroment)\
    3.3. [Building](#building)
4. [Usage](#usage)
5. [License](#license)

## Requirements

→ [Rust](https://www.rust-lang.org/)\
→ [Cargo](https://doc.rust-lang.org/cargo/)\
→ [Filmweb account](https://www.filmweb.pl/)

This project was developed on the Ubuntu 20.04.6 operating system and will likely work on most Linux-based systems without issues. It should also work on Windows, but I haven't tested it. If you encounter any problems, feel free to open a Github Issue.

## Download

Download the source code using the ```git clone``` command:

```bash
$ git clone https://github.com/wedkarz02/filmweb-csv.git
```

Or use the *Download ZIP* option from the Github repository [page](https://github.com/wedkarz02/filmweb-csv.git).

## Getting Started

### Getting the Cookie header

TBA (remember to warn about keeping this cookie private)

### Setting up the enviroment

You need to create a  ```.env``` file in the root directory (The app will exit with an error message informing you about it if you don't). There's only one variable you should set: ```COOKIE_HEADER``` - it's the value you got in the previous step. It will likely be very long, make sure it's all there.

The ```.env``` file should look like this:

```
COOKIE_HEADER="actual-value"
```

Please note that the value is inside of quotation marks. This is necessary.

### Building

Build the application using ```cargo``` in debug mode:

```bash
$ cargo build
```

or in release mode:

```bash
$ cargo build --release
```

It's up to you whether to build in debug or release mode. It doesn't really matter since the main bottleneck of the application is the API fetching. If you don't mind slightly longer compilation time, go for the release mode for some extra runtime performance.

## Usage

*To be added after implementing a command line interface.*

## License

If not directly stated otherwise, everything in this project is under the MIT License. See the [LICENSE](https://github.com/wedkarz02/filmweb-csv/blob/main/LICENSE) file for more info.

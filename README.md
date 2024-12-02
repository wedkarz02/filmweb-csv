<!-- <h1 align="center">filmweb-csv</h1> -->

<div align="center">
    <h1>
        <code> filmweb-csv </code>
    </h1>
    <p>
        <strong>🚀 Effortless Filmweb data ➔ CSV fetching 🚀</strong>
    </p>
</div>

[![Rust CI](https://github.com/wedkarz02/filmweb-csv/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/wedkarz02/filmweb-csv/actions/workflows/rust-ci.yml)
![GitHub Release](https://img.shields.io/github/v/release/wedkarz02/filmweb-csv)
![GitHub License](https://img.shields.io/github/license/wedkarz02/filmweb-csv)

## Overview

```filmweb-csv``` is a CLI application that allows users to effortlessly fetch their personal data from [Filmweb](https://www.filmweb.pl/) (basically Polish IMDB, but with a cooler name) and export this data into CSV format for easy analysis or sharing.

Key features include:
 * Fetching rated and watchlisted items.
 * Support for movies, TV shows and video games.
 * Export to CSV for easy use with tools like Excel or python.

Personally, I use it to generate histograms and analyze the statistical distribution of my movie ratings.

**Disclaimer**: The Filmweb API has not been officially released, therefore it's not at all documented and might change at any time, which could break this app. If you encounter any issues, feel free to open a Github Issue and I'll do my best to resolve them quickly.

## Table of Contents

* [Requirements](#requirements)
* [Install](#install)
* [Download](#download)
* [Building](#building)
* [Getting Started](#getting-started)
    * [Security Considerations](#security-considerations)
    * [Getting the Cookie header](#getting-the-cookie-header)
* [Usage](#usage)
    * [Auth](#auth)
    * [Options](#options)
* [License](#license)

## Requirements

→ [Rust](https://www.rust-lang.org/)\
→ [Cargo](https://doc.rust-lang.org/cargo/)\
→ [Filmweb account](https://www.filmweb.pl/)

This project was developed on the Ubuntu 20.04.6 operating system and will likely work on most Linux-based systems without issues. It should also work on Windows, but I haven't tested it. If you encounter any problems, feel free to open a Github Issue.

## Install

You can install the application by building it from source or by downloading it from [crates.io](https://crates.io/crates/filmweb-csv). For the latter, use ```cargo```:
```bash
$ cargo install filmweb-csv
```

After installing from [crates.io](https://crates.io/crates/filmweb-csv) you can skip to the [Getting Started](#getting-started) section.\
If you'd like to build from source - keep reading.

## Download

Download the source code using the ```git clone``` command:

```bash
$ git clone https://github.com/wedkarz02/filmweb-csv.git
```

Or use the *Download ZIP* option from the Github repository [page](https://github.com/wedkarz02/filmweb-csv.git).

## Building

Build the application using ```cargo``` in debug mode:

```bash
$ cargo build
```

or in release mode:

```bash
$ cargo build --release
```

It's up to you whether to build in debug or release mode. It doesn't really matter since the main bottleneck of the application is the API fetching. If you don't mind slightly longer compilation time, go for the release mode for some extra runtime performance.

The binary is self-contained so you can easily copy / move / symlink it from the ```target/``` directory:

```bash
$ cp ./target/release/filmweb-csv ~/.local/bin
$ mv ./target/release/filmweb-csv ~/.local/bin
$ ln ./target/release/filmweb-csv ~/.local/bin
```

## Getting Started

### **Security Considerations**:
- **Cookies contain sensitive session information**, which could be used to impersonate you or access your account.
- You should **never** provide your cookies to unknown parties.

### Getting the Cookie header

This app requires you to authenticate via an API that unfortunately doesn't have a documented login process. To work around this, you need to get the 'Cookie' header, which is generated when you log into Filmweb.

How to get the Cookie header:
1. Open your browser and log into [Filmweb](https://www.filmweb.pl/).
2. In a new tab, navigate to [https://www.filmweb.pl/api/v1/logged/info](https://www.filmweb.pl/api/v1/logged/info), you should see your profile details on the page.
3. Open the DevTools by pressing ```F12```, ```Ctrl+Shift+I``` or other shortcut depending on your browser.
4. Navigate to ```Network``` tab and refresh the page.
5. Select the row with ```info``` as the Name.
6. Make sure you are in ```Headers``` tab and scroll down to the ```Request Headers``` section.
7. Find the ```Cookie``` parameter and copy it's value (without the *Cookie:* part, just the value). It will likely be very long, make sure it's all there.

I haven't found an easier way of authenticating. I will automate this process if they decide to release the API officially in the future.

## Usage

To run the application use ```cargo``` or run the compiled executable directly:

```bash
$ cargo run -- [OPTIONS]
$ filmweb-csv [OPTIONS]
```

The app defaults to fetching rated movies if no options were given.

This is a full copy of a help message, which you can also get by using the ```--help``` option:

```
$ filmweb-csv --help


Usage: filmweb-csv [OPTIONS]

Options:
      --fetch <FETCH>    Type of resource to fetch [default: movies] [possible values: movies, series, games]
      --from <FROM>      Fetch from rated or watchlist [default: rated] [possible values: rated, watchlist]
  -o, --output <OUTPUT>  Specify the output directory [default: ./exports/]
  -v, --verbose          Log more details to stdout
      --cookie <COOKIE>  Cookie header for authentication
      --save-cookie      Save the cookie header to ~/.filmweb-csv
  -h, --help             Print help
  -V, --version          Print version
```

### Auth

To authenticate the application client use the ```--cookie``` option and paste the cookie header in single quotes:

```bash
$ filmweb-csv [OPTIONS] --cookie '<HEADER VALUE>'
```

Single quotes are important because tokens might contain special characters like double quotes, semicolons and spaces.

Optionally you can use the ```--save-cookie``` flag to cache the tokens. That way you won't have to include the ```--cookie``` every time:

```bash
$ filmweb-csv [OPTIONS] --cookie '<HEADER VALUE>' --save-cookie
```

Tokens are saved to ```~/.filmweb-csv``` in plaintext and are valid for about 15 minutes.

### Options

Full list of options (assuming ```~/.filmweb-csv``` has valid tokens):

```bash
# Get rated movies:
$ filmweb-csv --fetch movies --from rated

# Get watchlisted movies:
$ filmweb-csv --fetch movies --from watchlist

# Get rated series:
$ filmweb-csv --fetch series --from rated

# Get watchlisted series:
$ filmweb-csv --fetch series --from watchlist

# Get rated video games:
$ filmweb-csv --fetch games --from rated

# Get watchlisted video games:
$ filmweb-csv --fetch games --from watchlist
```

Optionally you can include an ```--output``` or ```-o``` option to specify the output directory:

```bash
# Save the output to "./data/movies_rated.csv":
$ filmweb-csv --fetch movies --from rated --output data
$ filmweb-csv --fetch movies --from rated -o data
```

The output path defaults to ```exports``` if not provided.

To enable info logging use the ```--verbose``` or ```-v``` flag:

```bash
# Enable verbose logging to stdout:
$ filmweb-csv --fetch movies --from rated --verbose
```

Logging to stdout is disabled by default (except for error logs) but logging to a file is always on. To see those logs navigate to ```logs``` directory.

## License

If not directly stated otherwise, everything in this project is under the MIT License. See the [LICENSE](https://github.com/wedkarz02/filmweb-csv/blob/main/LICENSE) file for more info.

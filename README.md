<h1 align="center">filmweb-csv</h1>

[![Rust CI](https://github.com/wedkarz02/filmweb-csv/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/wedkarz02/filmweb-csv/actions/workflows/rust-ci.yml)
![GitHub License](https://img.shields.io/github/license/wedkarz02/filmweb-csv)

**Please note** that this project is under active development. It's mostly usable but may contain bugs, missing features, etc.

This README is not complete yet.

## Overview

TBA

## Table of Contents

* [Requirements](#requirements)
* [Download](#download)
* [Getting Started](#getting-started)
    * [Getting the Cookie header](#getting-the-cookie-header)
        * [Security Considerations](#security-considerations)
        * [Using a script](#1-using-a-script)
        * [Doing it manually](#2-doing-it-manually)
    * [Setting up the enviroment](#setting-up-the-enviroment)
    * [Building](#building)
* [Usage](#usage)
* [License](#license)

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

This app requires you to authenticate via an API that unfortunately doesn't have a documented login process. To work around this, you need to get the 'Cookie' header, which is generated when you log into Filmweb.

#### Security Considerations
- **Cookies contain sensitive session information**, which could be used to impersonate you or access your account.
- You should **never run scripts from untrusted sources** or provide your cookies to unknown parties.
- The script below is designed only to copy the cookies from your active session, and no data is sent anywhere—it's copied to your clipboard for use in this application, which you can (and should) verify yourself as the script is very simple.

#### 1. Using a script 

The ```getCookie.js``` script is designed to simplify the process of extracting and copying your session cookies so that you can use them in the application. This is necessary because the Filmweb API requires a valid 'Cookie' header for authorization.

How to copy the 'Cookie' header using the script:

1. Open your browser and log in to [Filmweb](https://www.filmweb.pl/).
2. Open the DevTools by pressing ```F12```, ```Ctrl+Shift+I``` or other shortcut depending on your browser.
3. Navigate to ```Console``` tab.
4. Paste the JavaScript snippet and press *Enter*.
5. Done, the 'Cookie' value is now in your clipboard.

This is the JavaScript you need to paste (also avaliable in ```scripts/getCookie.js``` file with a full disclaimer):

```js
/**
 * DISCLAIMER: ...
 */

(function () {
    const cookies = document.cookie;
    const textarea = document.createElement("textarea");
    textarea.value = cookies;
    document.body.appendChild(textarea);
    textarea.select();
    document.execCommand("copy");
    document.body.removeChild(textarea);
})();

```

#### 2. Doing it manually

If, for any reason, you don't want to use the JavaScript method, here's how to do it manually:

1. Open your browser and log in to [Filmweb](https://www.filmweb.pl/).
2. In a new tab, navigate to [https://filmweb.pl/api/v1/](https://filmweb.pl/api/v1/), you should see ```Filmweb API :)``` text on the page.
3. Open the DevTools by pressing ```F12```, ```Ctrl+Shift+I``` or other shortcut depending on your browser.
4. Navigate to ```Network``` tab and refresh the page.
5. Select the row with ```v1/``` as the Name.
6. Make sure you are in ```Headers``` tab and scroll down to the ```Request Headers``` section.
7. Find the ```Cookie``` parameter and copy it's value (without the *Cookie:* part, just the value). It will likely be very long, make sure it's all there.

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

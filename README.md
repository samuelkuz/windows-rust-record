# Windows Rust Record
Basic wrapper around the Windows API to retrieve frame data from a monitor's display.

The goal of this project is to help any future developers easily utilize the Windows API to record their screen.

Creating the struct `WindowsScreenCapture` gets you started on everything you need to get the [u8] BGR8 Pixel Data.

## Build
* This will only build and work on Windows OS

I'd recommend forking this repo and using it as a starting point for your own project.

## Attributions
* Windows API code is adapted from [screenshot-rs](https://github.com/robmikh/screenshot-rs), which is licensed under the MIT license.
* Some code from [sharer](https://github.com/mira-screen-share/sharer), licensed under GPL-3.0 license

# Windows Rust Record
Basic wrapper around the Windows API to retrieve frame data from a monitor's display.

The goal of this project is to help any future developers easily interact with the Windows API to receive a displays BGRA8 frame data. (This will allow a user to take screenshots, record, etc.)

The struct `WindowsScreenCapture` gets you started on retrieving the [u8] BGRA8 Pixel Data from a display.

This is accomplished by initializing the WindowsScreenCapture Struct to find a monitor display to track and send the frame BGRA8 data through a channel.


## Build
* This will only build and work on Windows OS

I'd recommend forking this repo and using it as a starting point for your own project.

## Example
```
    // INITIALIZING THE WindowsScreenCapture Struct

    // 1. Create a WindowsScreenCapture for your primary display
    // This struct is utilized to begin capturing your display data and sending it through a channel (which you should consume from)
    let mut first_windows_screen_capture = WindowsScreenCapture::new_primary_capture()?;

    // 2. Manually select the display you want to capture (either utilizing the method Display::primary_display() or Display::enumerate_displays())
    let displays = Display::enumerate_displays()?;
    let display = displays.iter().nth(args.display).unwrap();

    // Create a WindowsScreenCapture object
    let mut second_windows_screen_capture = windows_screen_capture::WindowsScreenCapture::new(display)?;


    // USING THE WindowsScreenCapture Struct

    // Start recording the display frame data via: start_capture_session(), this will not actually start sending data through a channel but will begin the process of actually sending the frame data to our program.
    first_windows_screen_capture.start_capture_session();

    // Use the get_frame_receiver() method to set up the channel and get the receiver (the receiver will hold the BGRA8 [u8] data)
    let mut receiver = first_windows_screen_capture.get_frame_receiver().unwrap();


    // PROCESSING via the receiver

    // From there you can start receiving the frames and processing
    // This next portion is an example of receiving the Direct3D11CaptureFrame extracting
    // the BGR8 [u8] Pixel data and writing this information to a file 
    let mut ticker = tokio::time::interval(Duration::from_millis((1000 / 30) as u64));
    
    let mut file = File::create("test.raw").unwrap();

    while let Some(frame) = receiver.recv().await {
        let frame_time = frame.SystemRelativeTime()?.Duration;
        let (resource, frame_bits) = unsafe { first_windows_screen_capture.get_frame_content(frame)? };

        // encode here
        let encoded = encoder.encode(frame_bits, frame_time).unwrap();
        file.write_all(&encoded)?;

        unsafe {
            first_windows_screen_capture.unmap_d3d_context(&resource);
        }
        ticker.tick().await;
    }

    first_windows_screen_capture.session.Close().unwrap();
```

* Example project of mine using this crate: https://github.com/samuelkuz/streaming-server

## Attributions
* Windows API code is adapted from [screenshot-rs](https://github.com/robmikh/screenshot-rs), which is licensed under the MIT license.
* Some code from [sharer](https://github.com/mira-screen-share/sharer), licensed under GPL-3.0 license


## Misc Documentation
* https://learn.microsoft.com/en-us/windows/uwp/audio-video-camera/screen-capture
* https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscapturesession?view=winrt-22621

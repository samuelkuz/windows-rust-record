# Windows Rust Record
Basic wrapper around the Windows API to retrieve frame data from a monitor's display.

The goal of this project is to help any future developers easily utilize the Windows API to record their screen.

The struct `WindowsScreenCapture` gets you started on retrieving the [u8] BGR8 Pixel Data from a display.

## Build
* This will only build and work on Windows OS

I'd recommend forking this repo and using it as a starting point for your own project.

## Example
```
    // Gets your first display (another option is to enumerate your displays and choose one yourself)
    let primary_display = display::Display::primary_display().unwrap();

    // Create a WindowsScreenCapture object, 
    // which handles the processes for creating a Direct3D11CaptureFramePool and GraphicsCaptureSession 
    let mut windows_screen_capture = windows_screen_capture::WindowsScreenCapture::new(primary_display)?;
    
    // Once you've created a WindowsScreenCapture struct, you need to call the get_frame_receiver() method
    // This sets up the channel for the Direct3D11CaptureFramePool to send Direct3D11CaptureFrame to this receiver
    let receiver = windows_screen_capture.return_receiver().unwrap();

    // This next method calls StartCapture() on the GraphicsCaptureSession,
    // which kicks off recording frames and sending the data to the receiver
    windows_screen_capture.start_capture_session();

    // From there you can start receiving the frames and processing
    // This next portion is an example of receiving the Direct3D11CaptureFrame extracting
    // the BGR8 [u8] Pixel data and writing this information to a file 
    let mut ticker = tokio::time::interval(Duration::from_millis((1000 / 30) as u64));
    
    let mut file = File::create("test.raw").unwrap();

    while let Some(frame) = receiver.recv().await {
        let frame_time = frame.SystemRelativeTime()?.Duration;
        let (resource, frame_bits) = unsafe { windows_screen_capture.get_frame_content(frame)? };

        // encode here
        let encoded = encoder.encode(frame_bits, frame_time).unwrap();
        file.write_all(&encoded)?;

        unsafe {
            windows_screen_capture.unmap_d3d_context(&resource);
        }
        ticker.tick().await;
    }

    session.Close()?;
```

* Example project of mine using this crate: https://github.com/samuelkuz/streaming-server

## Attributions
* Windows API code is adapted from [screenshot-rs](https://github.com/robmikh/screenshot-rs), which is licensed under the MIT license.
* Some code from [sharer](https://github.com/mira-screen-share/sharer), licensed under GPL-3.0 license


## Misc Documentation
* https://learn.microsoft.com/en-us/windows/uwp/audio-video-camera/screen-capture
* https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscapturesession?view=winrt-22621

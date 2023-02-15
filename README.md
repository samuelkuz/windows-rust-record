# Windows Rust Record
Basic wrapper around the Windows API to retrieve frame data from a monitor's display.

The goal of this project is to help any future developers easily utilize the Windows API to record their screen.

Creating the struct `WindowsScreenCapture` and creating the GraphicsCaptureSession gets you started on retrieving the [u8] BGR8 Pixel Data from a display.

## Build
* This will only build and work on Windows OS

I'd recommend forking this repo and using it as a starting point for your own project.

## Attributions
* Windows API code is adapted from [screenshot-rs](https://github.com/robmikh/screenshot-rs), which is licensed under the MIT license.
* Some code from [sharer](https://github.com/mira-screen-share/sharer), licensed under GPL-3.0 license


## Example
```
    // Gets your first display (another option is to enumerate your displays and choose one yourself)
    let primary_display = display::Display::primary_display().unwrap();
    // Creates the GraphicsCaptureItem needed to create the WindowsScreenCapture struct
    let graphics_capture_item = display::create_capture_item_for_monitor(primary_display.handle)?;
    // WindowsScreenCapture::new creates a Direct3D11CaptureFramePool and GraphicsCaptureSession 
    let mut windows_screen_capture = windows_screen_capture::WindowsScreenCapture::new(&graphics_capture_item)?;
    /*
    Once you've created a WindowsScreenCapture struct, you need to call the get_frame_receiver() method
    to receive a Receiver<Direct3D11CaptureFrame>, as the frame_pool will send the Direct3D11CaptureFrame
    */
    let receiver = windows_screen_capture.return_receiver().unwrap();
    /*
    This next method calls StartCapture() on the GraphicsCaptureSession, which kicks off actually recording frames
    and sending the data to the receiver
    */
    windows_screen_captuer.start_capture_session();
```

## Misc Documentation
* https://learn.microsoft.com/en-us/windows/uwp/audio-video-camera/screen-capture
* https://learn.microsoft.com/en-us/uwp/api/windows.graphics.capture.graphicscapturesession?view=winrt-22621

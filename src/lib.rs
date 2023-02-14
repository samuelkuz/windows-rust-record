pub mod result;
pub mod display;
pub mod d3d;
pub mod windows_screen_capture;

pub use result::Result;

async fn record_demo() -> Result<()> {
    let primary_display = display::Display::primary_display().unwrap();
    let graphics_capture_item = display::create_capture_item_for_monitor(primary_display.handle)?;
    let mut windows_screen_capture = windows_screen_capture::WindowsScreenCapture::new(&graphics_capture_item)?;
    // windows_screen_capture.create_capture_session();
    // windows_screen_capture.record().await?;


    Ok(())
}

async fn test_this() -> Result<()> {
    let primary_display = display::Display::primary_display().unwrap();
    let graphics_capture_item = display::create_capture_item_for_monitor(primary_display.handle)?;
    let mut windows_screen_capture = windows_screen_capture::WindowsScreenCapture::new(&graphics_capture_item)?;
    windows_screen_capture.create_capture_session();
    let receiver = windows_screen_capture.return_receiver().unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_display() {
        let primary_monitor = display::Display::primary_display();

        assert_ne!(primary_monitor, None);
    }

    #[tokio::test]
    async fn test_receiver_stuff() {
        let receiver_result = test_this().await.unwrap();

        assert_eq!(receiver_result, ());
    }

    // #[tokio::test]
    // async fn test_record() {
    //     let demo_result = record_demo().await.unwrap();

    //     assert_eq!(demo_result, ());
    // }

}

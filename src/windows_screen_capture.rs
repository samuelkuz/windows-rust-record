use tokio::sync::mpsc::Receiver;
use std::slice;
use windows::core::{IInspectable, Interface};
use windows::Foundation::TypedEventHandler;
use windows::Graphics::Capture::{Direct3D11CaptureFrame, Direct3D11CaptureFramePool, GraphicsCaptureItem, GraphicsCaptureSession};
use windows::Graphics::DirectX::Direct3D11::IDirect3DSurface;
use windows::Graphics::DirectX::DirectXPixelFormat;
use windows::Win32::Graphics::Direct3D11::{D3D11_BIND_FLAG, D3D11_CPU_ACCESS_READ, D3D11_MAP_READ,
     D3D11_RESOURCE_MISC_FLAG, D3D11_TEXTURE2D_DESC, D3D11_USAGE_STAGING, ID3D11Device, ID3D11DeviceContext, ID3D11Resource, ID3D11Texture2D};
use crate::result::Result;
use crate::d3d;
use crate::display::{Display, create_capture_item_for_monitor};

pub struct WindowsScreenCapture {
    item: GraphicsCaptureItem,
    device: ID3D11Device,
    d3d_context: ID3D11DeviceContext,
    frame_pool: Direct3D11CaptureFramePool,
    pub session: GraphicsCaptureSession,
}

impl WindowsScreenCapture {
    pub fn new(display: &Display) -> Result<Self> {
        let item = create_capture_item_for_monitor(display.handle)?;
        let item_size = item.Size()?;
        let (device, d3d_device, d3d_context) = d3d::create_direct3d_devices_and_context()?;
        let frame_pool = Direct3D11CaptureFramePool::CreateFreeThreaded(
            &d3d_device,
            DirectXPixelFormat::B8G8R8A8UIntNormalized,
            1,
            item_size,
        )?;

        let session = frame_pool.CreateCaptureSession(&item).unwrap();

        Ok(Self {
            item,
            device,
            d3d_context,
            frame_pool,
            session,
        })
    }

    pub fn get_frame_receiver(&mut self) -> Result<Receiver<Direct3D11CaptureFrame>> {
        let (sender, mut receiver) = tokio::sync::mpsc::channel::<Direct3D11CaptureFrame>(1);

        self.frame_pool.FrameArrived(
            &TypedEventHandler::<Direct3D11CaptureFramePool, IInspectable>::new({
                move |frame_pool, _| {
                    let frame_pool = frame_pool.as_ref().unwrap();
                    let frame = frame_pool.TryGetNextFrame()?;
                    sender.try_send(frame).unwrap();
                    Ok(())
                }
            }),
        )?;

        return Ok(receiver)
    }

    pub fn start_capture_session(&mut self) {
        self.session.StartCapture().unwrap();
    }
    
    unsafe fn surface_to_texture(&mut self, surface: &IDirect3DSurface) -> Result<ID3D11Texture2D> {
        let source_texture: ID3D11Texture2D = d3d::get_d3d_interface_from_object(surface)?;
        let mut desc = D3D11_TEXTURE2D_DESC::default();
        source_texture.GetDesc(&mut desc);
        desc.BindFlags = D3D11_BIND_FLAG(0);
        desc.MiscFlags = D3D11_RESOURCE_MISC_FLAG(0);
        desc.Usage = D3D11_USAGE_STAGING;
        desc.CPUAccessFlags = D3D11_CPU_ACCESS_READ;
        let copy_texture = self.device.CreateTexture2D(&desc, None)?;
        let src: ID3D11Resource = source_texture.cast()?;
        let dst: ID3D11Resource = copy_texture.cast()?;
        self.d3d_context.CopyResource(&dst, &src);
        Ok(copy_texture)
    }

    pub unsafe fn unmap_d3d_context(&mut self, resource: &ID3D11Resource) {
        self.d3d_context.Unmap(resource, 0); 
    }

    pub unsafe fn get_frame_content(
        &mut self,
        frame: Direct3D11CaptureFrame,
    ) -> Result<(ID3D11Resource, &[u8])> {
        let texture = self.surface_to_texture(&frame.Surface()?)?;
        let resource: ID3D11Resource = texture.cast()?;
        let mapped = self.d3d_context.Map(&resource, 0, D3D11_MAP_READ, 0)?;
        let frame: &[u8] = slice::from_raw_parts(
            mapped.pData as *const _,
            (self.item.Size()?.Height as u32 * mapped.RowPitch) as usize,
        );
        Ok((resource, frame))
    }
}

impl Drop for WindowsScreenCapture {
    fn drop(&mut self) {
        self.frame_pool.Close().unwrap();
    }
}

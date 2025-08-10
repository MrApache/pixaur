use image::ImageError;
use thiserror::Error;
use wayland_client::{ConnectError, DispatchError};
use wgpu::{CreateSurfaceError, RequestAdapterError, RequestDeviceError, SurfaceError};

#[derive(Error, Debug)]
pub enum Error {
    //Wayland
    #[error("{0}")]
    Connect(#[from] ConnectError),
    #[error("{0}")]
    Dispatch(#[from] DispatchError),

    //Wgpu
    #[error("{0}")]
    CreateSurface(#[from] CreateSurfaceError),
    #[error("{0}")]
    RequestAdapter(#[from] RequestAdapterError),
    #[error("{0}")]
    RequestDevice(#[from] RequestDeviceError),
    #[error("{0}")]
    Surface(#[from] SurfaceError),

    //Image
    #[error("{0}")]
    Image(#[from] ImageError),

    //Std
    #[error("{0}")]
    IO(#[from] std::io::Error),
}

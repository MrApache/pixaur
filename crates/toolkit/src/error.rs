use thiserror::Error;
use wgpu::{CreateSurfaceError, RequestAdapterError, RequestDeviceError, SurfaceError};

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    CreateSurface(#[from] CreateSurfaceError),
    #[error("{0}")]
    RequestAdapter(#[from] RequestAdapterError),
    #[error("{0}")]
    RequestDevice(#[from] RequestDeviceError),
    #[error("{0}")]
    Surface(#[from] SurfaceError),
}

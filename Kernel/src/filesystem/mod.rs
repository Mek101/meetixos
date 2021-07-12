pub mod r#virtual;

pub type FsResult<T> = Result<T, FsError>;

pub enum FsError {
    NotSupported
}
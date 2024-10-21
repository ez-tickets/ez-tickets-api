#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Failed due to missing id. entity=[{entity}]")]
    MissingId { entity: &'static str },
    #[error("Failed due to lack of necessary data. require=[{data}]")]
    Require { data: &'static str },
    #[error("An error occurred in an driver service.")]
    Driver,
}

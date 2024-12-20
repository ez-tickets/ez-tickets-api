#[derive(Debug, thiserror::Error)]
pub enum ApplicationError {
    #[error("Failed due to invalid given command.")]
    InvalidGivenCommand,
    #[error("Failed due to missing id. entity=[{entity}]")]
    MissingId { entity: &'static str },
    #[error("Failed due to lack of necessary data. require=[{data}]")]
    Require { data: &'static str },
    #[error("An error occurred in an driver service.")]
    Driver,
    #[error("An error occurred at the Kernel layer.")]
    Kernel,
    #[error("Something went wrong...?")]
    Other,
}

use anchor_lang::error_code;

#[error_code]
pub enum CustomError {
    #[msg("Timeout not reached")]
    TimeOutNotReached,
}
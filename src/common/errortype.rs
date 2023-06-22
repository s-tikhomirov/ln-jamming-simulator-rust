#[derive(Debug)]
pub enum ErrorType {
    LowBalance,
    NoSlots,
    LowFee,
    FailedDeliberately,
}
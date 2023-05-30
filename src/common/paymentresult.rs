#[derive(Debug, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub enum PaymentResult {
    SUCCESS,
    FAILURE,
}
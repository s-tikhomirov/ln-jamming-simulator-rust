// ProtocolParams
// usize here because we use it as priority queue size
pub const MAX_NUM_SLOTS: usize = 483;

pub const DUST_LIMIT: u16 = 354;
pub const MAX_ROUTE_LENGTH: u16 = 20;

// FeeParams
pub const SUCCESS_BASE_FEE: u64 = 1;
pub const SUCCESS_FEE_RATE: f64 = 5.0 / (1000 * 1000) as f64;

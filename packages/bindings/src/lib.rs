mod msg;
mod query;
mod types;

pub use msg::OsmosisMsg;
pub use query::{EstimatePriceResponse, OsmosisQuery, PoolStateResponse, SpotPriceResponse};
pub use types::{Step, Swap, SwapAmount, SwapAmountWithLimit};

// This is a signal, such that any contract that imports these helpers will only run on the
// osmosis blockchain
#[no_mangle]
extern "C" fn requires_osmosis() {}

use casper_types::{
    account::AccountHash,
    bytesrepr::Bytes,
    U256
};
use error::Error;

pub(crate) trait ERC777Recipient {
    fn tokens_received(
        operator: AccountHash,
        from: AccountHash,
        to: AccountHash,
        amount: U256,
        data: Bytes,
        operator_data: Bytes
    ) -> Result<(), Error>;
}
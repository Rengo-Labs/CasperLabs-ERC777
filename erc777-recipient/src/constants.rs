//! Constants used by the ERC777 Recipient contract.
/// Contract Name
pub const ERC777_RECIPIENT_CONTRACT_NAME: &str = "erc777_recipient_contract";


/// Key name `movement_registry`
pub const BALANCES_REGISTRY_KEY_NAME: &str = "balances";
/// Key name `erc777_recipient_registry`
pub const ERC777_REGISTRY_KEY_NAME: &str = "erc777_recipient_registry";


/// Entry point named `transfer`
pub const TRANSFER_ENTRY_POINT: &str = "transfer";
/// Entry point named `burn`
pub const BURN_ENTRY_POINT: &str = "burn";
/// Entry points
pub const TOKENS_RECEIVED_ENTRY_POINT: &str = "tokens_received";
/// Entry points
pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";


/// Entry point's parameter
pub const ERC1820_CONTRACT_ARG_NAME: &str = "erc1820_contract";
/// Entry point's parameter
pub const ERC777_CONTRACT_ARG_NAME: &str = "erc777_contract";
/// Entry point's parameter
pub const SELF_CONTRACT_ARG_NAME: &str = "self_contract";
/// Entry point's parameter
pub const OPERATOR_RUNTIME_ARG_NAME: &str = "operator";
/// Entry point's parameter
pub const FROM_RUNTIME_ARG_NAME: &str = "from";
/// Entry point's parameter
pub const TO_RUNTIME_ARG_NAME: &str = "to";
/// Entry point's parameter
pub const RECIPIENT_RUNTIME_ARG_NAME: &str = "recipient";
/// Entry point's parameter
pub const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
/// Entry point's parameter
pub const USER_DATA_RUNTIME_ARG_NAME: &str = "user_data";
/// Entry point's parameter
pub const OPERATOR_DATA_RUNTIME_ARG_NAME: &str = "operator_data";


///-------- External Contract
/// Registry contract's entry points
pub const SET_INTERFACE_OF_EXTERNAL_ENTRY_POINT: &str = "set_interface_implementer";

/// Name of `account` runtime argument.
pub const ACCOUNT_RUNTIME_ARG_NAME: &str = "account";
/// Name of `i_hash` runtime argument.
pub const I_HASH_RUNTIME_ARG_NAME: &str = "i_hash";
/// Name of `implementer` runtime argument.
pub const IMPLEMENTER_RUNTIME_ARG_NAME: &str = "implementer";


/// Registry contract's entry points
pub const BALANCE_OF_EXTERNAL_ENTRY_POINT: &str = "balance_of";
/// Name of `address` runtime argument.
pub const ADDRESS_RUNTIME_ARG_NAME: &str = "address";


/// ERC777 contract's entry points
pub const OPERATOR_SEND_EXTERNAL_ENTRY_POINT: &str = "operator_send";
/// ERC777 contract's entry points
pub const OPERATOR_BURN_EXTERNAL_ENTRY_POINT: &str = "operator_burn";


/// Name of `sender` runtime argument.
pub const SENDER_RUNTIME_ARG_NAME: &str = "sender";
/// Parameter user_data
pub const DATA_RUNTIME_ARG_NAME: &str = "data";

///-------- TAG to registry contract in the erc1820 global registry
/// Constant to retrieve an implementer to receive tokens
pub const HASH_ERC1820_RECIPIENT: &str = "ERC777TokensRecipient";
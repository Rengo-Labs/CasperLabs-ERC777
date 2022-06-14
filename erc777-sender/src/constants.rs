//! Constants used by the ERC777 Sender contract.
/// Contract Name
pub const ERC777_SENDER_CONTRACT_NAME: &str = "erc777_sender_contract";

/// Key name `movement_registry`
pub const MOVEMENTS_REGISTRY_KEY_NAME: &str = "movement_registry";
/// Key name `movement_registry`
pub const ERC777_REGISTRY_KEY_NAME: &str = "erc777_sender_registry";


/// ------------ Entry Points
/// Entry point named `tokens_to_send`
pub const TOKENS_TO_SEND_ENTRY_POINT: &str = "tokens_to_send";
/// Entry point named `transfer`
pub const TRANSFER_ENTRY_POINT: &str = "transfer";
/// Entry point named `transfer`
pub const BURN_ENTRY_POINT: &str = "burn";


/// Entry point's parameter
/// Parameter operator
pub const OPERATOR_RUNTIME_ARG_NAME: &str = "operator";
/// Parameter from
pub const FROM_RUNTIME_ARG_NAME: &str = "from";
/// Parameter to
pub const TO_RUNTIME_ARG_NAME: &str = "to";
/// Parameter amount
pub const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
/// Parameter user_data
pub const USER_DATA_RUNTIME_ARG_NAME: &str = "user_data";
/// Parameter operator_data
pub const OPERATOR_DATA_RUNTIME_ARG_NAME: &str = "operator_data";
/// Parameter erc1820_contract
pub const ERC1820_CONTRACT_ARG_NAME: &str = "erc1820_contract";
/// Parameter erc777_contract
pub const ERC777_CONTRACT_ARG_NAME: &str = "erc777_contract";


///-------- External Contract
/// Registry contract's entry points
pub const SET_INTERFACE_OF_EXTERNAL_ENTRY_POINT: &str = "set_interface_implementer";

/// Name of `account` runtime argument.
pub const ACCOUNT_RUNTIME_ARG_NAME: &str = "account";
/// Name of `i_hash` runtime argument.
pub const I_HASH_RUNTIME_ARG_NAME: &str = "i_hash";
/// Name of `implementer` runtime argument.
pub const IMPLEMENTER_RUNTIME_ARG_NAME: &str = "implementer";


/// ERC777 contract's entry points
pub const OPERATOR_SEND_EXTERNAL_ENTRY_POINT: &str = "operator_send";
/// ERC777 contract's entry points
pub const OPERATOR_BURN_EXTERNAL_ENTRY_POINT: &str = "operator_burn";


/// Name of `sender` runtime argument.
pub const SENDER_RUNTIME_ARG_NAME: &str = "sender";
/// Name of `recipient` runtime argument.
pub const RECIPIENT_RUNTIME_ARG_NAME: &str = "recipient";
/// Parameter user_data
pub const DATA_RUNTIME_ARG_NAME: &str = "data";


///-------- TAG to registry contract in the erc1820 global registry
/// Constant to retrieve an implementer to receive tokens
pub const HASH_ERC1820_SENDER: &str = "ERC777TokensSender";
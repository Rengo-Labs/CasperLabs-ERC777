//! Constants used by the ERC20 contract.

/// Name of named-key for `name`.
pub const NAME_KEY_NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL_KEY_NAME: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS_KEY_NAME: &str = "decimals";
/// constant value
pub const DECIMALS_KEY_VALUE: u8 = 18;
/// Name of dictionary-key for `granularity`
pub const GRANULARITY_KEY_NAME: &str = "granularity";
/// Name of named-key for `contract`
pub const ERC20_TOKEN_CONTRACT_NAME: &str = "erc20_token_contract";
/// Name of dictionary-key for `balances`
pub const BALANCES_KEY_NAME: &str = "balances";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES_KEY_NAME: &str = "allowances";
/// Name of named-key for `total_supply`
pub const TOTAL_SUPPLY_KEY_NAME: &str = "total_supply";
/// Name of named-key for `operators`
pub const OPERATORS_KEY_NAME: &str = "operators";

/// Name of `name` entry point.
pub const NAME_ENTRY_POINT_NAME: &str = "name";
/// Name of `symbol` entry point.
pub const SYMBOL_ENTRY_POINT_NAME: &str = "symbol";
/// Name of `decimals` entry point.
pub const DECIMALS_ENTRY_POINT_NAME: &str = "decimals";
/// Name of `granularity` entry point.
pub const GRANULARITY_ENTRY_POINT_NAME: &str = "granularity";
/// Name of `balance_of` entry point.
pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";
/// Name of `transfer` entry point.
pub const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
/// Name of `send` entry point.
pub const SEND_ENTRY_POINT_NAME: &str = "send";
/// Name of `burn` entry point.
pub const BURN_ENTRY_POINT_NAME: &str = "burn";
/// Name of `is_operator_for` entry point.
pub const IS_OPERATOR_FOR_ENTRY_POINT_NAME: &str = "is_operator_for";
/// Name of `authorize_operator` entry point.
pub const AUTHORIZE_OPERATOR_ENTRY_POINT_NAME: &str = "authorize_operator";
/// Name of `revoke_operator` entry point.
pub const REVOKE_OPERATOR_ENTRY_POINT_NAME: &str = "revoke_operator";
/// Name of `default_operators` entry point.
pub const DEFAULT_OPERATORS_ENTRY_POINT_NAME: &str = "default_operators";
/// Name of `operator_send` entry point.
pub const OPERATOR_SEND_ENTRY_POINT_NAME: &str = "operator_send";
/// Name of `operator_burn` entry point.
pub const OPERATOR_BURN_ENTRY_POINT_NAME: &str = "operator_burn";
/// Name of `set_registry` entry point.
pub const SET_REGISTRY_ENTRY_POINT_NAME: &str = "set_registry";
/// Name of `set_interface_registry` entry point.
pub const SET_INTERFACE_REGISTRY_ENTRY_POINT_NAME_2: &str = "set_interface_registry2";

/// Name of `approve` entry point.
pub const APPROVE_ENTRY_POINT_NAME: &str = "approve";
/// Name of `allowance` entry point.
pub const ALLOWANCE_ENTRY_POINT_NAME: &str = "allowance";
/// Name of `transfer_from` entry point.
pub const TRANSFER_FROM_ENTRY_POINT_NAME: &str = "transfer_from";
/// Name of `total_supply` entry point.
pub const TOTAL_SUPPLY_ENTRY_POINT_NAME: &str = "total_supply";

/// Name of `address` runtime argument.
pub const ADDRESS_RUNTIME_ARG_NAME: &str = "address";
/// Name of `owner` runtime argument.
pub const OWNER_RUNTIME_ARG_NAME: &str = "owner";
/// Name of `spender` runtime argument.
pub const SPENDER_RUNTIME_ARG_NAME: &str = "spender";
/// Name of `amount` runtime argument.
pub const AMOUNT_RUNTIME_ARG_NAME: &str = "amount";
/// Name of `recipient` runtime argument.
pub const RECIPIENT_RUNTIME_ARG_NAME: &str = "recipient";
/// Name of `name` runtime argument.
pub const NAME_RUNTIME_ARG_NAME: &str = "name";
/// Name of `symbol` runtime argument.
pub const SYMBOL_RUNTIME_ARG_NAME: &str = "symbol";
/// Name of `decimals` runtime argument.
pub const DECIMALS_RUNTIME_ARG_NAME: &str = "decimals";
/// Name of `total_supply` runtime argument.
pub const GRANULARITY_RUNTIME_ARG_NAME: &str = "granularity";
/// Name of `total_supply` runtime argument.
pub const TOTAL_SUPPLY_RUNTIME_ARG_NAME: &str = "total_supply";
/// Name of `sender` runtime argument.
pub const SENDER_RUNTIME_ARG_NAME: &str = "sender";
/// Name of `data` runtime argument.
pub const DATA_RUNTIME_ARG_NAME: &str = "data";
/// Name of `operator_data` runtime argument.
pub const OPERATOR_DATA_RUNTIME_ARG_NAME: &str = "operator_data";
/// Name of `operator` runtime argument.
pub const OPERATOR_RUNTIME_ARG_NAME: &str = "operator";
/// Name of `operators` runtime argument.
pub const OPERATORS_RUNTIME_ARG_NAME: &str = "operators";
/// Name of `token_holder` runtime argument.
pub const TOKEN_HOLDER_RUNTIME_ARG_NAME: &str = "token_holder";
/// Name of `account` runtime argument.
pub const ACCOUNT_RUNTIME_ARG_NAME: &str = "account";

/// External contracts
pub const REGISTRY_CONTRACT_NAME: &str = "erc1820_global_registry";
/// External contracts
pub const ERC777_SENDER_CONTRACT_NAME: &str = "erc777_sender_contract";
/// External contracts
pub const ERC777_RECIPIENT_CONTRACT_NAME: &str = "erc777_recipient_contract";

/// Name of `from` runtime argument.
pub const FROM_RUNTIME_ARG_NAME: &str = "from";
/// Name of `to` runtime argument.
pub const TO_RUNTIME_ARG_NAME: &str = "to";
/// Name of `user_data` runtime argument.
pub const USER_DATA_RUNTIME_ARG_NAME: &str = "user_data";
/// Name of `i_hash` runtime argument.
pub const I_HASH_RUNTIME_ARG_NAME: &str = "i_hash";
/// Name of `implementer` runtime argument.
pub const IMPLEMENTER_RUNTIME_ARG_NAME: &str = "implementer";
/// Name of `erc1820_contract` runtime argument.
pub const HASH_ERC1820_RUNTIME_ARG_NAME: &str = "erc1820_contract";


/// Constants used for external calls
/// Constant to retrieve an implementer to send tokens
pub const HASH_ERC1820_SENDER: &str = "ERC777TokensSender";
/// Constant to retrieve an implementer to receive tokens
pub const HASH_ERC1820_RECIPIENT: &str = "ERC777TokensRecipient";

/// This hash must be configured to use with the actual ERC1820 deployed.
pub const HASH_ERC1820_REGISTRY: &str = "contract-dbd88acafb7c031f8c9f2aa3a8da7a3a6b74c655921496cdc3bd0d50f61aa997";

/// Registry contract's entry points
pub const SET_INTERFACE_OF_EXTERNAL_ENTRY_POINT: &str = "set_interface_implementer";
/// Registry contract's entry points
pub const GET_INTERFACE_OF_EXTERNAL_ENTRY_POINT: &str = "get_interface_implementer";
/// ERC777 Sender's entry point
pub const TOKENS_TO_SEND_OF_EXTERNAL_ENTRY_POINT: &str = "tokens_to_send";
/// ERC777 Recipient's entry point
pub const TOKENS_RECEIVED_OF_EXTERNAL_ENTRY_POINT: &str = "tokens_received";
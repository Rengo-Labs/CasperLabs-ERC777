//! Constants used by the ERC20 contract.

/// Name of named-key for `name`.
pub const NAME_KEY_NAME: &str = "name";
/// Name of named-key for `symbol`
pub const SYMBOL_KEY_NAME: &str = "symbol";
/// Name of named-key for `decimals`
pub const DECIMALS_KEY_NAME: &str = "decimals";
pub const DECIMALS_KEY_VALUE: u8 = 18;

pub const GRANULARITY_KEY_NAME: &str = "granularity";
/// Name of named-key for `contract`
pub const ERC20_TOKEN_CONTRACT_KEY_NAME: &str = "erc20_token_contract";
/// Name of dictionary-key for `balances`
pub const BALANCES_KEY_NAME: &str = "balances";
/// Name of dictionary-key for `allowances`
pub const ALLOWANCES_KEY_NAME: &str = "allowances";
/// Name of named-key for `total_supply`
pub const TOTAL_SUPPLY_KEY_NAME: &str = "total_supply";
pub const OPERATORS_KEY_NAME: &str = "operators";

/// Name of `name` entry point.
pub const NAME_ENTRY_POINT_NAME: &str = "name";
/// Name of `symbol` entry point.
pub const SYMBOL_ENTRY_POINT_NAME: &str = "symbol";
/// Name of `decimals` entry point.
pub const DECIMALS_ENTRY_POINT_NAME: &str = "decimals";
pub const GRANULARITY_ENTRY_POINT_NAME: &str = "granularity";
/// Name of `balance_of` entry point.
pub const BALANCE_OF_ENTRY_POINT_NAME: &str = "balance_of";
/// Name of `transfer` entry point.
pub const TRANSFER_ENTRY_POINT_NAME: &str = "transfer";
pub const SEND_ENTRY_POINT_NAME: &str = "send";
pub const BURN_ENTRY_POINT_NAME: &str = "burn";
pub const IS_OPERATOR_FOR_ENTRY_POINT_NAME: &str = "is_operator_for";
pub const AUTHORIZE_OPERATOR_ENTRY_POINT_NAME: &str = "authorize_operator";
pub const REVOKE_OPERATOR_ENTRY_POINT_NAME: &str = "revoke_operator";
pub const DEFAULT_OPERATORS_ENTRY_POINT_NAME: &str = "default_operators";
pub const OPERATOR_SEND_ENTRY_POINT_NAME: &str = "operator_send";
pub const OPERATOR_BURN_ENTRY_POINT_NAME: &str = "operator_burn";

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
///
pub const TOTAL_SUPPLY_RUNTIME_ARG_NAME: &str = "total_supply";
pub const SENDER_RUNTIME_ARG_NAME: &str = "sender";
pub const DATA_RUNTIME_ARG_NAME: &str = "data";
pub const OPERATOR_DATA_RUNTIME_ARG_NAME: &str = "operator_data";
pub const OPERATOR_RUNTIME_ARG_NAME: &str = "operator";
pub const OPERATORS_RUNTIME_ARG_NAME: &str = "operators";
pub const TOKEN_HOLDER_RUNTIME_ARG_NAME: &str = "token_holder";
pub const ACCOUNT_RUNTIME_ARG_NAME: &str = "account";

// External contracts
pub const REGISTRY_CONTRACT_NAME: &str = "registry_contract";
pub const REGISTRY_CONTRACT_SET_INTERFACE_ENTRY_POINT: &str = "setInterfaceImplementer";
pub const REGISTRY_CONTRACT_GET_INTERFACE_ENTRY_POINT: &str = "getInterfaceImplementer";

pub const I_HASH_RUNTIME_ARG_NAME: &str = "i_hash";
pub const IMPLEMENTER_RUNTIME_ARG_NAME: &str = "implementer";



use reth_primitives::{rpc::transaction::eip2930::AccessListItem, Address, Bytes, U128, U256};
use serde::{Deserialize, Serialize};

/// Call request
#[derive(Debug, Clone, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default, rename_all = "camelCase", deny_unknown_fields)]
pub struct CallRequest {
    /// From
    pub from: Option<Address>,
    /// To
    pub to: Option<Address>,
    /// Gas Price
    pub gas_price: Option<U128>,
    /// EIP-1559 Max base fee the caller is willing to pay
    pub max_fee_per_gas: Option<U128>,
    /// EIP-1559 Priority fee the caller is paying to the block author
    pub max_priority_fee_per_gas: Option<U128>,
    /// Gas
    pub gas: Option<U256>,
    /// Value
    pub value: Option<U256>,
    /// Data
    pub data: Option<Bytes>,
    /// Nonce
    pub nonce: Option<U256>,
    /// AccessList
    pub access_list: Option<Vec<AccessListItem>>,
    /// EIP-2718 type
    #[serde(rename = "type")]
    pub transaction_type: Option<U256>,
}

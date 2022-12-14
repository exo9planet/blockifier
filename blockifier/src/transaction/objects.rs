use std::collections::HashMap;

use starknet_api::transaction::Fee;

use crate::execution::entry_point::CallInfo;

// TODO(Adi, 10/12/2022): Add a 'transaction_type' field.
/// Contains the information gathered by the execution of a transaction.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct TransactionExecutionInfo {
    /// Transaction validation call info.
    pub validate_info: CallInfo,
    /// Transaction execution call info; trivial for `Declare`.
    pub execute_info: Option<CallInfo>,
    /// Fee transfer call info.
    pub fee_transfer_info: CallInfo,
    /// The actual fee that was charged (in Wei).
    pub actual_fee: Fee,
    /// Actual execution resources the transaction is charged for,
    /// including L1 gas and additional OS resources estimation.
    pub actual_resources: ResourcesMapping,
}

/// A mapping from a transaction execution resource to its actual usage.
#[derive(Debug, Default, Eq, PartialEq)]
pub struct ResourcesMapping(pub HashMap<String, usize>);
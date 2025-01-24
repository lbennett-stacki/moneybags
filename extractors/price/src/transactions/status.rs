use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

pub fn is_failed_transaction(tx: &EncodedConfirmedTransactionWithStatusMeta) -> bool {
    if let Some(ref meta) = tx.transaction.meta {
        meta.err.is_some()
    } else {
        false
    }
}

use hedera::{transaction::{TransactionCryptoAddClaim}, AccountId};
use pyo3::PyResult;
use crate::PyPublicKey;

def_transaction!(TransactionCryptoAddClaim(AccountId, Vec<u8>){}{
    pub fn add_key(&mut self, key: &PyPublicKey) -> PyResult<()> {
        self.inner.key(key.clone().into());
        Ok(())
    }
});

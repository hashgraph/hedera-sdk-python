use super::{
    errors::PyValueError, query_crypto_get_account_balance::*, query_crypto_get_info::*,
    query_file_get_contents::*, query_transaction_get_receipt::*
};
use crate::{
    either::Either,
    id::{PyAccountId, PyContractId, PyFileId},
    transaction_id::PyTransactionId, PyQueryContractCall,
    PyQueryCryptoGetClaim, PyQueryFileGetInfo, PyQueryTransactionGetRecord,
    PyTransactionContractCall, PyTransactionContractCreate, PyTransactionContractUpdate,
    PyTransactionCryptoCreate, PyTransactionCryptoDelete, PyTransactionCryptoDeleteClaim,
    PyTransactionCryptoTransfer, PyTransactionCryptoUpdate, PyTransactionFileAppend,
    PyTransactionFileCreate, PyTransactionFileDelete, PySecretKey
};
use hedera::{AccountId, Client, ContractId, FileId, TransactionId};
use pyo3::{prelude::*, types::PyObjectRef};
use std::rc::Rc;
use try_from::TryInto;

#[pyclass(name = Client)]
pub struct PyClient {
    pub inner: Rc<Client>,
}

#[pymethods]
impl PyClient {
    #[new]
    pub fn __new__(obj: &PyRawObject, address: &str) -> PyResult<()> {
        let client = Client::new(address).map_err(PyValueError)?;
        obj.init(move || Self {
            inner: Rc::new(client),
        })
    }

    pub fn set_node(&mut self, node: &PyObjectRef) -> PyResult<()> {
        let n = (FromPyObject::extract(node)?: Either<&str, &PyAccountId>).try_into()?;
        match Rc::get_mut(&mut self.inner) {
            Some(c) => c.set_node(n),
            None => ()
        };
        Ok(())
    }

    pub fn set_operator(&mut self, operator: &PyObjectRef,
                        secret: &'static PyObjectRef) -> PyResult<()> {
        let op = (FromPyObject::extract(operator)?: Either<&str, &PyAccountId>).try_into()?;
        let sk = FromPyObject::extract(secret)?: &PySecretKey;

        let s = move || {
            return sk.inner.clone()
        };

        match Rc::get_mut(&mut self.inner) {
            Some(c) => c.set_operator(op, s),
            None => ()
        }
        Ok(())
    }

    /// transfer_crypto(self) TransactionCryptoTransfer
    /// --
    ///
    /// Transfer hbar between accounts.
    ///
    /// If a transfer fails for any accounts in the transaction, the whole transaction fails.
    pub fn transfer_crypto(&self) -> PyResult<PyTransactionCryptoTransfer> {
        Ok(PyTransactionCryptoTransfer::new(&self.inner))
    }

    /// create_account(self) -> TransactionCryptoCreate
    /// --
    ///
    /// Create a crypto-currency account.
    pub fn create_account(&self) -> PyResult<PyTransactionCryptoCreate> {
        Ok(PyTransactionCryptoCreate::new(&self.inner))
    }

    /// account(self, id: Union[str, AccountId]) -> PartialAccountMessage
    /// --
    ///
    /// Access available operations on a single crypto-currency account.
    pub fn account(&self, id: &PyObjectRef) -> PyResult<PyPartialAccountMessage> {
        Ok(PyPartialAccountMessage {
            client: Rc::clone(&self.inner),
            account: (FromPyObject::extract(id)?: Either<&str, &PyAccountId>).try_into()?,
        })
    }

    /// create_contract(self) -> TransactionContractCreate
    /// --
    ///
    /// Create a smart contract instance.
    ///
    /// The instance will run the bytecode stored in the given file.
    pub fn create_contract(&self) -> PyResult<PyTransactionContractCreate> {
        Ok(PyTransactionContractCreate::new(&self.inner))
    }

    /// contract(self, id: Union[str, ContractId]) -> PartialContractMessage
    /// --
    ///
    /// Access available operations on a single smart contract.
    pub fn contract(&self, id: &PyObjectRef) -> PyResult<PyPartialContractMessage> {
        Ok(PyPartialContractMessage {
            client: Rc::clone(&self.inner),
            contract: (FromPyObject::extract(id)?: Either<&str, &PyContractId>).try_into()?,
        })
    }

    /// create_file(self) -> TransactionFileCreate
    /// --
    ///
    /// Create a file.
    ///
    /// The contents of the file to be created may too large, so this may be used in conjunction
    /// with :py:method:`hedera.PartialFileMessage.append` to create such files.
    pub fn create_file(&self) -> PyResult<PyTransactionFileCreate> {
        Ok(PyTransactionFileCreate::new(&self.inner))
    }

    /// file(self, id: Union[str, FileId]) -> PartialFileMessage
    /// --
    ///
    /// Access available operations on a single file.
    pub fn file(&self, id: &PyObjectRef) -> PyResult<PyPartialFileMessage> {
        Ok(PyPartialFileMessage {
            client: Rc::clone(&self.inner),
            file: (FromPyObject::extract(id)?: Either<&str, &PyFileId>).try_into()?,
        })
    }

    /// transaction(self, id: Union[str, TransactionId]) -> PartialTransactionMessage
    /// --
    ///
    /// Access available operations on a single transaction.
    pub fn transaction(&self, id: &PyObjectRef) -> PyResult<PyPartialTransactionMessage> {
        Ok(PyPartialTransactionMessage {
            client: Rc::clone(&self.inner),
            transaction: (FromPyObject::extract(id)?: Either<&str, &PyTransactionId>).try_into()?,
        })
    }
}

#[pyclass(name = PartialAccountMessage)]
pub struct PyPartialAccountMessage {
    client: Rc<Client>,
    account: AccountId,
}

#[pymethods]
impl PyPartialAccountMessage {
    /// balance(self) -> QueryCryptoGetAccountBalance
    /// --
    ///
    /// Get the balance of a crypto-currency account.
    ///
    /// This returns only the balance, so it is a smaller and faster reply than
    /// :py:method:`hedera.PartialAccountMessage.info`.
    pub fn balance(&self) -> PyResult<PyQueryCryptoGetAccountBalance> {
        Ok(PyQueryCryptoGetAccountBalance::new(
            &self.client,
            self.account,
        ))
    }

    /// info(self) -> QueryCryptoGetInfo
    /// --
    ///
    /// Get information about an account.
    ///
    /// Information returned includes the balance, but not account records. If only the balance is
    /// needed, :py:method:`hedera.PartialAccountMessage.balance` is a smaller and faster reply.
    pub fn info(&self) -> PyResult<PyQueryCryptoGetInfo> {
        Ok(PyQueryCryptoGetInfo::new(&self.client, self.account))
    }

    /// update(self) -> TransactionCryptoUpdate
    /// --
    ///
    /// Modify an account.
    ///
    /// To change the key, both the old and new key must be used to sign the transaction.
    pub fn update(&self) -> PyResult<PyTransactionCryptoUpdate> {
        Ok(PyTransactionCryptoUpdate::new(&self.client, self.account))
    }

    /// delete(self) -> TransactionCryptoDelete
    /// --
    ///
    /// Mark an account as deleted, transferring current balance to another account.
    ///
    /// Transfers into a deleted account will fail, but its expiration date can be extended as
    /// normal.
    pub fn delete(&self) -> PyResult<PyTransactionCryptoDelete> {
        Ok(PyTransactionCryptoDelete::new(&self.client, self.account))
    }

    /// claim(self, hash: bytes) -> PartialAccountClaimMessage
    /// --
    ///
    /// Access available operations on the claims of an account.
    pub fn claim(&self, hash: Vec<u8>) -> PyResult<PyPartialAccountClaimMessage> {
        Ok(PyPartialAccountClaimMessage {
            client: Rc::clone(&self.client),
            account: self.account,
            hash,
        })
    }
}

#[pyclass(name = PartialAccountClaimMessage)]
pub struct PyPartialAccountClaimMessage {
    client: Rc<Client>,
    account: AccountId,
    hash: Vec<u8>,
}

#[pymethods]
impl PyPartialAccountClaimMessage {
    /// delete(self) -> TransactionCryptoDeleteClaim
    /// --
    ///
    /// Delete a claim attached to an account.
    ///
    /// This transaction is valid if signed by all the keys that are used for transfers out of the
    /// account, or if signed by any of the keys used to attach the claim in the first place.
    pub fn delete(&self) -> PyResult<PyTransactionCryptoDeleteClaim> {
        Ok(PyTransactionCryptoDeleteClaim::new(
            &self.client,
            self.account,
            self.hash.clone(),
        ))
    }

    /// get(self) -> QueryCryptoGetClaim
    /// --
    ///
    /// Get a single claim attached to an account, if it exists.
    pub fn get(&self) -> PyResult<PyQueryCryptoGetClaim> {
        Ok(PyQueryCryptoGetClaim::new(
            &self.client,
            self.account,
            self.hash.clone(),
        ))
    }
}

#[pyclass(name = PartialFileMessage)]
pub struct PyPartialFileMessage {
    client: Rc<Client>,
    file: FileId,
}

#[pymethods]
impl PyPartialFileMessage {
    /// append(self, contents) -> TransactionFileAppend
    /// --
    ///
    /// Append the contents to the end of the file.
    ///
    /// If a file was too large to create in a single transaction, it can be created with the first
    /// part of its contents and adding the rest by appending to it as needed with this transaction.
    pub fn append(&self, contents: Vec<u8>) -> PyResult<PyTransactionFileAppend> {
        Ok(PyTransactionFileAppend::new(
            &self.client,
            self.file,
            contents,
        ))
    }

    /// delete(self) -> TransactionFileDelete
    /// --
    ///
    /// Delete a file.
    ///
    /// The file will be marked as deleted, having no contents, until the expiration date.
    /// Then it will disappear.
    pub fn delete(&self) -> PyResult<PyTransactionFileDelete> {
        Ok(PyTransactionFileDelete::new(&self.client, self.file))
    }

    /// info(self) -> QueryFileGetInfo
    /// --
    ///
    /// Get information about a file (excludes contents).
    ///
    /// If a file has expired, there will not be any information.
    pub fn info(&self) -> PyResult<PyQueryFileGetInfo> {
        Ok(PyQueryFileGetInfo::new(&self.client, self.file))
    }

    /// contents(self) -> QueryFileGetContents
    /// --
    ///
    /// Get the contents of a file.
    pub fn contents(&self) -> PyResult<PyQueryFileGetContents> {
        Ok(PyQueryFileGetContents::new(&self.client, self.file))
    }
}

#[pyclass(name = PartialContractMessage)]
pub struct PyPartialContractMessage {
    client: Rc<Client>,
    contract: ContractId,
}

#[pymethods]
impl PyPartialContractMessage {
    /// call(self) -> TransactionContractCall
    /// --
    ///
    /// Call a function of the given smart contract instance.
    ///
    /// It can use the given amount of gas, and any unspent gas will be refunded to the paying
    /// account.
    pub fn call(&self) -> PyResult<PyTransactionContractCall> {
        Ok(PyTransactionContractCall::new(&self.client, self.contract))
    }

    pub fn query(&self, gas: i64, params: Vec<u8>, max_result_size: i64) -> PyResult<PyQueryContractCall> {
        Ok(PyQueryContractCall::new(&self.client, self.contract, gas, params, max_result_size))
    }

    /// update(self) -> TransactionContractUpdate
    /// --
    ///
    /// Modify a smart contract instance.
    ///
    /// If the contract had no adminKey from the start only the expiration time can be updated.
    pub fn update(&self) -> PyResult<PyTransactionContractUpdate> {
        Ok(PyTransactionContractUpdate::new(
            &self.client,
            self.contract,
        ))
    }
}

#[pyclass(name = PartialTransactionMessage)]
pub struct PyPartialTransactionMessage {
    client: Rc<Client>,
    transaction: TransactionId,
}

#[pymethods]
impl PyPartialTransactionMessage {
    /// receipt(self) -> QueryTransactionGetReceipt
    /// --
    ///
    /// Get the receipt of the transaction.
    ///
    /// Once a transaction reaches consensus, then information about whether it succeeded or
    /// failed will be available until the end of the receipt period (180 seconds).
    pub fn receipt(&self) -> PyResult<PyQueryTransactionGetReceipt> {
        Ok(PyQueryTransactionGetReceipt::new(
            &self.client,
            self.transaction.clone(),
        ))
    }

    /// record(self) -> QueryTransactionGetRecord
    /// --
    ///
    /// Get the record of the transaction.
    ///
    /// If a transaction requested a record, then it will be available for one hour, and a state
    /// proof is available for it.
    pub fn record(&self) -> PyResult<PyQueryTransactionGetRecord> {
        Ok(PyQueryTransactionGetRecord::new(
            &self.client,
            self.transaction.clone(),
        ))
    }
}

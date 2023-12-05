use reth_db::DatabaseError;
use reth_primitives::trie::{BranchNodeCompact, Nibbles, StoredNibblesSubKey};

use crate::updates::TrieKey;

use super::{TrieCursor, TrieCursorFactory};

/// Noop trie cursor factory.
#[derive(Default, Debug)]
#[non_exhaustive]
pub struct NoopTrieCursorFactory;

impl TrieCursorFactory for NoopTrieCursorFactory {
    fn account_trie_cursor(
        &self,
    ) -> Result<Box<dyn TrieCursor<Key = Nibbles> + '_>, DatabaseError> {
        Ok(Box::<NoopAccountTrieCursor>::default())
    }

    fn storage_tries_cursor(
        &self,
        _hashed_address: reth_primitives::B256,
    ) -> Result<Box<dyn TrieCursor<Key = StoredNibblesSubKey> + '_>, DatabaseError> {
        Ok(Box::<NoopStorageTrieCursor>::default())
    }
}

/// Noop account trie cursor.
#[derive(Default, Debug)]
#[non_exhaustive]
pub struct NoopAccountTrieCursor;

impl TrieCursor for NoopAccountTrieCursor {
    type Key = Nibbles;

    fn seek(
        &mut self,
        _key: Self::Key,
    ) -> Result<Option<(Vec<u8>, BranchNodeCompact)>, DatabaseError> {
        Ok(None)
    }

    fn seek_exact(
        &mut self,
        _key: Self::Key,
    ) -> Result<Option<(Vec<u8>, BranchNodeCompact)>, DatabaseError> {
        Ok(None)
    }

    fn current(&mut self) -> Result<Option<TrieKey>, DatabaseError> {
        Ok(None)
    }
}

/// Noop account trie cursor.
#[derive(Default, Debug)]
#[non_exhaustive]
pub struct NoopStorageTrieCursor;

impl TrieCursor for NoopStorageTrieCursor {
    type Key = StoredNibblesSubKey;

    fn seek(
        &mut self,
        _key: Self::Key,
    ) -> Result<Option<(Vec<u8>, BranchNodeCompact)>, DatabaseError> {
        Ok(None)
    }

    fn seek_exact(
        &mut self,
        _key: Self::Key,
    ) -> Result<Option<(Vec<u8>, BranchNodeCompact)>, DatabaseError> {
        Ok(None)
    }

    fn current(&mut self) -> Result<Option<TrieKey>, DatabaseError> {
        Ok(None)
    }
}

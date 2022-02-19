use crate::{Blob, EthSpec, SignedBeaconBlock};
use serde_derive::{Deserialize, Serialize};
use ssz_derive::{Encode};
use ssz_types::{VariableList};
use tree_hash::TreeHash;
use tree_hash_derive::TreeHash;

#[cfg_attr(feature = "arbitrary-fuzz", derive(arbitrary::Arbitrary))]
#[derive(Debug, Clone, Serialize, Deserialize, Encode, TreeHash)]
pub struct BeaconBlockAndBlobs<E: EthSpec> {
    pub block: SignedBeaconBlock<E>,
    pub blobs: VariableList<Blob<E::ChunksPerBlob>, E::MaxObjectListSize>,
}
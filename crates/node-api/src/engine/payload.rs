use crate::PayloadAttributes;
use reth_primitives::B256;
use reth_rpc_types::engine::ExecutionPayload;

/// Either an [ExecutionPayload] or a types that implements the [PayloadAttributes] trait.
#[derive(Debug)]
pub enum PayloadOrAttributes<'a, AttributesType> {
    /// An [ExecutionPayload] and optional parent beacon block root.
    ExecutionPayload {
        /// The inner execution payload
        payload: &'a ExecutionPayload,
        /// The parent beacon block root
        parent_beacon_block_root: Option<B256>,
    },
    /// A payload attributes type.
    PayloadAttributes(&'a AttributesType),
}

impl<'a, AttributesType> PayloadOrAttributes<'a, AttributesType>
where
    AttributesType: PayloadAttributes,
{
    /// Construct a [PayloadOrAttributes] from an [ExecutionPayload] and optional parent beacon
    /// block root.
    pub fn from_execution_payload(
        payload: &'a ExecutionPayload,
        parent_beacon_block_root: Option<B256>,
    ) -> Self {
        Self::ExecutionPayload { payload, parent_beacon_block_root }
    }

    /// Return the withdrawals for the payload or attributes.
    pub fn withdrawals(&self) -> Option<&Vec<reth_rpc_types::engine::payload::Withdrawal>> {
        match self {
            Self::ExecutionPayload { payload, .. } => payload.withdrawals(),
            Self::PayloadAttributes(attributes) => attributes.withdrawals(),
        }
    }

    /// Return the timestamp for the payload or attributes.
    pub fn timestamp(&self) -> u64 {
        match self {
            Self::ExecutionPayload { payload, .. } => payload.timestamp(),
            Self::PayloadAttributes(attributes) => attributes.timestamp(),
        }
    }

    /// Return the parent beacon block root for the payload or attributes.
    pub fn parent_beacon_block_root(&self) -> Option<B256> {
        match self {
            Self::ExecutionPayload { parent_beacon_block_root, .. } => *parent_beacon_block_root,
            Self::PayloadAttributes(attributes) => attributes.parent_beacon_block_root(),
        }
    }
}

impl<'a, AttributesType> From<&'a AttributesType> for PayloadOrAttributes<'a, AttributesType>
where
    AttributesType: PayloadAttributes,
{
    fn from(attributes: &'a AttributesType) -> Self {
        Self::PayloadAttributes(attributes)
    }
}

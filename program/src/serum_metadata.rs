#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(packed)]
pub struct SerumLeafNodeMetadata {
    tag: u32,
    owner_slot: u8,
    fee_tier: u8,
    padding: [u8; 2],
    owner: [u64; 4],
    client_order_id: u64,
}

#[repr(u8)]
pub enum FeeTier {
    Base,
    SRM2,
    SRM3,
    SRM4,
    SRM5,
    SRM6,
    MSRM,
    Stable,
}


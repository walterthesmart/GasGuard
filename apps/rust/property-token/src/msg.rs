#[cw_serde]
pub struct Auth {
    pub nonce: u64,
    pub expires_at: Option<u64>,
}

#[cw_serde]
pub enum ExecuteMsg {
    SetMetadata {
        token_id: String,
        metadata: PropertyMetadata,
        auth: Auth,
    },
    UpdateMetadata {
        token_id: String,
        metadata: PropertyMetadata,
        auth: Auth,
    },
    Batch {
        msgs: Vec<BatchMsg>,
        auth: Auth,
    },

    Batch {
        msgs: Vec<BatchMsg>,
    },
}

#[cw_serde]
pub enum BatchMsg {
    SetMetadata {
        token_id: String,
        metadata: PropertyMetadata,
    },
    UpdateMetadata {
        token_id: String,
        metadata: PropertyMetadata,
    },
}
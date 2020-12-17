pub mod poker {
    tonic::include_proto!("poker"); // The string specified here must match the proto package name
}

#[derive(PartialEq)]
pub enum ActionAllowed {
    NoAction, FoldOrCall, Any
}

#[derive(PartialEq)]
pub enum PlayerStatus {
    Playing, Folded, Allin
}

pub struct Player {
    uid: u32,
    name: String,
    balance: u64,
}


mod types;
use types::{V2, Float, Id};

pub enum ClientPacket {
    SpawnRequest {},
    Control {
        velocity: V2,
        position: V2,
        rotation: Float,
        shooting: bool,
    }
}
pub enum ServerPacket {
    Spawn {
        id: Id,
        t: Id,
        team: Id,
        health: Float,
        velocity: V2,
        position: V2,
        rotation: Float        
    },
    Update {
        id: Id,
        team: Id,
        health: Float,
        velocity: V2,
        position: V2,
        rotation: Float,
    },
    Remove {
        id: Id,
    },
    PlayerUpdate {
        id: Id,
    },
    Info {
        invite: Id,
    },
}


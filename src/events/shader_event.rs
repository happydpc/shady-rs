use crate::resources::CreationCandidate;
use bevy::prelude::{Entity, Vec2};
use shady_generator::ConnectionAttempt;

#[derive(Debug, Clone)]
pub enum ShaderEvent {
    CreateElement {
        target_position: Vec2,
        candidate: CreationCandidate,
    },
    DeleteNode {
        id: String,
    },
    DeleteInputProperty {
        id: String,
    },
    DeleteOutputProperty {
        id: String,
    },
    Connect {
        attempt: ConnectionAttempt,
        from: Entity,
        to: Entity,
    },
}
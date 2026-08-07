use crate::player::AI;

pub enum PlayerType {
    Human,
    AI(Box<dyn AI>),
}

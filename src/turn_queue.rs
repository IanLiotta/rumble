use legion::*;
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub struct TurnQueue {
    pub queue: VecDeque<Entity>,
}

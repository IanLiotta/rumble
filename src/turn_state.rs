#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TurnState {
    StartGame,
    AwaitingInput,
    PlayerTargeting,
    PlayerTurn,
    EnemyTurn,
}
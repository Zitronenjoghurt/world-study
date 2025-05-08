pub trait PersistentObject {
    type PersistentState;

    fn save_state(&self) -> Self::PersistentState;
    fn load_state(state: Self::PersistentState) -> Self;
}

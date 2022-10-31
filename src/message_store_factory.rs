pub trait MessageStoreFactory {
    fn create(&self) -> u32;
}

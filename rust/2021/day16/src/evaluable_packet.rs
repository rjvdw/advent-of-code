pub trait EvaluablePacket {
    fn eval(&self) -> u64;
}

pub trait IERC777Sender {
    fn tokensToSend(
        &mut self,
        operator: Address,
        from: Address,
        to: Address,
        amount: U256,
        userData: Bytes,
        operatorData: Bytes
    );
}
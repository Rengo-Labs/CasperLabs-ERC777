pub trait IERC777Recipient {
    fn tokensReceived(
        &mut self,
        operator: Address,
        from: Address,
        to: Address,
        amount: U256,
        data: Bytes,
        operatorData: Bytes
    );
}
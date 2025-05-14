use uint::construct_uint;

construct_uint! {
    /// 256-bit unsigned integer.
    pub struct U256(4);
}
// El crate que agregamos hace varias versiones, significa que esta función solo va a ser visible por este módulo, es decir para este contrato, pero que no se va a exponer a otro.
// El pub se pone para que sea visible para otros archivos.
pub(crate) fn proportional(amount: u128, numerator: u128, denominator: u128) -> u128 {
    (U256::from(amount) * U256::from(numerator) / U256::from(denominator)).as_u128()
}

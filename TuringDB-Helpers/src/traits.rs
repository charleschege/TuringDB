use async_trait::async_trait;

/// Ensures that a data structure is converted to bytes before it is sent over the wire
#[async_trait]
pub trait TuringPacket<'tp> {
    /// ### Converts a data structure into bytes in order to be sent over the wire
    /// #### Usage
    /// ```
    /// use crate::TuringPacket;
    /// 
    /// struct Foo;
    ///
    /// impl TuringPacket for Foo {
    ///     async fn into_packet(value: T) -> &'tp [u8] {
    ///         value.as_bytes()
    ///     }
    /// }
    /// ```
    async fn into_packet<T>(value: T) -> &'tp [u8];
}
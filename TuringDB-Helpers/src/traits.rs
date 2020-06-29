use async_trait::async_trait;

#[async_trait]
pub trait TuringPacket<'tp> {
    async fn into_packet() -> &'tp [u8];
}
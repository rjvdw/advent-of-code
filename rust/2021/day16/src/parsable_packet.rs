use rdcl_aoc_helpers::error::ParseError;

pub trait ParsablePacket: Sized {
    fn parse(
        bits: &str,
        position: usize,
        version: u8,
        type_id: u8,
    ) -> Result<(usize, Self), ParseError>;
}

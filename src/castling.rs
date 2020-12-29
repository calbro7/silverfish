pub enum CastleType {
    WhiteKingside = 1,
    WhiteQueenside = 2,
    BlackKingside = 4,
    BlackQueenside = 8
}

pub fn decode_castling(bits: u8, castle_type: CastleType) -> bool {
    (bits & castle_type as u8) != 0
}
pub mod att;
pub mod hci;

use nom::number::complete::le_u8;

named!(pub parse_uuid_128<&[u8], [u8; 16]>, map!(count!(le_u8, 16), |v| {
    let mut res: [u8; 16] = [0; 16];
    res.copy_from_slice(&v[0..16]);
    res
}));

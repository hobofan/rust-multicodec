extern crate unsigned_varint;
#[macro_use] extern crate failure;
extern crate serde;


use unsigned_varint::{decode::u16 as decode_varint,
                      encode::u16 as encode_varint,
                      decode::Error as VarintError};

use serde::{Serialize, Serializer, ser::SerializeSeq};

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Codec {} unknown", _0)]
    InvalidCodec(u16),
    #[fail(display = "Can't parse varint: {}", _0)]
    VarintFailed(VarintError)

}

macro_rules! build_codec_enum {
    {$( $val:expr => $var:ident, )*} => {
        #[allow(non_camel_case_types)]
        #[derive(PartialEq, Eq, Clone, Copy, Debug)]
        pub enum Codec {
            $( $var, )*
        }

        use Codec::*;

        impl Codec {
            /// Get the base code.
            pub fn code(&self) -> u16 {
                match *self {
                    $( $var => $val, )*
                }
            }

            /// Convert a code to a base.
            pub fn from_code(code: u16) -> Result<Codec, Error> {
                match code {
                    $( $val => Ok($var), )*
                    _ => Err(Error::InvalidCodec(code)),
                }
            }
        }
    }
}

// SOURCE: https://github.com/multiformats/multicodec/blob/master/table.csv
build_codec_enum! {
	0x55 => Bin,
    // bases encodings
    0x01 => Base1,
    0x07 => Base8,
    0x09 => Base10, 

    // serialization formats
    0x51 => Cbor,
    0x50 => Protobuf,
    0x60 => Rlp,
    0x63 => Bencode,

    // multiformats
    0x30 => Multicodec,
    0x31 => Multihash,
    0x32 => Multiaddr,
    0x33 => Multibase,

    // multihashes
    0x11 => Sha1,
    0x12 => Sha2_256,
    0x13 => Sha2_512,
    0x56 => Dbl_Sha2_256,
    0x17 => Sha3_224,
    0x16 => Sha3_256,
    0x15 => Sha3_384,
    0x14 => Sha3_512,
    0x18 => Shake_128,
    0x19 => Shake_256,
    0x1A => Keccak_224,
    0x1B => Keccak_256,
    0x1C => Keccak_384,
    0x1D => Keccak_512,
    0x22 => Murmur3,
    0xb201 => Blake2b_8,
    0xb202 => Blake2b_16,
    0xb203 => Blake2b_24,
    0xb204 => Blake2b_32,
    0xb205 => Blake2b_40,
    0xb206 => Blake2b_48,
    0xb207 => Blake2b_56,
    0xb208 => Blake2b_64,
    0xb209 => Blake2b_72,
    0xb20a => Blake2b_80,
    0xb20b => Blake2b_88,
    0xb20c => Blake2b_96,
    0xb20d => Blake2b_104,
    0xb20e => Blake2b_112,
    0xb20f => Blake2b_120,
    0xb210 => Blake2b_128,
    0xb211 => Blake2b_136,
    0xb212 => Blake2b_144,
    0xb213 => Blake2b_152,
    0xb214 => Blake2b_160,
    0xb215 => Blake2b_168,
    0xb216 => Blake2b_176,
    0xb217 => Blake2b_184,
    0xb218 => Blake2b_192,
    0xb219 => Blake2b_200,
    0xb21a => Blake2b_208,
    0xb21b => Blake2b_216,
    0xb21c => Blake2b_224,
    0xb21d => Blake2b_232,
    0xb21e => Blake2b_240,
    0xb21f => Blake2b_248,
    0xb220 => Blake2b_256,
    0xb221 => Blake2b_264,
    0xb222 => Blake2b_272,
    0xb223 => Blake2b_280,
    0xb224 => Blake2b_288,
    0xb225 => Blake2b_296,
    0xb226 => Blake2b_304,
    0xb227 => Blake2b_312,
    0xb228 => Blake2b_320,
    0xb229 => Blake2b_328,
    0xb22a => Blake2b_336,
    0xb22b => Blake2b_344,
    0xb22c => Blake2b_352,
    0xb22d => Blake2b_360,
    0xb22e => Blake2b_368,
    0xb22f => Blake2b_376,
    0xb230 => Blake2b_384,
    0xb231 => Blake2b_392,
    0xb232 => Blake2b_400,
    0xb233 => Blake2b_408,
    0xb234 => Blake2b_416,
    0xb235 => Blake2b_424,
    0xb236 => Blake2b_432,
    0xb237 => Blake2b_440,
    0xb238 => Blake2b_448,
    0xb239 => Blake2b_456,
    0xb23a => Blake2b_464,
    0xb23b => Blake2b_472,
    0xb23c => Blake2b_480,
    0xb23d => Blake2b_488,
    0xb23e => Blake2b_496,
    0xb23f => Blake2b_504,
    0xb240 => Blake2b_512,
    0xb241 => Blake2s_8,
    0xb242 => Blake2s_16,
    0xb243 => Blake2s_24,
    0xb244 => Blake2s_32,
    0xb245 => Blake2s_40,
    0xb246 => Blake2s_48,
    0xb247 => Blake2s_56,
    0xb248 => Blake2s_64,
    0xb249 => Blake2s_72,
    0xb24a => Blake2s_80,
    0xb24b => Blake2s_88,
    0xb24c => Blake2s_96,
    0xb24d => Blake2s_104,
    0xb24e => Blake2s_112,
    0xb24f => Blake2s_120,
    0xb250 => Blake2s_128,
    0xb251 => Blake2s_136,
    0xb252 => Blake2s_144,
    0xb253 => Blake2s_152,
    0xb254 => Blake2s_160,
    0xb255 => Blake2s_168,
    0xb256 => Blake2s_176,
    0xb257 => Blake2s_184,
    0xb258 => Blake2s_192,
    0xb259 => Blake2s_200,
    0xb25a => Blake2s_208,
    0xb25b => Blake2s_216,
    0xb25c => Blake2s_224,
    0xb25d => Blake2s_232,
    0xb25e => Blake2s_240,
    0xb25f => Blake2s_248,
    0xb260 => Blake2s_256,

    // multiaddrs
    0x04 => Ip4,
    0x29 => Ip6,
    0x06 => Tcp,
    0x0111 => Udp,
    0x21 => Dccp,
    0x84 => Sctp,
    0x012D => Udt,
    0x012E => Utp,
    0x01A5 => Ipfs,
    0x01E0 => Http,
    0x01BB => Https,
    0x01CC => Quic,
    0x01DD => Ws,
    0x01BC => Onion,
    0x0122 => P2p_Circuit,

    // IPLD formats
    0x70 => Dag_Pb, 
    0x71 => Dag_Cbor,

    0x78 => Git_Raw,

    0x90 => Eth_Block,
    0x91 => Eth_Block_List,
    0x92 => Eth_Tx_Trie,
    0x93 => Eth_Tx, 
    0x94 => Eth_Tx_Receipt_Trie,
    0x95 => Eth_Tx_Receipt,
    0x96 => Eth_State_Trie,
    0x97 => Eth_Account_Snapshot, 
    0x98 => Eth_Storage_Trie,

    0xb0 => Bitcoin_Block,
    0xb1 => Bitcoin_Tx,

    0xc0 => Zcash_Block,
    0xc1 => Zcash_Tx,

    0xd0 => Stellar_Block,
    0xd1 => Stellar_Tx,

    0x7b => Torrent_Info,
    0x7c => Torrent_File,
    0xed => Ed25519_Pub,
}


#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct MultiCodec<'a> {
	codec: Codec,
	data: &'a [u8]
}

impl<'a> MultiCodec<'a> {
	/// create a new MultiCodec
	pub fn new(codec: Codec, data: &'a [u8]) -> MultiCodec {
		MultiCodec { codec, data }
	}
	/// try to parse a MultiCodec from a packed bytestring
	pub fn from(packed: &'a [u8]) -> Result<MultiCodec, Error> {
		let (code, data) = decode_varint(packed).map_err(|e| Error::VarintFailed(e))?;
		let codec = Codec::from_code(code)?;
		Ok(MultiCodec { codec, data })
	}

    pub fn pack(&self) -> Vec<u8> {
        let mut buf = [0u8; 3];
        encode_varint(self.codec.code(), &mut buf);
        let mut v : Vec<u8> = Vec::new();
        for b in &buf {
            v.push(*b);
            // varint uses first bit to indicate another byte follows, stop if not the case
            if *b <= 127 { break }
        }
        v.extend(self.data);
        v
    }
}

impl<'a> Serialize for MultiCodec<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_bytes(&self.pack())
    }
}



#[cfg(test)]
mod tests {
	use super::*;
    #[test]
    fn it_works() {
        let codec = MultiCodec::new(Codec::Sha2_256, b"EiC5TSe5k00");
        let packed = codec.pack();
        let redo = MultiCodec::from(&packed).unwrap();
        assert_eq!(packed, "\x12EiC5TSe5k00".as_bytes());
        assert_eq!(redo, codec);
    }
}

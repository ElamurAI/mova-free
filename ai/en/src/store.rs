//! Trained model file: our own binary format via serde. Training takes ~40 s, while the file
//! loads in a fraction of a second — for real-time tagging on a CPU.
//!
//! The format is not self-describing: struct fields go in order, without names.
//! - Numbers are fixed-width little-endian. Enum variant indices and lengths are LEB128.
//! - Strings and bytes are length plus content. Large number arrays are one chunk of bytes (`f32s`, `u64s`, `bytes`).
//!
//! File = header + body:
//! - header: magic `MOVA-EN\0`, format version, enum sizes (Tag, Rel, UPos, Feat);
//! - further in the header: fingerprint of the in-code lexicon (lemma ids in the model come from it), body length and checksum.
//!
//! Any mismatch is a load error, not silently different tagging: truncated file, foreign version,
//! different lexicon, corrupted byte, trailing bytes.

use std::fmt;

use serde::de::{self, DeserializeSeed, IntoDeserializer, SeqAccess, Visitor};
use serde::ser::{self, SerializeTuple};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::gram::{Feat, Rel, Tag, UPos};

/// (De)serialization error.
#[derive(Debug)]
pub struct Error(pub String);

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for Error {}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(m: T) -> Self {
        Error(m.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(m: T) -> Self {
        Error(m.to_string())
    }
}

type R<T> = Result<T, Error>;

// ---------------------------------------------------------------- writing

fn varint(out: &mut Vec<u8>, mut x: u64) {
    while x >= 0x80 {
        out.push(x as u8 | 0x80);
        x >>= 7;
    }
    out.push(x as u8);
}

/// Value to format bytes.
pub fn to_bytes<T: Serialize + ?Sized>(v: &T) -> R<Vec<u8>> {
    let mut s = Ser { out: Vec::new() };
    v.serialize(&mut s)?;
    Ok(s.out)
}

struct Ser {
    out: Vec<u8>,
}

impl Ser {
    fn raw(&mut self, b: &[u8]) {
        self.out.extend_from_slice(b);
    }
    fn len(&mut self, n: Option<usize>) -> R<()> {
        let n = n.ok_or_else(|| Error("sequence length not known in advance".into()))?;
        varint(&mut self.out, n as u64);
        Ok(())
    }
}

impl<'a> Serializer for &'a mut Ser {
    type Ok = ();
    type Error = Error;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> R<()> {
        self.out.push(v as u8);
        Ok(())
    }
    fn serialize_i8(self, v: i8) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i16(self, v: i16) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i32(self, v: i32) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i64(self, v: i64) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_i128(self, v: i128) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u8(self, v: u8) -> R<()> {
        self.out.push(v);
        Ok(())
    }
    fn serialize_u16(self, v: u16) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u32(self, v: u32) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u64(self, v: u64) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_u128(self, v: u128) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_f32(self, v: f32) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_f64(self, v: f64) -> R<()> {
        self.raw(&v.to_le_bytes());
        Ok(())
    }
    fn serialize_char(self, v: char) -> R<()> {
        varint(&mut self.out, v as u64);
        Ok(())
    }
    fn serialize_str(self, v: &str) -> R<()> {
        self.serialize_bytes(v.as_bytes())
    }
    fn serialize_bytes(self, v: &[u8]) -> R<()> {
        varint(&mut self.out, v.len() as u64);
        self.raw(v);
        Ok(())
    }
    fn serialize_none(self) -> R<()> {
        self.out.push(0);
        Ok(())
    }
    fn serialize_some<T: Serialize + ?Sized>(self, v: &T) -> R<()> {
        self.out.push(1);
        v.serialize(self)
    }
    fn serialize_unit(self) -> R<()> {
        Ok(())
    }
    fn serialize_unit_struct(self, _: &'static str) -> R<()> {
        Ok(())
    }
    fn serialize_unit_variant(self, _: &'static str, i: u32, _: &'static str) -> R<()> {
        varint(&mut self.out, i as u64);
        Ok(())
    }
    fn serialize_newtype_struct<T: Serialize + ?Sized>(self, _: &'static str, v: &T) -> R<()> {
        v.serialize(self)
    }
    fn serialize_newtype_variant<T: Serialize + ?Sized>(self, _: &'static str, i: u32, _: &'static str, v: &T) -> R<()> {
        varint(&mut self.out, i as u64);
        v.serialize(self)
    }
    fn serialize_seq(self, n: Option<usize>) -> R<Self> {
        self.len(n)?;
        Ok(self)
    }
    fn serialize_tuple(self, _: usize) -> R<Self> {
        Ok(self)
    }
    fn serialize_tuple_struct(self, _: &'static str, _: usize) -> R<Self> {
        Ok(self)
    }
    fn serialize_tuple_variant(self, _: &'static str, i: u32, _: &'static str, _: usize) -> R<Self> {
        varint(&mut self.out, i as u64);
        Ok(self)
    }
    fn serialize_map(self, n: Option<usize>) -> R<Self> {
        self.len(n)?;
        Ok(self)
    }
    fn serialize_struct(self, _: &'static str, _: usize) -> R<Self> {
        Ok(self)
    }
    fn serialize_struct_variant(self, _: &'static str, i: u32, _: &'static str, _: usize) -> R<Self> {
        varint(&mut self.out, i as u64);
        Ok(self)
    }
    fn is_human_readable(&self) -> bool {
        false
    }
}

macro_rules! compound {
    ($($tr:ident :: $f:ident),+) => {$(
        impl<'a> ser::$tr for &'a mut Ser {
            type Ok = ();
            type Error = Error;
            fn $f<T: Serialize + ?Sized>(&mut self, v: &T) -> R<()> {
                v.serialize(&mut **self)
            }
            fn end(self) -> R<()> {
                Ok(())
            }
        }
    )+};
}
compound!(SerializeSeq::serialize_element, SerializeTuple::serialize_element, SerializeTupleStruct::serialize_field, SerializeTupleVariant::serialize_field);

impl<'a> ser::SerializeMap for &'a mut Ser {
    type Ok = ();
    type Error = Error;
    fn serialize_key<T: Serialize + ?Sized>(&mut self, k: &T) -> R<()> {
        k.serialize(&mut **self)
    }
    fn serialize_value<T: Serialize + ?Sized>(&mut self, v: &T) -> R<()> {
        v.serialize(&mut **self)
    }
    fn end(self) -> R<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Ser {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, _: &'static str, v: &T) -> R<()> {
        v.serialize(&mut **self)
    }
    fn end(self) -> R<()> {
        Ok(())
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Ser {
    type Ok = ();
    type Error = Error;
    fn serialize_field<T: Serialize + ?Sized>(&mut self, _: &'static str, v: &T) -> R<()> {
        v.serialize(&mut **self)
    }
    fn end(self) -> R<()> {
        Ok(())
    }
}

// ---------------------------------------------------------------- reading

/// Value from format bytes; the bytes must end exactly where the value ends.
pub fn from_bytes<'de, T: Deserialize<'de>>(b: &'de [u8]) -> R<T> {
    let mut d = De { b, pos: 0 };
    let v = T::deserialize(&mut d)?;
    if d.pos != b.len() {
        return Err(Error(format!("trailing bytes: {} of {}", b.len() - d.pos, b.len())));
    }
    Ok(v)
}

struct De<'de> {
    b: &'de [u8],
    pos: usize,
}

impl<'de> De<'de> {
    fn take(&mut self, n: usize) -> R<&'de [u8]> {
        if self.b.len() - self.pos < n {
            return Err(Error(format!("file truncated: need {n} bytes at position {}, have {}", self.pos, self.b.len() - self.pos)));
        }
        let s = &self.b[self.pos..self.pos + n];
        self.pos += n;
        Ok(s)
    }
    fn arr<const N: usize>(&mut self) -> R<[u8; N]> {
        Ok(self.take(N)?.try_into().expect("N bytes"))
    }
    fn varint(&mut self) -> R<u64> {
        let mut x = 0u64;
        for k in 0..10 {
            let b = self.take(1)?[0];
            x |= ((b & 0x7f) as u64) << (7 * k);
            if b & 0x80 == 0 {
                return Ok(x);
            }
        }
        Err(Error(format!("corrupted number at position {}", self.pos)))
    }
    fn len(&mut self) -> R<usize> {
        let n = self.varint()?;
        usize::try_from(n).map_err(|_| Error(format!("length {n} too large")))
    }
    fn bytes(&mut self) -> R<&'de [u8]> {
        let n = self.len()?;
        self.take(n)
    }
}

/// Sequence of known length: seq elements, struct fields, map pairs.
struct Items<'a, 'de> {
    de: &'a mut De<'de>,
    left: usize,
}

impl<'a, 'de> SeqAccess<'de> for Items<'a, 'de> {
    type Error = Error;
    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> R<Option<T::Value>> {
        if self.left == 0 {
            return Ok(None);
        }
        self.left -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.left)
    }
}

impl<'a, 'de> de::MapAccess<'de> for Items<'a, 'de> {
    type Error = Error;
    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> R<Option<K::Value>> {
        if self.left == 0 {
            return Ok(None);
        }
        self.left -= 1;
        seed.deserialize(&mut *self.de).map(Some)
    }
    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> R<V::Value> {
        seed.deserialize(&mut *self.de)
    }
    fn size_hint(&self) -> Option<usize> {
        Some(self.left)
    }
}

impl<'a, 'de> de::EnumAccess<'de> for &'a mut De<'de> {
    type Error = Error;
    type Variant = Self;
    fn variant_seed<V: DeserializeSeed<'de>>(self, seed: V) -> R<(V::Value, Self)> {
        let i = u32::try_from(self.varint()?).map_err(|_| Error("variant index too large".into()))?;
        let v = seed.deserialize(IntoDeserializer::<Error>::into_deserializer(i))?;
        Ok((v, self))
    }
}

impl<'a, 'de> de::VariantAccess<'de> for &'a mut De<'de> {
    type Error = Error;
    fn unit_variant(self) -> R<()> {
        Ok(())
    }
    fn newtype_variant_seed<T: DeserializeSeed<'de>>(self, seed: T) -> R<T::Value> {
        seed.deserialize(self)
    }
    fn tuple_variant<V: Visitor<'de>>(self, len: usize, v: V) -> R<V::Value> {
        v.visit_seq(Items { de: self, left: len })
    }
    fn struct_variant<V: Visitor<'de>>(self, fields: &'static [&'static str], v: V) -> R<V::Value> {
        v.visit_seq(Items { de: self, left: fields.len() })
    }
}

macro_rules! num {
    ($($f:ident $t:ty => $visit:ident),+ $(,)?) => {$(
        fn $f<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
            v.$visit(<$t>::from_le_bytes(self.arr()?))
        }
    )+};
}

impl<'a, 'de> Deserializer<'de> for &'a mut De<'de> {
    type Error = Error;

    num! {
        deserialize_i8 i8 => visit_i8, deserialize_i16 i16 => visit_i16, deserialize_i32 i32 => visit_i32,
        deserialize_i64 i64 => visit_i64, deserialize_i128 i128 => visit_i128,
        deserialize_u8 u8 => visit_u8, deserialize_u16 u16 => visit_u16, deserialize_u32 u32 => visit_u32,
        deserialize_u64 u64 => visit_u64, deserialize_u128 u128 => visit_u128,
        deserialize_f32 f32 => visit_f32, deserialize_f64 f64 => visit_f64,
    }

    fn deserialize_any<V: Visitor<'de>>(self, _: V) -> R<V::Value> {
        Err(Error("format is not self-describing: the type must be known".into()))
    }
    fn deserialize_ignored_any<V: Visitor<'de>>(self, _: V) -> R<V::Value> {
        Err(Error("format is not self-describing: cannot skip a value".into()))
    }
    fn deserialize_bool<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        match self.take(1)?[0] {
            0 => v.visit_bool(false),
            1 => v.visit_bool(true),
            b => Err(Error(format!("bool = {b}"))),
        }
    }
    fn deserialize_char<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        let c = self.varint()?;
        let c = u32::try_from(c).ok().and_then(char::from_u32).ok_or_else(|| Error(format!("character {c}")))?;
        v.visit_char(c)
    }
    fn deserialize_str<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        let b = self.bytes()?;
        v.visit_borrowed_str(std::str::from_utf8(b).map_err(|e| Error(format!("string is not UTF-8: {e}")))?)
    }
    fn deserialize_string<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        self.deserialize_str(v)
    }
    fn deserialize_bytes<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        v.visit_borrowed_bytes(self.bytes()?)
    }
    fn deserialize_byte_buf<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        self.deserialize_bytes(v)
    }
    fn deserialize_option<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        match self.take(1)?[0] {
            0 => v.visit_none(),
            1 => v.visit_some(self),
            b => Err(Error(format!("Option = {b}"))),
        }
    }
    fn deserialize_unit<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        v.visit_unit()
    }
    fn deserialize_unit_struct<V: Visitor<'de>>(self, _: &'static str, v: V) -> R<V::Value> {
        v.visit_unit()
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(self, _: &'static str, v: V) -> R<V::Value> {
        v.visit_newtype_struct(self)
    }
    fn deserialize_seq<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        let left = self.len()?;
        v.visit_seq(Items { de: self, left })
    }
    fn deserialize_tuple<V: Visitor<'de>>(self, len: usize, v: V) -> R<V::Value> {
        v.visit_seq(Items { de: self, left: len })
    }
    fn deserialize_tuple_struct<V: Visitor<'de>>(self, _: &'static str, len: usize, v: V) -> R<V::Value> {
        v.visit_seq(Items { de: self, left: len })
    }
    fn deserialize_map<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        let left = self.len()?;
        v.visit_map(Items { de: self, left })
    }
    fn deserialize_struct<V: Visitor<'de>>(self, _: &'static str, fields: &'static [&'static str], v: V) -> R<V::Value> {
        v.visit_seq(Items { de: self, left: fields.len() })
    }
    fn deserialize_enum<V: Visitor<'de>>(self, _: &'static str, _: &'static [&'static str], v: V) -> R<V::Value> {
        v.visit_enum(self)
    }
    fn deserialize_identifier<V: Visitor<'de>>(self, v: V) -> R<V::Value> {
        let i = self.varint()?;
        v.visit_u64(i)
    }
    fn is_human_readable(&self) -> bool {
        false
    }
}

// ---------------------------------------------------------------- large arrays as one chunk

/// Bytes as is (`#[serde(with = "crate::store::bytes")]`), not a sequence of u8 one by one.
pub mod bytes {
    use super::*;

    pub fn serialize<S: Serializer>(v: &[u8], s: S) -> Result<S::Ok, S::Error> {
        s.serialize_bytes(v)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<u8>, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Vec<u8>;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("bytes")
            }
            fn visit_bytes<E: de::Error>(self, b: &[u8]) -> Result<Vec<u8>, E> {
                Ok(b.to_vec())
            }
        }
        d.deserialize_bytes(V)
    }
}

macro_rules! chunked {
    ($(#[$m:meta])* $name:ident $t:ty) => {
        $(#[$m])*
        pub mod $name {
            use super::*;
            const W: usize = std::mem::size_of::<$t>();

            pub fn serialize<S: Serializer>(v: &[$t], s: S) -> Result<S::Ok, S::Error> {
                let mut b = Vec::with_capacity(v.len() * W);
                for x in v {
                    b.extend_from_slice(&x.to_le_bytes());
                }
                s.serialize_bytes(&b)
            }

            pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<$t>, D::Error> {
                struct V;
                impl<'de> Visitor<'de> for V {
                    type Value = Vec<$t>;
                    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                        write!(f, "bytes, a multiple of {}", W)
                    }
                    fn visit_bytes<E: de::Error>(self, b: &[u8]) -> Result<Vec<$t>, E> {
                        if b.len() % W != 0 {
                            return Err(E::custom(format!("{} bytes are not a multiple of {}", b.len(), W)));
                        }
                        Ok(b.chunks_exact(W).map(|c| <$t>::from_le_bytes(c.try_into().expect("W bytes"))).collect())
                    }
                }
                d.deserialize_bytes(V)
            }
        }
    };
}

chunked!(
    /// `Vec<f32>` as one chunk (`#[serde(with = "crate::store::f32s")]`): bits as is, no rounding.
    f32s f32
);
chunked!(
    /// `Vec<u64>` as one chunk (`#[serde(with = "crate::store::u64s")]`).
    u64s u64
);

/// Fixed-length array of any size (serde itself only handles up to 32):
/// `#[serde(with = "crate::store::arr")]`.
pub mod arr {
    use super::*;
    use std::marker::PhantomData;

    pub fn serialize<S: Serializer, T: Serialize, const N: usize>(a: &[T; N], s: S) -> Result<S::Ok, S::Error> {
        let mut t = s.serialize_tuple(N)?;
        for x in a {
            t.serialize_element(x)?;
        }
        t.end()
    }

    pub fn deserialize<'de, D: Deserializer<'de>, T: Deserialize<'de> + Copy + Default, const N: usize>(d: D) -> Result<[T; N], D::Error> {
        struct V<T, const N: usize>(PhantomData<T>);
        impl<'de, T: Deserialize<'de> + Copy + Default, const N: usize> Visitor<'de> for V<T, N> {
            type Value = [T; N];
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "array of {N} elements")
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut a: A) -> Result<[T; N], A::Error> {
                let mut out = [T::default(); N];
                for (i, x) in out.iter_mut().enumerate() {
                    *x = a.next_element()?.ok_or_else(|| de::Error::invalid_length(i, &self))?;
                }
                Ok(out)
            }
        }
        d.deserialize_tuple(N, V::<T, N>(PhantomData))
    }
}

// ---------------------------------------------------------------- file: header and body

const MAGIC: &[u8; 8] = b"MOVA-EN\0";
/// Format version: bump whenever any model structure changes.
pub const FORMAT: u32 = 1;
const HEADER: usize = 8 + 4 + 4 * 2 + 8 + 8 + 8;

/// Body checksum: hash over 8-byte words (the same as in the model tables) and the length.
pub(crate) fn checksum(b: &[u8]) -> u64 {
    use std::hash::Hasher;
    let mut h = crate::hash::Fx::default();
    h.write(b);
    h.write_usize(b.len());
    h.finish()
}

fn shape() -> [u16; 4] {
    [Tag::N as u16, Rel::N as u16, UPos::N as u16, Feat::N as u16]
}

/// File: header (magic, version, enum sizes, lexicon fingerprint, body length and checksum) + body.
pub fn pack<T: Serialize + ?Sized>(v: &T) -> R<Vec<u8>> {
    let body = to_bytes(v)?;
    let mut out = Vec::with_capacity(HEADER + body.len());
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&FORMAT.to_le_bytes());
    for n in shape() {
        out.extend_from_slice(&n.to_le_bytes());
    }
    out.extend_from_slice(&crate::dict::fingerprint().to_le_bytes());
    out.extend_from_slice(&(body.len() as u64).to_le_bytes());
    out.extend_from_slice(&checksum(&body).to_le_bytes());
    out.extend_from_slice(&body);
    Ok(out)
}

/// Value from a `pack` file; checks the header and checksum before parsing the body.
pub fn unpack<'de, T: Deserialize<'de>>(b: &'de [u8]) -> R<T> {
    if b.len() < HEADER || &b[..8] != MAGIC {
        return Err(Error("not an en model file (no MOVA-EN magic)".into()));
    }
    let u16_at = |o: usize| u16::from_le_bytes([b[o], b[o + 1]]);
    let u64_at = |o: usize| u64::from_le_bytes(b[o..o + 8].try_into().expect("8 bytes"));
    let format = u32::from_le_bytes(b[8..12].try_into().expect("4 bytes"));
    if format != FORMAT {
        return Err(Error(format!("format version {format}, but this build reads {FORMAT} — retrain the model (`en train`)")));
    }
    let got = [u16_at(12), u16_at(14), u16_at(16), u16_at(18)];
    if got != shape() {
        return Err(Error(format!("enum sizes Tag/Rel/UPos/Feat {got:?}, but this build has {:?} — retrain the model", shape())));
    }
    let (dict, len, sum) = (u64_at(20), u64_at(28), u64_at(36));
    if dict != crate::dict::fingerprint() {
        return Err(Error("the model was trained with a different in-code lexicon (data/) — retrain the model".into()));
    }
    let body = &b[HEADER..];
    if body.len() as u64 != len {
        return Err(Error(format!("body is {} bytes, but the header says {len} — file truncated or appended to", body.len())));
    }
    if checksum(body) != sum {
        return Err(Error("checksum mismatch — file corrupted".into()));
    }
    from_bytes(body)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gram::Feats;
    use crate::hash::FastMap;

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    enum E {
        A,
        B(u8),
        C { x: i16, y: String },
        D(Option<Tag>, f64),
    }

    #[derive(Serialize, Deserialize, PartialEq, Debug)]
    struct S {
        e: Vec<E>,
        m: FastMap<u64, Vec<(u8, f32)>>,
        #[serde(with = "f32s")]
        w: Vec<f32>,
        #[serde(with = "arr")]
        big: [bool; 40],
        f: Feats,
        r: Rel,
        t: (char, u128, i64, bool),
    }

    fn sample() -> S {
        let mut m = FastMap::default();
        m.insert(7, vec![(1, -0.5), (3, 2.25)]);
        m.insert(u64::MAX, vec![]);
        let mut big = [false; 40];
        big[39] = true;
        S {
            e: vec![E::A, E::B(9), E::C { x: -300, y: "lémma ’s".into() }, E::D(Some(Tag::PRPS), f64::MIN_POSITIVE), E::D(None, -0.0)],
            m,
            w: vec![0.0, -0.0, 1.5e-30, f32::MAX, -7.25],
            big,
            f: Feats(1 << 100 | 5),
            r: Rel::NsubjPass,
            t: ('ï', u128::MAX - 1, i64::MIN, true),
        }
    }

    /// Round trip gives the same value to the bit; -0.0 stays -0.0. Bytes of a re-write may
    /// differ in the order of map pairs (hash map iteration order) — the value does not change.
    #[test]
    fn round_trip_is_exact() {
        let s = sample();
        let b = to_bytes(&s).unwrap();
        let back: S = from_bytes(&b).unwrap();
        assert_eq!(back, s);
        assert_eq!(back.w[1].to_bits(), (-0.0f32).to_bits());
    }

    /// Negative control: truncated body, extra byte, foreign tag index, corrupted file byte,
    /// foreign version — an error, not a silently different value.
    #[test]
    fn broken_bytes_fail() {
        let b = to_bytes(&sample()).unwrap();
        assert!(from_bytes::<S>(&b[..b.len() - 1]).is_err());
        let mut long = b.clone();
        long.push(0);
        assert!(from_bytes::<S>(&long).unwrap_err().0.contains("trailing bytes"));
        assert!(from_bytes::<Tag>(&[Tag::N as u8]).is_err());
        let file = pack(&sample()).unwrap();
        assert_eq!(unpack::<S>(&file).unwrap(), sample());
        for i in [0, 8, 12, 20, 28, 36, HEADER, file.len() - 1] {
            let mut bad = file.clone();
            bad[i] ^= 0x10;
            assert!(unpack::<S>(&bad).is_err(), "byte {i} corrupted, yet the file was read"
);
        }
        assert!(unpack::<S>(&file[..file.len() - 3]).is_err());
    }
}

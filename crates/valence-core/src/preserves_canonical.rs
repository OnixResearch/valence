//! Pure bounded Preserves 0.996.3 text parser, canonical binary encoder, and
//! strict canonical binary decoder.
//!
//! Normative references:
//! - Binary syntax: <https://preserves.dev/preserves-binary.html>
//! - Canonical binary form: <https://preserves.dev/canonical-binary.html>
//! - Text syntax: <https://preserves.dev/preserves-text.html>
//!
//! Admitted application profile subset:
//! - Booleans (`#f` = `0x80`, `#t` = `0x81`)
//! - Arbitrary-precision signed integers (`0xB0` + minimal varint + minimal big-endian
//!   two's-complement bytes; `0` encodes as `0xB0 0x00`)
//! - UTF-8 strings (`0xB1` + minimal varint + UTF-8 bytes)
//! - Byte strings (`0xB2` + minimal varint + raw bytes)
//! - UTF-8 symbols (`0xB3` + minimal varint + UTF-8 bytes)
//! - Records (`0xB4` + label + fields + `0x84`)
//! - Sequences (`0xB5` + elements + `0x84`)
//! - Sets (`0xB6` + canonical-encoded-byte-sorted distinct elements + `0x84`)
//! - Dictionaries (`0xB7` + canonical-encoded-byte-sorted distinct key/value pairs + `0x84`)
//!
//! Explicitly rejected by this profile:
//! - Annotations (`@`, `;`, `0x85`)
//! - Embedded values (`#:`, `0x86`)
//! - IEEE 754 floats (`0x87`, float literals)
//! - Duplicate set members or dictionary keys
//! - Non-minimal varint or signed-integer encodings
//! - Non-canonical binary set/dictionary ordering
//! - Trailing bytes or trailing text tokens
//! - Any input/output exceeding configured [`PreservesLimits`]

use serde::Deserialize;
use serde::Serialize;

/// Preserves specification version implemented by this bounded codec.
pub const PRESERVES_SPEC_VERSION: &str = "0.996.3";

const TAG_FALSE: u8 = 0x80;
const TAG_TRUE: u8 = 0x81;
const TAG_END: u8 = 0x84;
const TAG_ANNOTATION: u8 = 0x85;
const TAG_EMBEDDED: u8 = 0x86;
const TAG_FLOAT: u8 = 0x87;
const TAG_SIGNED_INTEGER: u8 = 0xB0;
const TAG_STRING: u8 = 0xB1;
const TAG_BYTE_STRING: u8 = 0xB2;
const TAG_SYMBOL: u8 = 0xB3;
const TAG_RECORD: u8 = 0xB4;
const TAG_SEQUENCE: u8 = 0xB5;
const TAG_SET: u8 = 0xB6;
const TAG_DICTIONARY: u8 = 0xB7;

const DEFAULT_MAX_INPUT_BYTES: usize = 1_048_576;
const DEFAULT_MAX_OUTPUT_BYTES: usize = 2_097_152;
const DEFAULT_MAX_DEPTH: usize = 64;
const DEFAULT_MAX_COLLECTION_ITEMS: usize = 4_096;
const DEFAULT_MAX_ATOM_BYTES: usize = 262_144;
const DEFAULT_MAX_VALUES: usize = 65_536;

/// Explicit resource ceilings enforced by the bounded Preserves codec.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct PreservesLimits {
    pub max_input_bytes: usize,
    pub max_output_bytes: usize,
    pub max_depth: usize,
    pub max_collection_items: usize,
    pub max_atom_bytes: usize,
    pub max_values: usize,
}

impl Default for PreservesLimits {
    fn default() -> Self {
        Self {
            max_input_bytes: DEFAULT_MAX_INPUT_BYTES,
            max_output_bytes: DEFAULT_MAX_OUTPUT_BYTES,
            max_depth: DEFAULT_MAX_DEPTH,
            max_collection_items: DEFAULT_MAX_COLLECTION_ITEMS,
            max_atom_bytes: DEFAULT_MAX_ATOM_BYTES,
            max_values: DEFAULT_MAX_VALUES,
        }
    }
}

impl PreservesLimits {
    /// Validate that every configured limit is nonzero.
    pub const fn validate(&self) -> Result<(), PreservesCodecError> {
        if self.max_input_bytes == 0 {
            return Err(PreservesCodecError::InvalidLimit("max_input_bytes"));
        }
        if self.max_output_bytes == 0 {
            return Err(PreservesCodecError::InvalidLimit("max_output_bytes"));
        }
        if self.max_depth == 0 {
            return Err(PreservesCodecError::InvalidLimit("max_depth"));
        }
        if self.max_collection_items == 0 {
            return Err(PreservesCodecError::InvalidLimit("max_collection_items"));
        }
        if self.max_atom_bytes == 0 {
            return Err(PreservesCodecError::InvalidLimit("max_atom_bytes"));
        }
        if self.max_values == 0 {
            return Err(PreservesCodecError::InvalidLimit("max_values"));
        }
        Ok(())
    }
}

/// Deterministic error taxonomy for bounded Preserves text and canonical binary
/// parsing, decoding, and encoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreservesCodecError {
    InvalidLimit(&'static str),
    InputLimitExceeded,
    OutputLimitExceeded,
    DepthLimitExceeded,
    CollectionLimitExceeded,
    AtomLimitExceeded,
    ValueLimitExceeded,
    UnsupportedAnnotation,
    UnsupportedEmbeddedValue,
    UnsupportedFloat,
    UnsupportedTag(u8),
    NonMinimalVarint,
    NonMinimalInteger,
    DuplicateSetMember,
    DuplicateDictionaryKey,
    NonCanonicalSetOrder,
    NonCanonicalDictionaryOrder,
    TrailingBytes,
    TrailingInput,
    UnexpectedEof,
    EmptyRecord,
    OddDictionaryEntries,
    InvalidUtf8,
    MalformedSyntax(&'static str),
}

impl PreservesCodecError {
    /// Stable machine-readable diagnostic code.
    #[must_use]
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidLimit(_) => "invalid_limit",
            Self::InputLimitExceeded => "input_limit_exceeded",
            Self::OutputLimitExceeded => "output_limit_exceeded",
            Self::DepthLimitExceeded => "depth_limit_exceeded",
            Self::CollectionLimitExceeded => "collection_limit_exceeded",
            Self::AtomLimitExceeded => "atom_limit_exceeded",
            Self::ValueLimitExceeded => "value_limit_exceeded",
            Self::UnsupportedAnnotation => "unsupported_annotation",
            Self::UnsupportedEmbeddedValue => "unsupported_embedded_value",
            Self::UnsupportedFloat => "unsupported_float",
            Self::UnsupportedTag(_) => "unsupported_tag",
            Self::NonMinimalVarint => "nonminimal_varint",
            Self::NonMinimalInteger => "nonminimal_integer",
            Self::DuplicateSetMember => "duplicate_set_member",
            Self::DuplicateDictionaryKey => "duplicate_dictionary_key",
            Self::NonCanonicalSetOrder => "noncanonical_set_order",
            Self::NonCanonicalDictionaryOrder => "noncanonical_dictionary_order",
            Self::TrailingBytes => "trailing_bytes",
            Self::TrailingInput => "trailing_input",
            Self::UnexpectedEof => "unexpected_eof",
            Self::EmptyRecord => "empty_record",
            Self::OddDictionaryEntries => "odd_dictionary_entries",
            Self::InvalidUtf8 => "invalid_utf8",
            Self::MalformedSyntax(_) => "malformed_preserves_syntax",
        }
    }
}

/// Arbitrary-precision signed integer stored in minimal big-endian
/// two's-complement byte form per Preserves 0.996.3.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedInt {
    be_bytes: Vec<u8>,
}

impl SignedInt {
    /// Zero encodes as the empty two's-complement byte sequence (`0xB0 0x00`).
    #[must_use]
    pub const fn zero() -> Self {
        Self { be_bytes: Vec::new() }
    }

    /// Construct from an `i128` using minimal big-endian two's-complement bytes.
    #[must_use]
    pub fn from_i128(value: i128) -> Self {
        if value == 0 {
            return Self::zero();
        }
        let raw = value.to_be_bytes();
        let trimmed = trim_minimal_twos_complement(&raw);
        Self { be_bytes: trimmed }
    }

    /// Construct from big-endian two's-complement bytes, rejecting any
    /// non-minimal representation.
    pub fn from_minimal_be_bytes(bytes: Vec<u8>) -> Result<Self, PreservesCodecError> {
        if !is_minimal_twos_complement(&bytes) {
            return Err(PreservesCodecError::NonMinimalInteger);
        }
        Ok(Self { be_bytes: bytes })
    }

    /// Parse a decimal integer string (arbitrary precision) into minimal
    /// big-endian two's-complement bytes.
    pub fn from_decimal_str(text: &str) -> Result<Self, PreservesCodecError> {
        let (negative, digits) = match text.as_bytes().first() {
            Some(b'-') => (true, &text[1..]),
            Some(b'+') => (false, &text[1..]),
            Some(_) => (false, text),
            None => return Err(PreservesCodecError::MalformedSyntax("empty_integer")),
        };
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(PreservesCodecError::MalformedSyntax("invalid_integer"));
        }
        let mut mag: Vec<u8> = Vec::new();
        for digit in digits.bytes() {
            let mut carry = u16::from(digit - b'0');
            for byte in mag.iter_mut().rev() {
                let acc = u16::from(*byte) * 10 + carry;
                *byte = (acc & 0xFF) as u8;
                carry = acc >> 8;
            }
            if carry > 0 {
                mag.insert(0, carry as u8);
            }
        }
        let first_nonzero = mag.iter().position(|&b| b != 0);
        let Some(pos) = first_nonzero else {
            return Ok(Self::zero());
        };
        let mag = &mag[pos..];
        let be_bytes = if negative {
            negate_magnitude_twos_complement(mag)
        } else {
            let mut out = Vec::with_capacity(mag.len() + 1);
            if (mag[0] & 0x80) != 0 {
                out.push(0x00);
            }
            out.extend_from_slice(mag);
            out
        };
        Ok(Self { be_bytes })
    }

    /// Minimal big-endian two's-complement byte representation.
    #[must_use]
    pub fn as_be_bytes(&self) -> &[u8] {
        &self.be_bytes
    }

    /// Convert to `i128` if it fits in 16 signed bytes.
    #[must_use]
    pub fn to_i128(&self) -> Option<i128> {
        if self.be_bytes.is_empty() {
            return Some(0);
        }
        if self.be_bytes.len() > 16 {
            return None;
        }
        let sign_fill = if (self.be_bytes[0] & 0x80) != 0 { 0xFF } else { 0x00 };
        let mut buf = [sign_fill; 16];
        let start = 16 - self.be_bytes.len();
        buf[start..].copy_from_slice(&self.be_bytes);
        Some(i128::from_be_bytes(buf))
    }
}

fn is_minimal_twos_complement(bytes: &[u8]) -> bool {
    match bytes {
        [] => true,
        [0x00] => false,
        [_] => true,
        [0x00, second, ..] => (second & 0x80) != 0,
        [0xFF, second, ..] => (second & 0x80) == 0,
        _ => true,
    }
}

fn trim_minimal_twos_complement(raw: &[u8]) -> Vec<u8> {
    let mut start = 0;
    while start + 1 < raw.len() {
        let curr = raw[start];
        let next = raw[start + 1];
        if (curr == 0x00 && (next & 0x80) == 0) || (curr == 0xFF && (next & 0x80) != 0) {
            start += 1;
        } else {
            break;
        }
    }
    raw[start..].to_vec()
}

fn negate_magnitude_twos_complement(mag: &[u8]) -> Vec<u8> {
    let mut buf = Vec::with_capacity(mag.len() + 1);
    buf.push(0x00);
    buf.extend_from_slice(mag);
    let mut carry = 1u16;
    for byte in buf.iter_mut().rev() {
        let sum = u16::from(!*byte) + carry;
        *byte = (sum & 0xFF) as u8;
        carry = sum >> 8;
    }
    trim_minimal_twos_complement(&buf)
}

/// Admitted Preserves data model value for the bounded Valence profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreservesValue {
    Boolean(bool),
    SignedInteger(SignedInt),
    String(String),
    ByteString(Vec<u8>),
    Symbol(String),
    Record { label: Box<Self>, fields: Vec<Self> },
    Sequence(Vec<Self>),
    Set(Vec<Self>),
    Dictionary(Vec<(Self, Self)>),
}

/// Encode a [`PreservesValue`] into upstream Preserves 0.996.3 canonical binary
/// bytes, enforcing all configured [`PreservesLimits`] and canonical
/// encoded-byte ordering for sets and dictionaries.
pub fn encode_canonical_binary(
    value: &PreservesValue,
    limits: PreservesLimits,
) -> Result<Vec<u8>, PreservesCodecError> {
    limits.validate()?;
    let mut state = EncodeState { limits, value_count: 0 };
    let bytes = state.encode_value(value, 1)?;
    if bytes.len() > limits.max_output_bytes {
        return Err(PreservesCodecError::OutputLimitExceeded);
    }
    Ok(bytes)
}

/// Decode a single upstream Preserves 0.996.3 canonical binary value.
///
/// Rejects trailing bytes, non-minimal varints, non-minimal integers,
/// duplicate or out-of-order set members/dictionary keys, annotations,
/// embedded values, floats, unknown tags, invalid UTF-8, and limit violations.
pub fn decode_canonical_binary(input: &[u8], limits: PreservesLimits) -> Result<PreservesValue, PreservesCodecError> {
    limits.validate()?;
    if input.len() > limits.max_input_bytes {
        return Err(PreservesCodecError::InputLimitExceeded);
    }
    if input.is_empty() {
        return Err(PreservesCodecError::UnexpectedEof);
    }
    let mut decoder = BinaryDecoder {
        input,
        cursor: 0,
        limits,
        value_count: 0,
    };
    let value = decoder.decode_value(1)?;
    if decoder.cursor != decoder.input.len() {
        return Err(PreservesCodecError::TrailingBytes);
    }
    Ok(value)
}

/// Parse a single Preserves text value within the admitted bounded profile.
///
/// Rejects annotations (`@`, `;`), embedded values (`#:`), float literals,
/// duplicate set members/dictionary keys, trailing tokens, and limit breaches.
pub fn parse_preserves_text(input: &str, limits: PreservesLimits) -> Result<PreservesValue, PreservesCodecError> {
    limits.validate()?;
    if input.len() > limits.max_input_bytes {
        return Err(PreservesCodecError::InputLimitExceeded);
    }
    let mut parser = TextParser {
        input,
        bytes: input.as_bytes(),
        cursor: 0,
        limits,
        value_count: 0,
    };
    parser.skip_ws()?;
    let value = parser.parse_value(1)?;
    parser.skip_ws()?;
    if parser.cursor != parser.bytes.len() {
        return Err(PreservesCodecError::TrailingInput);
    }
    Ok(value)
}

/// Parse Preserves text and encode directly into upstream Preserves 0.996.3
/// canonical binary bytes.
pub fn text_to_canonical_binary(input: &str, limits: PreservesLimits) -> Result<Vec<u8>, PreservesCodecError> {
    let value = parse_preserves_text(input, limits)?;
    encode_canonical_binary(&value, limits)
}

struct EncodeState {
    limits: PreservesLimits,
    value_count: usize,
}

impl EncodeState {
    fn count_value(&mut self) -> Result<(), PreservesCodecError> {
        self.value_count += 1;
        if self.value_count > self.limits.max_values {
            return Err(PreservesCodecError::ValueLimitExceeded);
        }
        Ok(())
    }

    fn check_depth(&self, depth: usize) -> Result<(), PreservesCodecError> {
        if depth > self.limits.max_depth {
            return Err(PreservesCodecError::DepthLimitExceeded);
        }
        Ok(())
    }

    fn push_checked(&self, out: &mut Vec<u8>, bytes: &[u8]) -> Result<(), PreservesCodecError> {
        if out.len().saturating_add(bytes.len()) > self.limits.max_output_bytes {
            return Err(PreservesCodecError::OutputLimitExceeded);
        }
        out.extend_from_slice(bytes);
        Ok(())
    }

    fn encode_value(&mut self, value: &PreservesValue, depth: usize) -> Result<Vec<u8>, PreservesCodecError> {
        self.check_depth(depth)?;
        self.count_value()?;
        let mut out = Vec::new();
        self.encode_into(value, depth, &mut out)?;
        Ok(out)
    }

    fn encode_into(
        &mut self,
        value: &PreservesValue,
        depth: usize,
        out: &mut Vec<u8>,
    ) -> Result<(), PreservesCodecError> {
        match value {
            PreservesValue::Boolean(false) => self.push_checked(out, &[TAG_FALSE]),
            PreservesValue::Boolean(true) => self.push_checked(out, &[TAG_TRUE]),
            PreservesValue::SignedInteger(integer) => self.encode_atom(TAG_SIGNED_INTEGER, integer.as_be_bytes(), out),
            PreservesValue::String(text) => self.encode_atom(TAG_STRING, text.as_bytes(), out),
            PreservesValue::ByteString(bytes) => self.encode_atom(TAG_BYTE_STRING, bytes, out),
            PreservesValue::Symbol(symbol) => self.encode_atom(TAG_SYMBOL, symbol.as_bytes(), out),
            PreservesValue::Record { label, fields } => self.encode_record(label, fields, depth, out),
            PreservesValue::Sequence(items) => self.encode_sequence(items, depth, out),
            PreservesValue::Set(items) => self.encode_set(items, depth, out),
            PreservesValue::Dictionary(entries) => self.encode_dictionary(entries, depth, out),
        }
    }

    fn encode_atom(&self, tag: u8, payload: &[u8], out: &mut Vec<u8>) -> Result<(), PreservesCodecError> {
        if payload.len() > self.limits.max_atom_bytes {
            return Err(PreservesCodecError::AtomLimitExceeded);
        }
        let mut header = Vec::with_capacity(11);
        header.push(tag);
        encode_varint(payload.len(), &mut header);
        self.push_checked(out, &header)?;
        self.push_checked(out, payload)
    }

    fn encode_record(
        &mut self,
        label: &PreservesValue,
        fields: &[PreservesValue],
        depth: usize,
        out: &mut Vec<u8>,
    ) -> Result<(), PreservesCodecError> {
        if fields.len() > self.limits.max_collection_items {
            return Err(PreservesCodecError::CollectionLimitExceeded);
        }
        self.push_checked(out, &[TAG_RECORD])?;
        let label_bytes = self.encode_value(label, depth + 1)?;
        self.push_checked(out, &label_bytes)?;
        for field in fields {
            let field_bytes = self.encode_value(field, depth + 1)?;
            self.push_checked(out, &field_bytes)?;
        }
        self.push_checked(out, &[TAG_END])
    }

    fn encode_sequence(
        &mut self,
        items: &[PreservesValue],
        depth: usize,
        out: &mut Vec<u8>,
    ) -> Result<(), PreservesCodecError> {
        if items.len() > self.limits.max_collection_items {
            return Err(PreservesCodecError::CollectionLimitExceeded);
        }
        self.push_checked(out, &[TAG_SEQUENCE])?;
        for item in items {
            let item_bytes = self.encode_value(item, depth + 1)?;
            self.push_checked(out, &item_bytes)?;
        }
        self.push_checked(out, &[TAG_END])
    }

    fn encode_set(
        &mut self,
        items: &[PreservesValue],
        depth: usize,
        out: &mut Vec<u8>,
    ) -> Result<(), PreservesCodecError> {
        if items.len() > self.limits.max_collection_items {
            return Err(PreservesCodecError::CollectionLimitExceeded);
        }
        let mut encoded = Vec::with_capacity(items.len());
        for item in items {
            encoded.push(self.encode_value(item, depth + 1)?);
        }
        encoded.sort();
        for pair in encoded.windows(2) {
            if pair[0] == pair[1] {
                return Err(PreservesCodecError::DuplicateSetMember);
            }
        }
        self.push_checked(out, &[TAG_SET])?;
        for item_bytes in encoded {
            self.push_checked(out, &item_bytes)?;
        }
        self.push_checked(out, &[TAG_END])
    }

    fn encode_dictionary(
        &mut self,
        entries: &[(PreservesValue, PreservesValue)],
        depth: usize,
        out: &mut Vec<u8>,
    ) -> Result<(), PreservesCodecError> {
        if entries.len() > self.limits.max_collection_items {
            return Err(PreservesCodecError::CollectionLimitExceeded);
        }
        let mut encoded = Vec::with_capacity(entries.len());
        for (key, value) in entries {
            let key_bytes = self.encode_value(key, depth + 1)?;
            let value_bytes = self.encode_value(value, depth + 1)?;
            encoded.push((key_bytes, value_bytes));
        }
        encoded.sort_by(|left, right| left.0.cmp(&right.0));
        for pair in encoded.windows(2) {
            if pair[0].0 == pair[1].0 {
                return Err(PreservesCodecError::DuplicateDictionaryKey);
            }
        }
        self.push_checked(out, &[TAG_DICTIONARY])?;
        for (key_bytes, value_bytes) in encoded {
            self.push_checked(out, &key_bytes)?;
            self.push_checked(out, &value_bytes)?;
        }
        self.push_checked(out, &[TAG_END])
    }
}

fn encode_varint(mut value: usize, output: &mut Vec<u8>) {
    loop {
        let low = (value & 0x7F) as u8;
        value >>= 7;
        if value == 0 {
            output.push(low);
            return;
        }
        output.push(low | 0x80);
    }
}

struct BinaryDecoder<'a> {
    input: &'a [u8],
    cursor: usize,
    limits: PreservesLimits,
    value_count: usize,
}

impl<'a> BinaryDecoder<'a> {
    fn peek(&self) -> Option<u8> {
        self.input.get(self.cursor).copied()
    }

    fn take(&mut self) -> Result<u8, PreservesCodecError> {
        let byte = self.peek().ok_or(PreservesCodecError::UnexpectedEof)?;
        self.cursor += 1;
        Ok(byte)
    }

    fn take_slice(&mut self, len: usize) -> Result<&'a [u8], PreservesCodecError> {
        if len > self.limits.max_atom_bytes {
            return Err(PreservesCodecError::AtomLimitExceeded);
        }
        let end = self.cursor.checked_add(len).ok_or(PreservesCodecError::UnexpectedEof)?;
        if end > self.input.len() {
            return Err(PreservesCodecError::UnexpectedEof);
        }
        let slice = &self.input[self.cursor..end];
        self.cursor = end;
        Ok(slice)
    }

    fn read_varint(&mut self) -> Result<usize, PreservesCodecError> {
        let mut result: usize = 0;
        let mut shift: u32 = 0;
        let mut byte_count: usize = 0;
        loop {
            let byte = self.take()?;
            byte_count += 1;
            let payload = usize::from(byte & 0x7F);
            if shift >= usize::BITS {
                return Err(PreservesCodecError::MalformedSyntax("varint_overflow"));
            }
            let shifted = payload.checked_shl(shift).ok_or(PreservesCodecError::MalformedSyntax("varint_overflow"))?;
            if (shifted >> shift) != payload {
                return Err(PreservesCodecError::MalformedSyntax("varint_overflow"));
            }
            result |= shifted;
            if (byte & 0x80) == 0 {
                if byte_count > 1 && byte == 0x00 {
                    return Err(PreservesCodecError::NonMinimalVarint);
                }
                return Ok(result);
            }
            shift += 7;
        }
    }

    fn count_value(&mut self) -> Result<(), PreservesCodecError> {
        self.value_count += 1;
        if self.value_count > self.limits.max_values {
            return Err(PreservesCodecError::ValueLimitExceeded);
        }
        Ok(())
    }

    fn decode_value(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        if depth > self.limits.max_depth {
            return Err(PreservesCodecError::DepthLimitExceeded);
        }
        self.count_value()?;
        let tag = self.take()?;
        match tag {
            TAG_FALSE => Ok(PreservesValue::Boolean(false)),
            TAG_TRUE => Ok(PreservesValue::Boolean(true)),
            TAG_END => Err(PreservesCodecError::MalformedSyntax("unexpected_end_tag")),
            TAG_ANNOTATION => Err(PreservesCodecError::UnsupportedAnnotation),
            TAG_EMBEDDED => Err(PreservesCodecError::UnsupportedEmbeddedValue),
            TAG_FLOAT => Err(PreservesCodecError::UnsupportedFloat),
            TAG_SIGNED_INTEGER => {
                let len = self.read_varint()?;
                let bytes = self.take_slice(len)?;
                Ok(PreservesValue::SignedInteger(SignedInt::from_minimal_be_bytes(bytes.to_vec())?))
            }
            TAG_STRING => {
                let len = self.read_varint()?;
                let bytes = self.take_slice(len)?;
                let text = std::str::from_utf8(bytes).map_err(|_| PreservesCodecError::InvalidUtf8)?;
                Ok(PreservesValue::String(text.to_owned()))
            }
            TAG_BYTE_STRING => {
                let len = self.read_varint()?;
                let bytes = self.take_slice(len)?;
                Ok(PreservesValue::ByteString(bytes.to_vec()))
            }
            TAG_SYMBOL => {
                let len = self.read_varint()?;
                let bytes = self.take_slice(len)?;
                let symbol = std::str::from_utf8(bytes).map_err(|_| PreservesCodecError::InvalidUtf8)?;
                Ok(PreservesValue::Symbol(symbol.to_owned()))
            }
            TAG_RECORD => self.decode_record(depth),
            TAG_SEQUENCE => self.decode_sequence(depth),
            TAG_SET => self.decode_set(depth),
            TAG_DICTIONARY => self.decode_dictionary(depth),
            other => Err(PreservesCodecError::UnsupportedTag(other)),
        }
    }

    fn decode_record(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        if self.peek() == Some(TAG_END) {
            return Err(PreservesCodecError::EmptyRecord);
        }
        let label = self.decode_value(depth + 1)?;
        let mut fields = Vec::new();
        loop {
            match self.peek() {
                Some(TAG_END) => {
                    self.cursor += 1;
                    return Ok(PreservesValue::Record {
                        label: Box::new(label),
                        fields,
                    });
                }
                Some(_) => {
                    if fields.len() >= self.limits.max_collection_items {
                        return Err(PreservesCodecError::CollectionLimitExceeded);
                    }
                    fields.push(self.decode_value(depth + 1)?);
                }
                None => return Err(PreservesCodecError::UnexpectedEof),
            }
        }
    }

    fn decode_sequence(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        let mut items = Vec::new();
        loop {
            match self.peek() {
                Some(TAG_END) => {
                    self.cursor += 1;
                    return Ok(PreservesValue::Sequence(items));
                }
                Some(_) => {
                    if items.len() >= self.limits.max_collection_items {
                        return Err(PreservesCodecError::CollectionLimitExceeded);
                    }
                    items.push(self.decode_value(depth + 1)?);
                }
                None => return Err(PreservesCodecError::UnexpectedEof),
            }
        }
    }

    fn decode_set(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        let mut items = Vec::new();
        let mut previous_encoded: Option<&'a [u8]> = None;
        loop {
            match self.peek() {
                Some(TAG_END) => {
                    self.cursor += 1;
                    return Ok(PreservesValue::Set(items));
                }
                Some(_) => {
                    if items.len() >= self.limits.max_collection_items {
                        return Err(PreservesCodecError::CollectionLimitExceeded);
                    }
                    let start = self.cursor;
                    let item = self.decode_value(depth + 1)?;
                    let encoded = &self.input[start..self.cursor];
                    if let Some(prev) = previous_encoded {
                        match encoded.cmp(prev) {
                            std::cmp::Ordering::Equal => return Err(PreservesCodecError::DuplicateSetMember),
                            std::cmp::Ordering::Less => return Err(PreservesCodecError::NonCanonicalSetOrder),
                            std::cmp::Ordering::Greater => {}
                        }
                    }
                    previous_encoded = Some(encoded);
                    items.push(item);
                }
                None => return Err(PreservesCodecError::UnexpectedEof),
            }
        }
    }

    fn decode_dictionary(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        let mut entries = Vec::new();
        let mut previous_key: Option<&'a [u8]> = None;
        loop {
            match self.peek() {
                Some(TAG_END) => {
                    self.cursor += 1;
                    return Ok(PreservesValue::Dictionary(entries));
                }
                Some(_) => {
                    if entries.len() >= self.limits.max_collection_items {
                        return Err(PreservesCodecError::CollectionLimitExceeded);
                    }
                    let key_start = self.cursor;
                    let key = self.decode_value(depth + 1)?;
                    let key_encoded = &self.input[key_start..self.cursor];
                    if let Some(prev) = previous_key {
                        match key_encoded.cmp(prev) {
                            std::cmp::Ordering::Equal => return Err(PreservesCodecError::DuplicateDictionaryKey),
                            std::cmp::Ordering::Less => return Err(PreservesCodecError::NonCanonicalDictionaryOrder),
                            std::cmp::Ordering::Greater => {}
                        }
                    }
                    previous_key = Some(key_encoded);
                    if self.peek() == Some(TAG_END) {
                        return Err(PreservesCodecError::OddDictionaryEntries);
                    }
                    let value = self.decode_value(depth + 1)?;
                    entries.push((key, value));
                }
                None => return Err(PreservesCodecError::UnexpectedEof),
            }
        }
    }
}

struct TextParser<'a> {
    input: &'a str,
    bytes: &'a [u8],
    cursor: usize,
    limits: PreservesLimits,
    value_count: usize,
}

impl<'a> TextParser<'a> {
    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.cursor).copied()
    }

    fn skip_ws(&mut self) -> Result<(), PreservesCodecError> {
        while let Some(byte) = self.peek() {
            match byte {
                b' ' | b'\t' | b'\r' | b'\n' | b',' => self.cursor += 1,
                b';' => return Err(PreservesCodecError::UnsupportedAnnotation),
                _ => break,
            }
        }
        Ok(())
    }

    fn count_value(&mut self) -> Result<(), PreservesCodecError> {
        self.value_count += 1;
        if self.value_count > self.limits.max_values {
            return Err(PreservesCodecError::ValueLimitExceeded);
        }
        Ok(())
    }

    fn parse_value(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        if depth > self.limits.max_depth {
            return Err(PreservesCodecError::DepthLimitExceeded);
        }
        self.skip_ws()?;
        self.count_value()?;
        match self.peek() {
            Some(b'@') | Some(b';') => Err(PreservesCodecError::UnsupportedAnnotation),
            Some(b'#') => self.parse_hash_prefixed(depth),
            Some(b'"') => self.parse_quoted_text(b'"').map(PreservesValue::String),
            Some(b'|') => self.parse_quoted_text(b'|').map(PreservesValue::Symbol),
            Some(b'<') => self.parse_record(depth),
            Some(b'[') => self.parse_sequence(depth),
            Some(b'{') => self.parse_dictionary(depth),
            Some(_) => self.parse_number_or_symbol(),
            None => Err(PreservesCodecError::UnexpectedEof),
        }
    }

    fn parse_hash_prefixed(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        self.cursor += 1;
        match self.peek() {
            Some(b't') => {
                self.cursor += 1;
                self.require_token_boundary()?;
                Ok(PreservesValue::Boolean(true))
            }
            Some(b'f') => {
                self.cursor += 1;
                self.require_token_boundary()?;
                Ok(PreservesValue::Boolean(false))
            }
            Some(b'{') => self.parse_set(depth),
            Some(b'"') => self.parse_quoted_byte_string().map(PreservesValue::ByteString),
            Some(b'[') => self.parse_base64_byte_string().map(PreservesValue::ByteString),
            Some(b':') => Err(PreservesCodecError::UnsupportedEmbeddedValue),
            Some(b'x') => {
                self.cursor += 1;
                match self.peek() {
                    Some(b'"') => self.parse_hex_byte_string().map(PreservesValue::ByteString),
                    Some(b'd') => Err(PreservesCodecError::UnsupportedFloat),
                    _ => Err(PreservesCodecError::MalformedSyntax("invalid_hash_x_literal")),
                }
            }
            Some(_) => Err(PreservesCodecError::MalformedSyntax("unknown_hash_literal")),
            None => Err(PreservesCodecError::UnexpectedEof),
        }
    }

    fn require_token_boundary(&self) -> Result<(), PreservesCodecError> {
        match self.peek() {
            None => Ok(()),
            Some(byte) if is_delimiter(byte) => Ok(()),
            Some(_) => Err(PreservesCodecError::MalformedSyntax("invalid_boolean_suffix")),
        }
    }

    fn parse_record(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        self.cursor += 1;
        self.skip_ws()?;
        if self.peek() == Some(b'>') {
            return Err(PreservesCodecError::EmptyRecord);
        }
        let label = self.parse_value(depth + 1)?;
        let mut fields = Vec::new();
        loop {
            self.skip_ws()?;
            match self.peek() {
                Some(b'>') => {
                    self.cursor += 1;
                    return Ok(PreservesValue::Record {
                        label: Box::new(label),
                        fields,
                    });
                }
                Some(_) => {
                    if fields.len() >= self.limits.max_collection_items {
                        return Err(PreservesCodecError::CollectionLimitExceeded);
                    }
                    fields.push(self.parse_value(depth + 1)?);
                }
                None => return Err(PreservesCodecError::UnexpectedEof),
            }
        }
    }

    fn parse_sequence(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        self.cursor += 1;
        let mut items = Vec::new();
        loop {
            self.skip_ws()?;
            match self.peek() {
                Some(b']') => {
                    self.cursor += 1;
                    return Ok(PreservesValue::Sequence(items));
                }
                Some(_) => {
                    if items.len() >= self.limits.max_collection_items {
                        return Err(PreservesCodecError::CollectionLimitExceeded);
                    }
                    items.push(self.parse_value(depth + 1)?);
                }
                None => return Err(PreservesCodecError::UnexpectedEof),
            }
        }
    }

    fn parse_set(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        self.cursor += 1;
        let mut items = Vec::new();
        loop {
            self.skip_ws()?;
            match self.peek() {
                Some(b'}') => {
                    self.cursor += 1;
                    return Ok(PreservesValue::Set(items));
                }
                Some(_) => {
                    if items.len() >= self.limits.max_collection_items {
                        return Err(PreservesCodecError::CollectionLimitExceeded);
                    }
                    items.push(self.parse_value(depth + 1)?);
                }
                None => return Err(PreservesCodecError::UnexpectedEof),
            }
        }
    }

    fn parse_dictionary(&mut self, depth: usize) -> Result<PreservesValue, PreservesCodecError> {
        self.cursor += 1;
        let mut entries = Vec::new();
        loop {
            self.skip_ws()?;
            match self.peek() {
                Some(b'}') => {
                    self.cursor += 1;
                    return Ok(PreservesValue::Dictionary(entries));
                }
                Some(_) => {
                    if entries.len() >= self.limits.max_collection_items {
                        return Err(PreservesCodecError::CollectionLimitExceeded);
                    }
                    let key = self.parse_value(depth + 1)?;
                    self.skip_ws()?;
                    if self.peek() != Some(b':') {
                        return Err(PreservesCodecError::MalformedSyntax("expected_colon_in_dictionary"));
                    }
                    self.cursor += 1;
                    let value = self.parse_value(depth + 1)?;
                    entries.push((key, value));
                }
                None => return Err(PreservesCodecError::UnexpectedEof),
            }
        }
    }

    fn parse_quoted_text(&mut self, quote: u8) -> Result<String, PreservesCodecError> {
        self.cursor += 1;
        let mut out = String::new();
        while self.cursor < self.bytes.len() {
            let byte = self.bytes[self.cursor];
            if byte == quote {
                self.cursor += 1;
                return Ok(out);
            }
            if byte == b'\\' {
                self.cursor += 1;
                let escaped = self.parse_text_escape(quote)?;
                out.push(escaped);
            } else if byte < 0x20 {
                return Err(PreservesCodecError::MalformedSyntax("control_char_in_text"));
            } else {
                let ch = self.input[self.cursor..].chars().next().ok_or(PreservesCodecError::UnexpectedEof)?;
                self.cursor += ch.len_utf8();
                out.push(ch);
            }
            if out.len() > self.limits.max_atom_bytes {
                return Err(PreservesCodecError::AtomLimitExceeded);
            }
        }
        Err(PreservesCodecError::UnexpectedEof)
    }

    fn parse_text_escape(&mut self, quote: u8) -> Result<char, PreservesCodecError> {
        let esc = self.peek().ok_or(PreservesCodecError::UnexpectedEof)?;
        self.cursor += 1;
        match esc {
            b'"' if quote == b'"' => Ok('"'),
            b'|' if quote == b'|' => Ok('|'),
            b'\\' => Ok('\\'),
            b'/' => Ok('/'),
            b'b' => Ok('\u{0008}'),
            b'f' => Ok('\u{000C}'),
            b'n' => Ok('\n'),
            b'r' => Ok('\r'),
            b't' => Ok('\t'),
            b'u' => self.parse_unicode_escape(),
            _ => Err(PreservesCodecError::MalformedSyntax("unsupported_escape")),
        }
    }

    fn parse_unicode_escape(&mut self) -> Result<char, PreservesCodecError> {
        let first = self.read_hex_u16()?;
        if (0xD800..=0xDBFF).contains(&first) {
            if self.bytes.get(self.cursor..self.cursor + 2) != Some(b"\\u") {
                return Err(PreservesCodecError::MalformedSyntax("unpaired_high_surrogate"));
            }
            self.cursor += 2;
            let second = self.read_hex_u16()?;
            if !(0xDC00..=0xDFFF).contains(&second) {
                return Err(PreservesCodecError::MalformedSyntax("invalid_low_surrogate"));
            }
            let code = 0x10000 + ((u32::from(first - 0xD800) << 10) | u32::from(second - 0xDC00));
            char::from_u32(code).ok_or(PreservesCodecError::MalformedSyntax("invalid_unicode_scalar"))
        } else if (0xDC00..=0xDFFF).contains(&first) {
            Err(PreservesCodecError::MalformedSyntax("unpaired_low_surrogate"))
        } else {
            char::from_u32(u32::from(first)).ok_or(PreservesCodecError::MalformedSyntax("invalid_unicode_scalar"))
        }
    }

    fn read_hex_u16(&mut self) -> Result<u16, PreservesCodecError> {
        if self.cursor + 4 > self.bytes.len() {
            return Err(PreservesCodecError::UnexpectedEof);
        }
        let hex_slice = &self.input[self.cursor..self.cursor + 4];
        self.cursor += 4;
        u16::from_str_radix(hex_slice, 16).map_err(|_| PreservesCodecError::MalformedSyntax("invalid_hex_escape"))
    }

    fn parse_quoted_byte_string(&mut self) -> Result<Vec<u8>, PreservesCodecError> {
        self.cursor += 1;
        let mut out = Vec::new();
        while self.cursor < self.bytes.len() {
            let byte = self.bytes[self.cursor];
            self.cursor += 1;
            match byte {
                b'"' => return Ok(out),
                b'\\' => {
                    let esc = self.peek().ok_or(PreservesCodecError::UnexpectedEof)?;
                    self.cursor += 1;
                    let val = match esc {
                        b'"' => b'"',
                        b'\\' => b'\\',
                        b'b' => 0x08,
                        b'f' => 0x0C,
                        b'n' => b'\n',
                        b'r' => b'\r',
                        b't' => b'\t',
                        b'x' => self.read_hex_u8()?,
                        _ => return Err(PreservesCodecError::MalformedSyntax("unsupported_byte_escape")),
                    };
                    out.push(val);
                }
                0x20..=0x7E => out.push(byte),
                _ => return Err(PreservesCodecError::MalformedSyntax("non_ascii_in_byte_string")),
            }
            if out.len() > self.limits.max_atom_bytes {
                return Err(PreservesCodecError::AtomLimitExceeded);
            }
        }
        Err(PreservesCodecError::UnexpectedEof)
    }

    fn read_hex_u8(&mut self) -> Result<u8, PreservesCodecError> {
        if self.cursor + 2 > self.bytes.len() {
            return Err(PreservesCodecError::UnexpectedEof);
        }
        let hex_slice = &self.input[self.cursor..self.cursor + 2];
        self.cursor += 2;
        u8::from_str_radix(hex_slice, 16).map_err(|_| PreservesCodecError::MalformedSyntax("invalid_hex_byte"))
    }

    fn parse_hex_byte_string(&mut self) -> Result<Vec<u8>, PreservesCodecError> {
        self.cursor += 1;
        let mut nibbles = Vec::new();
        while self.cursor < self.bytes.len() {
            let byte = self.bytes[self.cursor];
            self.cursor += 1;
            match byte {
                b'"' => {
                    if !nibbles.len().is_multiple_of(2) {
                        return Err(PreservesCodecError::MalformedSyntax("odd_hex_byte_string"));
                    }
                    let mut out = Vec::with_capacity(nibbles.len() / 2);
                    for pair in nibbles.chunks_exact(2) {
                        out.push((pair[0] << 4) | pair[1]);
                    }
                    return Ok(out);
                }
                b' ' | b'\t' | b'\r' | b'\n' => {}
                _ => {
                    let nibble = hex_nibble(byte).ok_or(PreservesCodecError::MalformedSyntax("invalid_hex_digit"))?;
                    nibbles.push(nibble);
                    if nibbles.len() / 2 > self.limits.max_atom_bytes {
                        return Err(PreservesCodecError::AtomLimitExceeded);
                    }
                }
            }
        }
        Err(PreservesCodecError::UnexpectedEof)
    }

    fn parse_base64_byte_string(&mut self) -> Result<Vec<u8>, PreservesCodecError> {
        self.cursor += 1;
        let mut chars = Vec::new();
        while self.cursor < self.bytes.len() {
            let byte = self.bytes[self.cursor];
            self.cursor += 1;
            match byte {
                b']' => {
                    let decoded = decode_base64(&chars)?;
                    if decoded.len() > self.limits.max_atom_bytes {
                        return Err(PreservesCodecError::AtomLimitExceeded);
                    }
                    return Ok(decoded);
                }
                b' ' | b'\t' | b'\r' | b'\n' => {}
                _ => chars.push(byte),
            }
        }
        Err(PreservesCodecError::UnexpectedEof)
    }

    fn parse_number_or_symbol(&mut self) -> Result<PreservesValue, PreservesCodecError> {
        let start = self.cursor;
        while let Some(byte) = self.peek() {
            if is_delimiter(byte) {
                break;
            }
            if byte < 0x20 {
                return Err(PreservesCodecError::MalformedSyntax("control_char_in_symbol"));
            }
            self.cursor += 1;
        }
        if self.cursor == start {
            return Err(PreservesCodecError::MalformedSyntax("unexpected_delimiter"));
        }
        let token = &self.input[start..self.cursor];
        if looks_numeric(token) {
            if token.contains('.') || token.contains('e') || token.contains('E') {
                return Err(PreservesCodecError::UnsupportedFloat);
            }
            let integer = SignedInt::from_decimal_str(token)?;
            if integer.as_be_bytes().len() > self.limits.max_atom_bytes {
                return Err(PreservesCodecError::AtomLimitExceeded);
            }
            Ok(PreservesValue::SignedInteger(integer))
        } else {
            if token.len() > self.limits.max_atom_bytes {
                return Err(PreservesCodecError::AtomLimitExceeded);
            }
            Ok(PreservesValue::Symbol(token.to_owned()))
        }
    }
}

const fn is_delimiter(byte: u8) -> bool {
    matches!(
        byte,
        b' ' | b'\t'
            | b'\r'
            | b'\n'
            | b','
            | b'<'
            | b'>'
            | b'['
            | b']'
            | b'{'
            | b'}'
            | b'('
            | b')'
            | b'"'
            | b'|'
            | b'\''
            | b':'
            | b';'
            | b'#'
            | b'@'
    )
}

fn looks_numeric(token: &str) -> bool {
    let bytes = token.as_bytes();
    match bytes {
        [first, ..] if first.is_ascii_digit() => true,
        [b'+' | b'-', second, ..] if second.is_ascii_digit() => true,
        _ => false,
    }
}

const fn hex_nibble(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}

const fn base64_value(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' | b'-' => Some(62),
        b'/' | b'_' => Some(63),
        _ => None,
    }
}

fn decode_base64(input: &[u8]) -> Result<Vec<u8>, PreservesCodecError> {
    let trimmed = match input.iter().position(|&b| b == b'=') {
        Some(pad_pos) => {
            if !input[pad_pos..].iter().all(|&b| b == b'=') {
                return Err(PreservesCodecError::MalformedSyntax("invalid_base64_padding"));
            }
            if !input.len().is_multiple_of(4) || input.len() - pad_pos > 2 {
                return Err(PreservesCodecError::MalformedSyntax("invalid_base64_padding"));
            }
            &input[..pad_pos]
        }
        None => input,
    };
    if trimmed.len() % 4 == 1 {
        return Err(PreservesCodecError::MalformedSyntax("invalid_base64_length"));
    }
    let mut out = Vec::with_capacity((trimmed.len() * 3) / 4);
    let mut chunks = trimmed.chunks_exact(4);
    for chunk in &mut chunks {
        let a = base64_value(chunk[0]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
        let b = base64_value(chunk[1]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
        let c = base64_value(chunk[2]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
        let d = base64_value(chunk[3]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
        out.push((a << 2) | (b >> 4));
        out.push((b << 4) | (c >> 2));
        out.push((c << 6) | d);
    }
    let rem = chunks.remainder();
    match rem.len() {
        0 => {}
        2 => {
            let a = base64_value(rem[0]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
            let b = base64_value(rem[1]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
            if (b & 0x0F) != 0 {
                return Err(PreservesCodecError::MalformedSyntax("noncanonical_base64_tail"));
            }
            out.push((a << 2) | (b >> 4));
        }
        3 => {
            let a = base64_value(rem[0]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
            let b = base64_value(rem[1]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
            let c = base64_value(rem[2]).ok_or(PreservesCodecError::MalformedSyntax("invalid_base64_char"))?;
            if (c & 0x03) != 0 {
                return Err(PreservesCodecError::MalformedSyntax("noncanonical_base64_tail"));
            }
            out.push((a << 2) | (b >> 4));
            out.push((b << 4) | (c >> 2));
        }
        _ => return Err(PreservesCodecError::MalformedSyntax("invalid_base64_length")),
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_and_decodes_upstream_preserves_reference_vectors() {
        let limits = PreservesLimits::default();
        assert_eq!(text_to_canonical_binary("#f", limits).unwrap(), vec![0x80]);
        assert_eq!(text_to_canonical_binary("#t", limits).unwrap(), vec![0x81]);
        assert_eq!(text_to_canonical_binary("0", limits).unwrap(), vec![0xB0, 0x00]);
        assert_eq!(text_to_canonical_binary("1", limits).unwrap(), vec![0xB0, 0x01, 0x01]);
        assert_eq!(text_to_canonical_binary("127", limits).unwrap(), vec![0xB0, 0x01, 0x7F]);
        assert_eq!(text_to_canonical_binary("128", limits).unwrap(), vec![0xB0, 0x02, 0x00, 0x80]);
        assert_eq!(text_to_canonical_binary("-1", limits).unwrap(), vec![0xB0, 0x01, 0xFF]);
        assert_eq!(text_to_canonical_binary("-128", limits).unwrap(), vec![0xB0, 0x01, 0x80]);
        assert_eq!(text_to_canonical_binary("-129", limits).unwrap(), vec![0xB0, 0x02, 0xFF, 0x7F]);
        // Zero-field record <x>
        assert_eq!(text_to_canonical_binary("<x>", limits).unwrap(), vec![0xB4, 0xB3, 0x01, b'x', 0x84]);
        // Distinct symbol, string, and byte-string encodings for "x"
        assert_eq!(text_to_canonical_binary("x", limits).unwrap(), vec![0xB3, 0x01, b'x']);
        assert_eq!(text_to_canonical_binary("\"x\"", limits).unwrap(), vec![0xB1, 0x01, b'x']);
        assert_eq!(text_to_canonical_binary("#\"x\"", limits).unwrap(), vec![0xB2, 0x01, b'x']);
        assert_eq!(text_to_canonical_binary("#x\"78\"", limits).unwrap(), vec![0xB2, 0x01, b'x']);
        assert_eq!(text_to_canonical_binary("#[eA==]", limits).unwrap(), vec![0xB2, 0x01, b'x']);

        // Kamacite reference record: <BoundRecord "alpha" [1 2]>
        let kamacite_expected = vec![
            0xB4, 0xB3, 0x0B, b'B', b'o', b'u', b'n', b'd', b'R', b'e', b'c', b'o', b'r', b'd', 0xB1, 0x05, b'a', b'l',
            b'p', b'h', b'a', 0xB5, 0xB0, 0x01, 0x01, 0xB0, 0x01, 0x02, 0x84, 0x84,
        ];
        assert_eq!(text_to_canonical_binary("<BoundRecord \"alpha\" [1 2]>", limits).unwrap(), kamacite_expected);
        let decoded = decode_canonical_binary(&kamacite_expected, limits).unwrap();
        assert_eq!(encode_canonical_binary(&decoded, limits).unwrap(), kamacite_expected);
    }

    #[test]
    fn handles_unicode_and_large_integers() {
        let limits = PreservesLimits::default();
        let unicode_bytes = text_to_canonical_binary("\"✓☃🚀\"", limits).unwrap();
        let expected_utf8 = "✓☃🚀".as_bytes();
        let mut expected = vec![0xB1, expected_utf8.len() as u8];
        expected.extend_from_slice(expected_utf8);
        assert_eq!(unicode_bytes, expected);
        assert_eq!(decode_canonical_binary(&unicode_bytes, limits).unwrap(), PreservesValue::String("✓☃🚀".to_owned()));

        // Surrogate pair escape decodes to the same astral scalar
        assert_eq!(
            text_to_canonical_binary("\"\\uD83D\\uDE80\"", limits).unwrap(),
            text_to_canonical_binary("\"🚀\"", limits).unwrap()
        );

        // Large integer exceeding i128: 2^128 = 340282366920938463463374607431768211456
        let big_pos = text_to_canonical_binary("340282366920938463463374607431768211456", limits).unwrap();
        let mut expected_big = vec![0xB0, 17, 0x01];
        expected_big.extend_from_slice(&[0x00; 16]);
        assert_eq!(big_pos, expected_big);
        assert_eq!(
            encode_canonical_binary(&decode_canonical_binary(&big_pos, limits).unwrap(), limits).unwrap(),
            big_pos
        );
    }

    #[test]
    fn canonical_sets_and_dicts_sort_by_encoded_byte_order() {
        let limits = PreservesLimits::default();
        // Encoded-byte order differs from mathematical/lexicographic order:
        // 0 = b0 00 < 1 = b0 01 01 < -1 = b0 01 ff < 128 = b0 02 00 80
        let set_a = text_to_canonical_binary("#{-1 128 0 1}", limits).unwrap();
        let set_b = text_to_canonical_binary("#{0 1 -1 128}", limits).unwrap();
        assert_eq!(set_a, set_b);
        assert_eq!(set_a, vec![
            0xB6, 0xB0, 0x00, 0xB0, 0x01, 0x01, 0xB0, 0x01, 0xFF, 0xB0, 0x02, 0x00, 0x80, 0x84
        ]);
        assert!(decode_canonical_binary(&set_a, limits).is_ok());

        // String length varint makes "b" (b1 01 62) sort BEFORE "aa" (b1 02 61 61)
        let dict_bytes = text_to_canonical_binary("{\"aa\": 1, \"b\": 2}", limits).unwrap();
        assert_eq!(dict_bytes, vec![
            0xB7, 0xB1, 0x01, b'b', 0xB0, 0x01, 0x02, 0xB1, 0x02, b'a', b'a', 0xB0, 0x01, 0x01, 0x84
        ]);
        // Swapped order in binary fails closed
        let noncanonical_dict = vec![
            0xB7, 0xB1, 0x02, b'a', b'a', 0xB0, 0x01, 0x01, 0xB1, 0x01, b'b', 0xB0, 0x01, 0x02, 0x84,
        ];
        assert_eq!(
            decode_canonical_binary(&noncanonical_dict, limits),
            Err(PreservesCodecError::NonCanonicalDictionaryOrder)
        );
    }

    #[test]
    fn rejects_duplicates_nonminimal_annotations_embedded_floats_and_limits() {
        let limits = PreservesLimits::default();
        assert_eq!(text_to_canonical_binary("#{1 1}", limits), Err(PreservesCodecError::DuplicateSetMember));
        assert_eq!(text_to_canonical_binary("{a: 1, a: 2}", limits), Err(PreservesCodecError::DuplicateDictionaryKey));
        assert_eq!(text_to_canonical_binary("@note <x>", limits), Err(PreservesCodecError::UnsupportedAnnotation));
        assert_eq!(text_to_canonical_binary("; comment\n<x>", limits), Err(PreservesCodecError::UnsupportedAnnotation));
        assert_eq!(
            text_to_canonical_binary("#:\"opaque\"", limits),
            Err(PreservesCodecError::UnsupportedEmbeddedValue)
        );
        assert_eq!(text_to_canonical_binary("1.5", limits), Err(PreservesCodecError::UnsupportedFloat));
        // Binary annotations, embedded, float, nonminimal integer, nonminimal varint, trailing bytes
        assert_eq!(
            decode_canonical_binary(&[0x85, 0x80, 0x81], limits),
            Err(PreservesCodecError::UnsupportedAnnotation)
        );
        assert_eq!(decode_canonical_binary(&[0x86, 0x80], limits), Err(PreservesCodecError::UnsupportedEmbeddedValue));
        assert_eq!(
            decode_canonical_binary(&[0x87, 0x08, 0, 0, 0, 0, 0, 0, 0, 0], limits),
            Err(PreservesCodecError::UnsupportedFloat)
        );
        assert_eq!(decode_canonical_binary(&[0xB0, 0x01, 0x00], limits), Err(PreservesCodecError::NonMinimalInteger));
        assert_eq!(
            decode_canonical_binary(&[0xB0, 0x02, 0x00, 0x01], limits),
            Err(PreservesCodecError::NonMinimalInteger)
        );
        assert_eq!(
            decode_canonical_binary(&[0xB1, 0x81, 0x00, b'a'], limits),
            Err(PreservesCodecError::NonMinimalVarint)
        );
        assert_eq!(decode_canonical_binary(&[0x80, 0x00], limits), Err(PreservesCodecError::TrailingBytes));
        // Resource ceilings
        assert_eq!(
            text_to_canonical_binary("<a [1 2]>", PreservesLimits { max_depth: 2, ..limits }),
            Err(PreservesCodecError::DepthLimitExceeded)
        );
        assert_eq!(
            text_to_canonical_binary("[1 2 3]", PreservesLimits {
                max_collection_items: 2,
                ..limits
            }),
            Err(PreservesCodecError::CollectionLimitExceeded)
        );
        assert_eq!(
            text_to_canonical_binary("\"abcd\"", PreservesLimits {
                max_atom_bytes: 3,
                ..limits
            }),
            Err(PreservesCodecError::AtomLimitExceeded)
        );
        assert_eq!(
            text_to_canonical_binary("<x>", PreservesLimits { max_depth: 0, ..limits }),
            Err(PreservesCodecError::InvalidLimit("max_depth"))
        );
    }
}

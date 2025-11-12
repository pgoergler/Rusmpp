use rusmpp_macros::Rusmpp;

use crate::{
    encode::Length,
    tlvs::{owned::TlvValue, tag::TlvTag},
};

mod broadcast_request;
pub use broadcast_request::*;

mod broadcast_response;
pub use broadcast_response::*;

mod cancel_broadcast;
pub use cancel_broadcast::*;

mod message_delivery_request;
pub use message_delivery_request::*;

mod message_delivery_response;
pub use message_delivery_response::*;

mod message_submission_request;
pub use message_submission_request::*;

mod message_submission_response;
pub use message_submission_response::*;

mod query_broadcast_response;
pub use query_broadcast_response::*;

/// See module level documentation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = owned, test = skip)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct Tlv {
    tag: TlvTag,
    value_length: u16,
    #[rusmpp(key = tag, length = value_length)]
    value: Option<TlvValue>,
}

impl Tlv {
    pub fn new(value: impl Into<TlvValue>) -> Self {
        let value = value.into();
        let tag = value.tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }

    pub const fn tag(&self) -> TlvTag {
        self.tag
    }

    pub const fn value_length(&self) -> u16 {
        self.value_length
    }

    pub const fn value(&self) -> Option<&TlvValue> {
        self.value.as_ref()
    }

    /// Create a custom TLV with arbitrary tag and value bytes.
    ///
    /// This method allows creating vendor-specific TLVs with custom tags (0x1400-0x3FFF).
    ///
    /// # Example
    ///
    /// ```ignore
    /// use rusmpp_core::tlvs::owned::Tlv;
    ///
    /// let custom_tlv = Tlv::new_custom(0x1400, vec![1, 2, 3, 4]);
    /// ```
    pub fn new_custom(tag: u16, value: alloc::vec::Vec<u8>) -> Self {
        use crate::types::owned::AnyOctetString;

        let tag = TlvTag::Other(tag);
        let value_length = value.len() as u16;
        let value = TlvValue::Other {
            tag,
            value: AnyOctetString::new(&value),
        };

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }

    /// Create a custom TLV from a u16 value (big-endian).
    pub fn new_custom_u16(tag: u16, value: u16) -> Self {
        Self::new_custom(tag, value.to_be_bytes().to_vec())
    }

    /// Create a custom TLV from a u32 value (big-endian).
    pub fn new_custom_u32(tag: u16, value: u32) -> Self {
        Self::new_custom(tag, value.to_be_bytes().to_vec())
    }

    /// Create a custom TLV from a u64 value (big-endian).
    pub fn new_custom_u64(tag: u16, value: u64) -> Self {
        Self::new_custom(tag, value.to_be_bytes().to_vec())
    }

    /// Create a custom TLV from a string value (null-terminated).
    pub fn new_custom_string(tag: u16, value: &str) -> Self {
        let mut bytes = value.as_bytes().to_vec();
        bytes.push(0); // null terminator
        Self::new_custom(tag, bytes)
    }

    /// Extract raw bytes from a custom TLV.
    pub fn extract_raw_bytes(&self) -> Option<&[u8]> {
        if let Some(TlvValue::Other { value, .. }) = &self.value {
            Some(value.bytes())
        } else {
            None
        }
    }

    /// Extract a u16 value from a custom TLV (big-endian).
    pub fn extract_u16(&self) -> Option<u16> {
        let bytes = self.extract_raw_bytes()?;
        if bytes.len() == 2 {
            Some(u16::from_be_bytes([bytes[0], bytes[1]]))
        } else {
            None
        }
    }

    /// Extract a u32 value from a custom TLV (big-endian).
    pub fn extract_u32(&self) -> Option<u32> {
        let bytes = self.extract_raw_bytes()?;
        if bytes.len() == 4 {
            Some(u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
        } else {
            None
        }
    }

    /// Extract a u64 value from a custom TLV (big-endian).
    pub fn extract_u64(&self) -> Option<u64> {
        let bytes = self.extract_raw_bytes()?;
        if bytes.len() == 8 {
            Some(u64::from_be_bytes([
                bytes[0], bytes[1], bytes[2], bytes[3],
                bytes[4], bytes[5], bytes[6], bytes[7],
            ]))
        } else {
            None
        }
    }

    /// Extract a string value from a custom TLV (null-terminated).
    pub fn extract_string(&self) -> Option<alloc::string::String> {
        let bytes = self.extract_raw_bytes()?;

        // Remove null terminator if present
        let bytes = if bytes.last() == Some(&0) {
            &bytes[..bytes.len() - 1]
        } else {
            bytes
        };

        alloc::string::String::from_utf8(bytes.to_vec()).ok()
    }
}

impl From<TlvValue> for Tlv {
    fn from(value: TlvValue) -> Self {
        Self::new(value)
    }
}

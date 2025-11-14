/// Macro to implement `TlvContainer` trait for PDU structs.
///
/// This macro reduces boilerplate by generating the standard implementation
/// of `TlvContainer` for PDU types that have a `tlvs` field.
///
/// # Variants
///
/// ## Basic implementation (no short_message handling)
/// ```ignore
/// impl_tlv_container!(DataSm);
/// ```
///
/// ## With short_message clearing
/// For PDUs that have a `short_message` field and need to clear it when
/// `MessagePayload` TLV is present:
/// ```ignore
/// impl_tlv_container!(SubmitSm, with_short_message_clear);
/// ```
#[macro_export]
macro_rules! impl_tlv_container {
    // Variant 1: PDU with short_message field (SubmitSm, DeliverSm)
    ($pdu_type:ty, with_short_message_clear) => {
        impl $crate::tlvs::TlvContainer for $pdu_type {
            fn push_tlv_raw(&mut self, tlv: $crate::tlvs::owned::Tlv) {
                self.tlvs.push(tlv);
                self.clear_short_message_if_message_payload_exists();
            }

            fn get_tlv(&self, tag: $crate::tlvs::TlvTag) -> Option<&$crate::tlvs::owned::Tlv> {
                self.tlvs.iter().find(|tlv| tlv.tag() == tag)
            }

            fn get_tlvs(&self) -> &[$crate::tlvs::owned::Tlv] {
                &self.tlvs
            }

            fn get_tlvs_mut(&mut self) -> &mut alloc::vec::Vec<$crate::tlvs::owned::Tlv> {
                &mut self.tlvs
            }

            fn remove_tlv(&mut self, tag: $crate::tlvs::TlvTag) -> Option<$crate::tlvs::owned::Tlv> {
                if let Some(pos) = self.tlvs.iter().position(|tlv| tlv.tag() == tag) {
                    Some(self.tlvs.remove(pos))
                } else {
                    None
                }
            }
        }
    };

    // Variant 2: PDU without short_message field (DataSm)
    ($pdu_type:ty) => {
        impl $crate::tlvs::TlvContainer for $pdu_type {
            fn push_tlv_raw(&mut self, tlv: $crate::tlvs::owned::Tlv) {
                self.tlvs.push(tlv);
            }

            fn get_tlv(&self, tag: $crate::tlvs::TlvTag) -> Option<&$crate::tlvs::owned::Tlv> {
                self.tlvs.iter().find(|tlv| tlv.tag() == tag)
            }

            fn get_tlvs(&self) -> &[$crate::tlvs::owned::Tlv] {
                &self.tlvs
            }

            fn get_tlvs_mut(&mut self) -> &mut alloc::vec::Vec<$crate::tlvs::owned::Tlv> {
                &mut self.tlvs
            }

            fn remove_tlv(&mut self, tag: $crate::tlvs::TlvTag) -> Option<$crate::tlvs::owned::Tlv> {
                if let Some(pos) = self.tlvs.iter().position(|tlv| tlv.tag() == tag) {
                    Some(self.tlvs.remove(pos))
                } else {
                    None
                }
            }
        }
    };
}

/// Trait for PDUs that support TLV manipulation.
///
/// This trait provides a uniform API for adding, removing, and querying TLVs
/// across different PDU types that support optional TLV fields.
///
/// # Example
///
/// ```ignore
/// use rusmpp_core::tlvs::{TlvContainer, owned::Tlv};
/// use rusmpp_core::pdus::owned::SubmitSm;
///
/// let mut submit_sm = SubmitSm::default();
/// let custom_tlv = Tlv::new_custom(0x1400, vec![1, 2, 3, 4]);
/// submit_sm.push_tlv_raw(custom_tlv);
/// ```
pub trait TlvContainer {
    /// Push a raw TLV to the container.
    ///
    /// This method allows adding any TLV, including custom ones with
    /// vendor-specific tags (0x1400-0x3FFF).
    fn push_tlv_raw(&mut self, tlv: crate::tlvs::owned::Tlv);

    /// Get a reference to a TLV by tag.
    fn get_tlv(&self, tag: crate::tlvs::TlvTag) -> Option<&crate::tlvs::owned::Tlv>;

    /// Get all TLVs as a slice.
    fn get_tlvs(&self) -> &[crate::tlvs::owned::Tlv];

    /// Get a mutable reference to the TLVs vector.
    fn get_tlvs_mut(&mut self) -> &mut alloc::vec::Vec<crate::tlvs::owned::Tlv>;

    /// Remove a TLV by tag and return it if found.
    fn remove_tlv(&mut self, tag: crate::tlvs::TlvTag) -> Option<crate::tlvs::owned::Tlv>;

    /// Check if a TLV with the given tag exists.
    fn has_tlv(&self, tag: crate::tlvs::TlvTag) -> bool {
        self.get_tlv(tag).is_some()
    }

    /// Clear all TLVs.
    fn clear_tlvs(&mut self) {
        self.get_tlvs_mut().clear();
    }
}

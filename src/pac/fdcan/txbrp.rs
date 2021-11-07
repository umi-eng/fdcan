///Register `TXBRP` reader
pub struct R(crate::R<TXBRP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXBRP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXBRP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXBRP_SPEC>) -> Self {
        R(reader)
    }
}
///Field `TRP` reader - Transmission Request Pending
pub struct TRP_R(crate::FieldReader<u32, u32>);
impl TRP_R {
    pub(crate) fn new(bits: u32) -> Self {
        TRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TRP_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    ///Bits 0:31 - Transmission Request Pending
    #[inline(always)]
    pub fn trp(&self) -> TRP_R {
        TRP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
///FDCAN Tx Buffer Request Pending Register
///
///This register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).
///
///For information about available fields see [txbrp](index.html) module
pub struct TXBRP_SPEC;
impl crate::RegisterSpec for TXBRP_SPEC {
    type Ux = u32;
}
///`read()` method returns [txbrp::R](R) reader structure
impl crate::Readable for TXBRP_SPEC {
    type Reader = R;
}
///`reset()` method sets TXBRP to value 0
impl crate::Resettable for TXBRP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}

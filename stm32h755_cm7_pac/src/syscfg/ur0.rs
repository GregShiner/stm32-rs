#[doc = "Register `UR0` reader"]
pub type R = crate::R<Ur0Spec>;
#[doc = "Field `BKS` reader - Bank Swap"]
pub type BksR = crate::BitReader;
#[doc = "Field `RDP` reader - Readout protection"]
pub type RdpR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Bank Swap"]
    #[inline(always)]
    pub fn bks(&self) -> BksR {
        BksR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - Readout protection"]
    #[inline(always)]
    pub fn rdp(&self) -> RdpR {
        RdpR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "SYSCFG user register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ur0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur0Spec;
impl crate::RegisterSpec for Ur0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur0::R`](R) reader structure"]
impl crate::Readable for Ur0Spec {}
#[doc = "`reset()` method sets UR0 to value 0"]
impl crate::Resettable for Ur0Spec {}

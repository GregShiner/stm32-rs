#[doc = "Register `CDSR` reader"]
pub type R = crate::R<CdsrSpec>;
#[doc = "Field `VDES` reader - Vertical Data Enable display Status"]
pub type VdesR = crate::BitReader;
#[doc = "Field `HDES` reader - Horizontal Data Enable display Status"]
pub type HdesR = crate::BitReader;
#[doc = "Field `VSYNCS` reader - Vertical Synchronization display Status"]
pub type VsyncsR = crate::BitReader;
#[doc = "Field `HSYNCS` reader - Horizontal Synchronization display Status"]
pub type HsyncsR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Vertical Data Enable display Status"]
    #[inline(always)]
    pub fn vdes(&self) -> VdesR {
        VdesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Horizontal Data Enable display Status"]
    #[inline(always)]
    pub fn hdes(&self) -> HdesR {
        HdesR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Vertical Synchronization display Status"]
    #[inline(always)]
    pub fn vsyncs(&self) -> VsyncsR {
        VsyncsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Horizontal Synchronization display Status"]
    #[inline(always)]
    pub fn hsyncs(&self) -> HsyncsR {
        HsyncsR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Current Display Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cdsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdsrSpec;
impl crate::RegisterSpec for CdsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdsr::R`](R) reader structure"]
impl crate::Readable for CdsrSpec {}
#[doc = "`reset()` method sets CDSR to value 0x0f"]
impl crate::Resettable for CdsrSpec {
    const RESET_VALUE: u32 = 0x0f;
}

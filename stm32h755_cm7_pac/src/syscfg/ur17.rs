#[doc = "Register `UR17` reader"]
pub type R = crate::R<Ur17Spec>;
#[doc = "Field `IO_HSLV` reader - I/O high speed / low voltage"]
pub type IoHslvR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - I/O high speed / low voltage"]
    #[inline(always)]
    pub fn io_hslv(&self) -> IoHslvR {
        IoHslvR::new((self.bits & 1) != 0)
    }
}
#[doc = "SYSCFG user register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`ur17::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur17Spec;
impl crate::RegisterSpec for Ur17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur17::R`](R) reader structure"]
impl crate::Readable for Ur17Spec {}
#[doc = "`reset()` method sets UR17 to value 0"]
impl crate::Resettable for Ur17Spec {}

#[doc = "Register `CRRCR` reader"]
pub type R = crate::R<CrrcrSpec>;
#[doc = "Field `RC48CAL` reader - Internal RC 48 MHz clock calibration"]
pub type Rc48calR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Internal RC 48 MHz clock calibration"]
    #[inline(always)]
    pub fn rc48cal(&self) -> Rc48calR {
        Rc48calR::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "RCC Clock Recovery RC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crrcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrrcrSpec;
impl crate::RegisterSpec for CrrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crrcr::R`](R) reader structure"]
impl crate::Readable for CrrcrSpec {}
#[doc = "`reset()` method sets CRRCR to value 0"]
impl crate::Resettable for CrrcrSpec {}

#[doc = "Register `FDCAN_TTLGT` reader"]
pub type R = crate::R<FdcanTtlgtSpec>;
#[doc = "Field `LT` reader - Local Time"]
pub type LtR = crate::FieldReader<u16>;
#[doc = "Field `GT` reader - Global Time"]
pub type GtR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Local Time"]
    #[inline(always)]
    pub fn lt(&self) -> LtR {
        LtR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Global Time"]
    #[inline(always)]
    pub fn gt(&self) -> GtR {
        GtR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "FDCAN TT Local and Global Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttlgt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtlgtSpec;
impl crate::RegisterSpec for FdcanTtlgtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttlgt::R`](R) reader structure"]
impl crate::Readable for FdcanTtlgtSpec {}
#[doc = "`reset()` method sets FDCAN_TTLGT to value 0"]
impl crate::Resettable for FdcanTtlgtSpec {}

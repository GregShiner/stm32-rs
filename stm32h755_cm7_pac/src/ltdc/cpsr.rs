#[doc = "Register `CPSR` reader"]
pub type R = crate::R<CpsrSpec>;
#[doc = "Field `CYPOS` reader - Current Y Position"]
pub type CyposR = crate::FieldReader<u16>;
#[doc = "Field `CXPOS` reader - Current X Position"]
pub type CxposR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Current Y Position"]
    #[inline(always)]
    pub fn cypos(&self) -> CyposR {
        CyposR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Current X Position"]
    #[inline(always)]
    pub fn cxpos(&self) -> CxposR {
        CxposR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Current Position Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpsrSpec;
impl crate::RegisterSpec for CpsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpsr::R`](R) reader structure"]
impl crate::Readable for CpsrSpec {}
#[doc = "`reset()` method sets CPSR to value 0"]
impl crate::Resettable for CpsrSpec {}

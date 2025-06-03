#[doc = "Register `DR_10` reader"]
pub type R = crate::R<Dr10Spec>;
#[doc = "Field `DRNL1` reader - Data value"]
pub type Drnl1R = crate::FieldReader<u16>;
#[doc = "Field `DRNL2` reader - Data value"]
pub type Drnl2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Data value"]
    #[inline(always)]
    pub fn drnl1(&self) -> Drnl1R {
        Drnl1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Data value"]
    #[inline(always)]
    pub fn drnl2(&self) -> Drnl2R {
        Drnl2R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr_10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dr10Spec;
impl crate::RegisterSpec for Dr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dr_10::R`](R) reader structure"]
impl crate::Readable for Dr10Spec {}
#[doc = "`reset()` method sets DR_10 to value 0"]
impl crate::Resettable for Dr10Spec {}

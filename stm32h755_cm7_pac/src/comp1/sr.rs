#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `C1VAL` reader - COMP channel 1 output status bit"]
pub type C1valR = crate::BitReader;
#[doc = "Field `C2VAL` reader - COMP channel 2 output status bit"]
pub type C2valR = crate::BitReader;
#[doc = "Field `C1IF` reader - COMP channel 1 Interrupt Flag"]
pub type C1ifR = crate::BitReader;
#[doc = "Field `C2IF` reader - COMP channel 2 Interrupt Flag"]
pub type C2ifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - COMP channel 1 output status bit"]
    #[inline(always)]
    pub fn c1val(&self) -> C1valR {
        C1valR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - COMP channel 2 output status bit"]
    #[inline(always)]
    pub fn c2val(&self) -> C2valR {
        C2valR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - COMP channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn c1if(&self) -> C1ifR {
        C1ifR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - COMP channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn c2if(&self) -> C2ifR {
        C2ifR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Comparator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}

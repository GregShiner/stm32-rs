#[doc = "Register `DIR` reader"]
pub type R = crate::R<DirSpec>;
#[doc = "Field `THI` reader - Threshold HIGH"]
pub type ThiR = crate::FieldReader<u16>;
#[doc = "Field `TLO` reader - Threshold LOW"]
pub type TloR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Threshold HIGH"]
    #[inline(always)]
    pub fn thi(&self) -> ThiR {
        ThiR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - Threshold LOW"]
    #[inline(always)]
    pub fn tlo(&self) -> TloR {
        TloR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
#[doc = "Debug Information register\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DirSpec;
impl crate::RegisterSpec for DirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dir::R`](R) reader structure"]
impl crate::Readable for DirSpec {}
#[doc = "`reset()` method sets DIR to value 0"]
impl crate::Resettable for DirSpec {}

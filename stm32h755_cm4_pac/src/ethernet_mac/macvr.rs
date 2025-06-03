#[doc = "Register `MACVR` reader"]
pub type R = crate::R<MacvrSpec>;
#[doc = "Field `SNPSVER` reader - SNPSVER"]
pub type SnpsverR = crate::FieldReader;
#[doc = "Field `USERVER` reader - USERVER"]
pub type UserverR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - SNPSVER"]
    #[inline(always)]
    pub fn snpsver(&self) -> SnpsverR {
        SnpsverR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - USERVER"]
    #[inline(always)]
    pub fn userver(&self) -> UserverR {
        UserverR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "Version register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacvrSpec;
impl crate::RegisterSpec for MacvrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvr::R`](R) reader structure"]
impl crate::Readable for MacvrSpec {}
#[doc = "`reset()` method sets MACVR to value 0x3041"]
impl crate::Resettable for MacvrSpec {
    const RESET_VALUE: u32 = 0x3041;
}

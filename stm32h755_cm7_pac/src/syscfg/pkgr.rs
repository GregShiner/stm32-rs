#[doc = "Register `PKGR` reader"]
pub type R = crate::R<PkgrSpec>;
#[doc = "Field `PKG` reader - Package"]
pub type PkgR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Package"]
    #[inline(always)]
    pub fn pkg(&self) -> PkgR {
        PkgR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "SYSCFG package register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkgr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PkgrSpec;
impl crate::RegisterSpec for PkgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pkgr::R`](R) reader structure"]
impl crate::Readable for PkgrSpec {}
#[doc = "`reset()` method sets PKGR to value 0"]
impl crate::Resettable for PkgrSpec {}

#[doc = "Register `MDIOS_CRDFR` reader"]
pub type R = crate::R<MdiosCrdfrSpec>;
#[doc = "Register `MDIOS_CRDFR` writer"]
pub type W = crate::W<MdiosCrdfrSpec>;
#[doc = "Field `CRDF` reader - Clear the read flag"]
pub type CrdfR = crate::FieldReader<u32>;
#[doc = "Field `CRDF` writer - Clear the read flag"]
pub type CrdfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Clear the read flag"]
    #[inline(always)]
    pub fn crdf(&self) -> CrdfR {
        CrdfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the read flag"]
    #[inline(always)]
    pub fn crdf(&mut self) -> CrdfW<MdiosCrdfrSpec> {
        CrdfW::new(self, 0)
    }
}
#[doc = "MDIOS clear read flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_crdfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_crdfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosCrdfrSpec;
impl crate::RegisterSpec for MdiosCrdfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_crdfr::R`](R) reader structure"]
impl crate::Readable for MdiosCrdfrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdios_crdfr::W`](W) writer structure"]
impl crate::Writable for MdiosCrdfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_CRDFR to value 0"]
impl crate::Resettable for MdiosCrdfrSpec {}

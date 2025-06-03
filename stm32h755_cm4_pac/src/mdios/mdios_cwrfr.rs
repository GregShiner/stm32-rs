#[doc = "Register `MDIOS_CWRFR` reader"]
pub type R = crate::R<MdiosCwrfrSpec>;
#[doc = "Register `MDIOS_CWRFR` writer"]
pub type W = crate::W<MdiosCwrfrSpec>;
#[doc = "Field `CWRF` reader - Clear the write flag"]
pub type CwrfR = crate::FieldReader<u32>;
#[doc = "Field `CWRF` writer - Clear the write flag"]
pub type CwrfW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Clear the write flag"]
    #[inline(always)]
    pub fn cwrf(&self) -> CwrfR {
        CwrfR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Clear the write flag"]
    #[inline(always)]
    pub fn cwrf(&mut self) -> CwrfW<MdiosCwrfrSpec> {
        CwrfW::new(self, 0)
    }
}
#[doc = "MDIOS clear write flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_cwrfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_cwrfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosCwrfrSpec;
impl crate::RegisterSpec for MdiosCwrfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_cwrfr::R`](R) reader structure"]
impl crate::Readable for MdiosCwrfrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdios_cwrfr::W`](W) writer structure"]
impl crate::Writable for MdiosCwrfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_CWRFR to value 0"]
impl crate::Resettable for MdiosCwrfrSpec {}

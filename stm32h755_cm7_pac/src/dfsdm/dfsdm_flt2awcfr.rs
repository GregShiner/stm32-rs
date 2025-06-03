#[doc = "Register `DFSDM_FLT2AWCFR` reader"]
pub type R = crate::R<DfsdmFlt2awcfrSpec>;
#[doc = "Register `DFSDM_FLT2AWCFR` writer"]
pub type W = crate::W<DfsdmFlt2awcfrSpec>;
#[doc = "Field `CLRAWLTF` reader - Clear the analog watchdog low threshold flag"]
pub type ClrawltfR = crate::FieldReader;
#[doc = "Field `CLRAWLTF` writer - Clear the analog watchdog low threshold flag"]
pub type ClrawltfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLRAWHTF` reader - Clear the analog watchdog high threshold flag"]
pub type ClrawhtfR = crate::FieldReader;
#[doc = "Field `CLRAWHTF` writer - Clear the analog watchdog high threshold flag"]
pub type ClrawhtfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clear the analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn clrawltf(&self) -> ClrawltfR {
        ClrawltfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Clear the analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn clrawhtf(&self) -> ClrawhtfR {
        ClrawhtfR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clear the analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn clrawltf(&mut self) -> ClrawltfW<DfsdmFlt2awcfrSpec> {
        ClrawltfW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Clear the analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn clrawhtf(&mut self) -> ClrawhtfW<DfsdmFlt2awcfrSpec> {
        ClrawhtfW::new(self, 8)
    }
}
#[doc = "analog watchdog clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2awcfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2awcfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt2awcfrSpec;
impl crate::RegisterSpec for DfsdmFlt2awcfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt2awcfr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt2awcfrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt2awcfr::W`](W) writer structure"]
impl crate::Writable for DfsdmFlt2awcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFSDM_FLT2AWCFR to value 0"]
impl crate::Resettable for DfsdmFlt2awcfrSpec {}

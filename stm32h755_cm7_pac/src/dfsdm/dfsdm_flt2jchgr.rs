#[doc = "Register `DFSDM_FLT2JCHGR` reader"]
pub type R = crate::R<DfsdmFlt2jchgrSpec>;
#[doc = "Register `DFSDM_FLT2JCHGR` writer"]
pub type W = crate::W<DfsdmFlt2jchgrSpec>;
#[doc = "Field `JCHG` reader - Injected channel group selection"]
pub type JchgR = crate::FieldReader;
#[doc = "Field `JCHG` writer - Injected channel group selection"]
pub type JchgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&self) -> JchgR {
        JchgR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Injected channel group selection"]
    #[inline(always)]
    pub fn jchg(&mut self) -> JchgW<DfsdmFlt2jchgrSpec> {
        JchgW::new(self, 0)
    }
}
#[doc = "injected channel group selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2jchgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt2jchgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt2jchgrSpec;
impl crate::RegisterSpec for DfsdmFlt2jchgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt2jchgr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt2jchgrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt2jchgr::W`](W) writer structure"]
impl crate::Writable for DfsdmFlt2jchgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFSDM_FLT2JCHGR to value 0x01"]
impl crate::Resettable for DfsdmFlt2jchgrSpec {
    const RESET_VALUE: u32 = 0x01;
}

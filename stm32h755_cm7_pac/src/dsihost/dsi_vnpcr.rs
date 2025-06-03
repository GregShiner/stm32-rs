#[doc = "Register `DSI_VNPCR` reader"]
pub type R = crate::R<DsiVnpcrSpec>;
#[doc = "Register `DSI_VNPCR` writer"]
pub type W = crate::W<DsiVnpcrSpec>;
#[doc = "Field `NPSIZE` reader - NPSIZE"]
pub type NpsizeR = crate::FieldReader<u16>;
#[doc = "Field `NPSIZE` writer - NPSIZE"]
pub type NpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    pub fn npsize(&self) -> NpsizeR {
        NpsizeR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - NPSIZE"]
    #[inline(always)]
    pub fn npsize(&mut self) -> NpsizeW<DsiVnpcrSpec> {
        NpsizeW::new(self, 0)
    }
}
#[doc = "DSI Host video null packet configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vnpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vnpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVnpcrSpec;
impl crate::RegisterSpec for DsiVnpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vnpcr::R`](R) reader structure"]
impl crate::Readable for DsiVnpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vnpcr::W`](W) writer structure"]
impl crate::Writable for DsiVnpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VNPCR to value 0"]
impl crate::Resettable for DsiVnpcrSpec {}

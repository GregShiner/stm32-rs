#[doc = "Register `OTG_HS_HCDMA4` reader"]
pub type R = crate::R<OtgHsHcdma4Spec>;
#[doc = "Register `OTG_HS_HCDMA4` writer"]
pub type W = crate::W<OtgHsHcdma4Spec>;
#[doc = "Field `DMAADDR` reader - DMA address"]
pub type DmaaddrR = crate::FieldReader<u32>;
#[doc = "Field `DMAADDR` writer - DMA address"]
pub type DmaaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&self) -> DmaaddrR {
        DmaaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA address"]
    #[inline(always)]
    pub fn dmaaddr(&mut self) -> DmaaddrW<OtgHsHcdma4Spec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "OTG_HS host channel-4 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHcdma4Spec;
impl crate::RegisterSpec for OtgHsHcdma4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcdma4::R`](R) reader structure"]
impl crate::Readable for OtgHsHcdma4Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcdma4::W`](W) writer structure"]
impl crate::Writable for OtgHsHcdma4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_HCDMA4 to value 0"]
impl crate::Resettable for OtgHsHcdma4Spec {}

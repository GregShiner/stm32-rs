#[doc = "Register `OTG_HS_HCDMA6` reader"]
pub type R = crate::R<OtgHsHcdma6Spec>;
#[doc = "Register `OTG_HS_HCDMA6` writer"]
pub type W = crate::W<OtgHsHcdma6Spec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<OtgHsHcdma6Spec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "OTG_HS host channel-6 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHcdma6Spec;
impl crate::RegisterSpec for OtgHsHcdma6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcdma6::R`](R) reader structure"]
impl crate::Readable for OtgHsHcdma6Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcdma6::W`](W) writer structure"]
impl crate::Writable for OtgHsHcdma6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_HCDMA6 to value 0"]
impl crate::Resettable for OtgHsHcdma6Spec {}

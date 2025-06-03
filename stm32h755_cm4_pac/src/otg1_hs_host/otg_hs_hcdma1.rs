#[doc = "Register `OTG_HS_HCDMA1` reader"]
pub type R = crate::R<OtgHsHcdma1Spec>;
#[doc = "Register `OTG_HS_HCDMA1` writer"]
pub type W = crate::W<OtgHsHcdma1Spec>;
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
    pub fn dmaaddr(&mut self) -> DmaaddrW<OtgHsHcdma1Spec> {
        DmaaddrW::new(self, 0)
    }
}
#[doc = "OTG_HS host channel-1 DMA address register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hcdma1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcdma1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHcdma1Spec;
impl crate::RegisterSpec for OtgHsHcdma1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcdma1::R`](R) reader structure"]
impl crate::Readable for OtgHsHcdma1Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcdma1::W`](W) writer structure"]
impl crate::Writable for OtgHsHcdma1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_HCDMA1 to value 0"]
impl crate::Resettable for OtgHsHcdma1Spec {}

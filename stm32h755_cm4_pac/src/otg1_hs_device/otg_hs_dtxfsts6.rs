#[doc = "Register `OTG_HS_DTXFSTS6` reader"]
pub type R = crate::R<OtgHsDtxfsts6Spec>;
#[doc = "Register `OTG_HS_DTXFSTS6` writer"]
pub type W = crate::W<OtgHsDtxfsts6Spec>;
#[doc = "Field `INEPTFSAV` reader - IN endpoint TxFIFO space avail"]
pub type IneptfsavR = crate::FieldReader<u16>;
#[doc = "Field `INEPTFSAV` writer - IN endpoint TxFIFO space avail"]
pub type IneptfsavW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&self) -> IneptfsavR {
        IneptfsavR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - IN endpoint TxFIFO space avail"]
    #[inline(always)]
    pub fn ineptfsav(&mut self) -> IneptfsavW<OtgHsDtxfsts6Spec> {
        IneptfsavW::new(self, 0)
    }
}
#[doc = "OTG_HS device IN endpoint transmit FIFO status register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_dtxfsts6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dtxfsts6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDtxfsts6Spec;
impl crate::RegisterSpec for OtgHsDtxfsts6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dtxfsts6::R`](R) reader structure"]
impl crate::Readable for OtgHsDtxfsts6Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dtxfsts6::W`](W) writer structure"]
impl crate::Writable for OtgHsDtxfsts6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DTXFSTS6 to value 0"]
impl crate::Resettable for OtgHsDtxfsts6Spec {}

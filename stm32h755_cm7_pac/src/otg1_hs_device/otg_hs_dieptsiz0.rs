#[doc = "Register `OTG_HS_DIEPTSIZ0` reader"]
pub type R = crate::R<OtgHsDieptsiz0Spec>;
#[doc = "Register `OTG_HS_DIEPTSIZ0` writer"]
pub type W = crate::W<OtgHsDieptsiz0Spec>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XfrsizR = crate::FieldReader;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XfrsizW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::FieldReader;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XfrsizR {
        XfrsizR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XfrsizW<OtgHsDieptsiz0Spec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bits 19:20 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<OtgHsDieptsiz0Spec> {
        PktcntW::new(self, 19)
    }
}
#[doc = "OTG_HS device IN endpoint 0 transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptsiz0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptsiz0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDieptsiz0Spec;
impl crate::RegisterSpec for OtgHsDieptsiz0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dieptsiz0::R`](R) reader structure"]
impl crate::Readable for OtgHsDieptsiz0Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dieptsiz0::W`](W) writer structure"]
impl crate::Writable for OtgHsDieptsiz0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DIEPTSIZ0 to value 0"]
impl crate::Resettable for OtgHsDieptsiz0Spec {}

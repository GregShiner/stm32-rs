#[doc = "Register `OTG_HS_DIEPTSIZ7` reader"]
pub type R = crate::R<OtgHsDieptsiz7Spec>;
#[doc = "Register `OTG_HS_DIEPTSIZ7` writer"]
pub type W = crate::W<OtgHsDieptsiz7Spec>;
#[doc = "Field `XFRSIZ` reader - Transfer size"]
pub type XfrsizR = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - Transfer size"]
pub type XfrsizW<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - Packet count"]
pub type PktcntR = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - Packet count"]
pub type PktcntW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MCNT` reader - Multi count"]
pub type McntR = crate::FieldReader;
#[doc = "Field `MCNT` writer - Multi count"]
pub type McntW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XfrsizR {
        XfrsizR::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PktcntR {
        PktcntR::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    pub fn mcnt(&self) -> McntR {
        McntR::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - Transfer size"]
    #[inline(always)]
    pub fn xfrsiz(&mut self) -> XfrsizW<OtgHsDieptsiz7Spec> {
        XfrsizW::new(self, 0)
    }
    #[doc = "Bits 19:28 - Packet count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PktcntW<OtgHsDieptsiz7Spec> {
        PktcntW::new(self, 19)
    }
    #[doc = "Bits 29:30 - Multi count"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> McntW<OtgHsDieptsiz7Spec> {
        McntW::new(self, 29)
    }
}
#[doc = "OTG_HS device endpoint transfer size register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_dieptsiz7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dieptsiz7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDieptsiz7Spec;
impl crate::RegisterSpec for OtgHsDieptsiz7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dieptsiz7::R`](R) reader structure"]
impl crate::Readable for OtgHsDieptsiz7Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dieptsiz7::W`](W) writer structure"]
impl crate::Writable for OtgHsDieptsiz7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DIEPTSIZ7 to value 0"]
impl crate::Resettable for OtgHsDieptsiz7Spec {}

#[doc = "Register `OTG_HS_DOEPINT1` reader"]
pub type R = crate::R<OtgHsDoepint1Spec>;
#[doc = "Register `OTG_HS_DOEPINT1` writer"]
pub type W = crate::W<OtgHsDoepint1Spec>;
#[doc = "Field `XFRC` reader - Transfer completed interrupt"]
pub type XfrcR = crate::BitReader;
#[doc = "Field `XFRC` writer - Transfer completed interrupt"]
pub type XfrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDISD` reader - Endpoint disabled interrupt"]
pub type EpdisdR = crate::BitReader;
#[doc = "Field `EPDISD` writer - Endpoint disabled interrupt"]
pub type EpdisdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STUP` reader - SETUP phase done"]
pub type StupR = crate::BitReader;
#[doc = "Field `STUP` writer - SETUP phase done"]
pub type StupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTEPDIS` reader - OUT token received when endpoint disabled"]
pub type OtepdisR = crate::BitReader;
#[doc = "Field `OTEPDIS` writer - OUT token received when endpoint disabled"]
pub type OtepdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B2BSTUP` reader - Back-to-back SETUP packets received"]
pub type B2bstupR = crate::BitReader;
#[doc = "Field `B2BSTUP` writer - Back-to-back SETUP packets received"]
pub type B2bstupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NYET` reader - NYET interrupt"]
pub type NyetR = crate::BitReader;
#[doc = "Field `NYET` writer - NYET interrupt"]
pub type NyetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&self) -> XfrcR {
        XfrcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&self) -> EpdisdR {
        EpdisdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&self) -> StupR {
        StupR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&self) -> OtepdisR {
        OtepdisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&self) -> B2bstupR {
        B2bstupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&self) -> NyetR {
        NyetR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer completed interrupt"]
    #[inline(always)]
    pub fn xfrc(&mut self) -> XfrcW<OtgHsDoepint1Spec> {
        XfrcW::new(self, 0)
    }
    #[doc = "Bit 1 - Endpoint disabled interrupt"]
    #[inline(always)]
    pub fn epdisd(&mut self) -> EpdisdW<OtgHsDoepint1Spec> {
        EpdisdW::new(self, 1)
    }
    #[doc = "Bit 3 - SETUP phase done"]
    #[inline(always)]
    pub fn stup(&mut self) -> StupW<OtgHsDoepint1Spec> {
        StupW::new(self, 3)
    }
    #[doc = "Bit 4 - OUT token received when endpoint disabled"]
    #[inline(always)]
    pub fn otepdis(&mut self) -> OtepdisW<OtgHsDoepint1Spec> {
        OtepdisW::new(self, 4)
    }
    #[doc = "Bit 6 - Back-to-back SETUP packets received"]
    #[inline(always)]
    pub fn b2bstup(&mut self) -> B2bstupW<OtgHsDoepint1Spec> {
        B2bstupW::new(self, 6)
    }
    #[doc = "Bit 14 - NYET interrupt"]
    #[inline(always)]
    pub fn nyet(&mut self) -> NyetW<OtgHsDoepint1Spec> {
        NyetW::new(self, 14)
    }
}
#[doc = "OTG_HS device endpoint-1 interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_doepint1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepint1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDoepint1Spec;
impl crate::RegisterSpec for OtgHsDoepint1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_doepint1::R`](R) reader structure"]
impl crate::Readable for OtgHsDoepint1Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_doepint1::W`](W) writer structure"]
impl crate::Writable for OtgHsDoepint1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DOEPINT1 to value 0"]
impl crate::Resettable for OtgHsDoepint1Spec {}

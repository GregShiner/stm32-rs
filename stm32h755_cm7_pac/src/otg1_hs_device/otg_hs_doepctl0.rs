#[doc = "Register `OTG_HS_DOEPCTL0` reader"]
pub type R = crate::R<OtgHsDoepctl0Spec>;
#[doc = "Register `OTG_HS_DOEPCTL0` writer"]
pub type W = crate::W<OtgHsDoepctl0Spec>;
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub type MpsizR = crate::FieldReader;
#[doc = "Field `USBAEP` reader - USB active endpoint"]
pub type UsbaepR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub type EptypR = crate::FieldReader;
#[doc = "Field `SNPM` reader - Snoop mode"]
pub type SnpmR = crate::BitReader;
#[doc = "Field `SNPM` writer - Snoop mode"]
pub type SnpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Stall` reader - STALL handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `Stall` writer - STALL handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MpsizR {
        MpsizR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbaep(&self) -> UsbaepR {
        UsbaepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - NAK status"]
    #[inline(always)]
    pub fn naksts(&self) -> NakstsR {
        NakstsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&self) -> EptypR {
        EptypR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snpm(&self) -> SnpmR {
        SnpmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 20 - Snoop mode"]
    #[inline(always)]
    pub fn snpm(&mut self) -> SnpmW<OtgHsDoepctl0Spec> {
        SnpmW::new(self, 20)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<OtgHsDoepctl0Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<OtgHsDoepctl0Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<OtgHsDoepctl0Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<OtgHsDoepctl0Spec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "OTG_HS device control OUT endpoint 0 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_doepctl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepctl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDoepctl0Spec;
impl crate::RegisterSpec for OtgHsDoepctl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_doepctl0::R`](R) reader structure"]
impl crate::Readable for OtgHsDoepctl0Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_doepctl0::W`](W) writer structure"]
impl crate::Writable for OtgHsDoepctl0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DOEPCTL0 to value 0x8000"]
impl crate::Resettable for OtgHsDoepctl0Spec {
    const RESET_VALUE: u32 = 0x8000;
}

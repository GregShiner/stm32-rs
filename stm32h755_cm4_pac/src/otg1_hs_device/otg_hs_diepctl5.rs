#[doc = "Register `OTG_HS_DIEPCTL5` reader"]
pub type R = crate::R<OtgHsDiepctl5Spec>;
#[doc = "Register `OTG_HS_DIEPCTL5` writer"]
pub type W = crate::W<OtgHsDiepctl5Spec>;
#[doc = "Field `MPSIZ` reader - Maximum packet size"]
pub type MpsizR = crate::FieldReader<u16>;
#[doc = "Field `MPSIZ` writer - Maximum packet size"]
pub type MpsizW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBAEP` reader - USB active endpoint"]
pub type UsbaepR = crate::BitReader;
#[doc = "Field `USBAEP` writer - USB active endpoint"]
pub type UsbaepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EONUM_DPID` reader - Even/odd frame"]
pub type EonumDpidR = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAK status"]
pub type NakstsR = crate::BitReader;
#[doc = "Field `EPTYP` reader - Endpoint type"]
pub type EptypR = crate::FieldReader;
#[doc = "Field `EPTYP` writer - Endpoint type"]
pub type EptypW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `Stall` reader - STALL handshake"]
pub type StallR = crate::BitReader;
#[doc = "Field `Stall` writer - STALL handshake"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFNUM` reader - TxFIFO number"]
pub type TxfnumR = crate::FieldReader;
#[doc = "Field `TXFNUM` writer - TxFIFO number"]
pub type TxfnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CNAK` writer - Clear NAK"]
pub type CnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - Set NAK"]
pub type SnakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD0PID_SEVNFRM` writer - Set DATA0 PID"]
pub type Sd0pidSevnfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SODDFRM` writer - Set odd frame"]
pub type SoddfrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - Endpoint disable"]
pub type EpdisR = crate::BitReader;
#[doc = "Field `EPDIS` writer - Endpoint disable"]
pub type EpdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - Endpoint enable"]
pub type EpenaR = crate::BitReader;
#[doc = "Field `EPENA` writer - Endpoint enable"]
pub type EpenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MpsizR {
        MpsizR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbaep(&self) -> UsbaepR {
        UsbaepR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Even/odd frame"]
    #[inline(always)]
    pub fn eonum_dpid(&self) -> EonumDpidR {
        EonumDpidR::new(((self.bits >> 16) & 1) != 0)
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
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&self) -> TxfnumR {
        TxfnumR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epdis(&self) -> EpdisR {
        EpdisR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epena(&self) -> EpenaR {
        EpenaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum packet size"]
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MpsizW<OtgHsDiepctl5Spec> {
        MpsizW::new(self, 0)
    }
    #[doc = "Bit 15 - USB active endpoint"]
    #[inline(always)]
    pub fn usbaep(&mut self) -> UsbaepW<OtgHsDiepctl5Spec> {
        UsbaepW::new(self, 15)
    }
    #[doc = "Bits 18:19 - Endpoint type"]
    #[inline(always)]
    pub fn eptyp(&mut self) -> EptypW<OtgHsDiepctl5Spec> {
        EptypW::new(self, 18)
    }
    #[doc = "Bit 21 - STALL handshake"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<OtgHsDiepctl5Spec> {
        StallW::new(self, 21)
    }
    #[doc = "Bits 22:25 - TxFIFO number"]
    #[inline(always)]
    pub fn txfnum(&mut self) -> TxfnumW<OtgHsDiepctl5Spec> {
        TxfnumW::new(self, 22)
    }
    #[doc = "Bit 26 - Clear NAK"]
    #[inline(always)]
    pub fn cnak(&mut self) -> CnakW<OtgHsDiepctl5Spec> {
        CnakW::new(self, 26)
    }
    #[doc = "Bit 27 - Set NAK"]
    #[inline(always)]
    pub fn snak(&mut self) -> SnakW<OtgHsDiepctl5Spec> {
        SnakW::new(self, 27)
    }
    #[doc = "Bit 28 - Set DATA0 PID"]
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&mut self) -> Sd0pidSevnfrmW<OtgHsDiepctl5Spec> {
        Sd0pidSevnfrmW::new(self, 28)
    }
    #[doc = "Bit 29 - Set odd frame"]
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SoddfrmW<OtgHsDiepctl5Spec> {
        SoddfrmW::new(self, 29)
    }
    #[doc = "Bit 30 - Endpoint disable"]
    #[inline(always)]
    pub fn epdis(&mut self) -> EpdisW<OtgHsDiepctl5Spec> {
        EpdisW::new(self, 30)
    }
    #[doc = "Bit 31 - Endpoint enable"]
    #[inline(always)]
    pub fn epena(&mut self) -> EpenaW<OtgHsDiepctl5Spec> {
        EpenaW::new(self, 31)
    }
}
#[doc = "OTG device endpoint-5 control register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_diepctl5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_diepctl5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDiepctl5Spec;
impl crate::RegisterSpec for OtgHsDiepctl5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_diepctl5::R`](R) reader structure"]
impl crate::Readable for OtgHsDiepctl5Spec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_diepctl5::W`](W) writer structure"]
impl crate::Writable for OtgHsDiepctl5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DIEPCTL5 to value 0"]
impl crate::Resettable for OtgHsDiepctl5Spec {}

#[doc = "Register `OTG_HS_GLPMCFG` reader"]
pub type R = crate::R<OtgHsGlpmcfgSpec>;
#[doc = "Register `OTG_HS_GLPMCFG` writer"]
pub type W = crate::W<OtgHsGlpmcfgSpec>;
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub type LpmenR = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub type LpmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable"]
pub type LpmackR = crate::BitReader;
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable"]
pub type LpmackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BESL` reader - Best effort service latency"]
pub type BeslR = crate::FieldReader;
#[doc = "Field `REMWAKE` reader - bRemoteWake value"]
pub type RemwakeR = crate::BitReader;
#[doc = "Field `L1SSEN` reader - L1 Shallow Sleep enable"]
pub type L1ssenR = crate::BitReader;
#[doc = "Field `L1SSEN` writer - L1 Shallow Sleep enable"]
pub type L1ssenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BESLTHRS` reader - BESL threshold"]
pub type BeslthrsR = crate::FieldReader;
#[doc = "Field `BESLTHRS` writer - BESL threshold"]
pub type BeslthrsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L1DSEN` reader - L1 deep sleep enable"]
pub type L1dsenR = crate::BitReader;
#[doc = "Field `L1DSEN` writer - L1 deep sleep enable"]
pub type L1dsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRST` reader - LPM response"]
pub type LpmrstR = crate::FieldReader;
#[doc = "Field `SLPSTS` reader - Port sleep status"]
pub type SlpstsR = crate::BitReader;
#[doc = "Field `L1RSMOK` reader - Sleep State Resume OK"]
pub type L1rsmokR = crate::BitReader;
#[doc = "Field `LPMCHIDX` reader - LPM Channel Index"]
pub type LpmchidxR = crate::FieldReader;
#[doc = "Field `LPMCHIDX` writer - LPM Channel Index"]
pub type LpmchidxW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPMRCNT` reader - LPM retry count"]
pub type LpmrcntR = crate::FieldReader;
#[doc = "Field `LPMRCNT` writer - LPM retry count"]
pub type LpmrcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SNDLPM` reader - Send LPM transaction"]
pub type SndlpmR = crate::BitReader;
#[doc = "Field `SNDLPM` writer - Send LPM transaction"]
pub type SndlpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRCNTSTS` reader - LPM retry count status"]
pub type LpmrcntstsR = crate::FieldReader;
#[doc = "Field `ENBESL` reader - Enable best effort service latency"]
pub type EnbeslR = crate::BitReader;
#[doc = "Field `ENBESL` writer - Enable best effort service latency"]
pub type EnbeslW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LpmenR {
        LpmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LpmackR {
        LpmackR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    pub fn besl(&self) -> BeslR {
        BeslR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwake(&self) -> RemwakeR {
        RemwakeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - L1 Shallow Sleep enable"]
    #[inline(always)]
    pub fn l1ssen(&self) -> L1ssenR {
        L1ssenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    pub fn beslthrs(&self) -> BeslthrsR {
        BeslthrsR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - L1 deep sleep enable"]
    #[inline(always)]
    pub fn l1dsen(&self) -> L1dsenR {
        L1dsenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - LPM response"]
    #[inline(always)]
    pub fn lpmrst(&self) -> LpmrstR {
        LpmrstR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port sleep status"]
    #[inline(always)]
    pub fn slpsts(&self) -> SlpstsR {
        SlpstsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Sleep State Resume OK"]
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1rsmokR {
        L1rsmokR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - LPM Channel Index"]
    #[inline(always)]
    pub fn lpmchidx(&self) -> LpmchidxR {
        LpmchidxR::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    pub fn lpmrcnt(&self) -> LpmrcntR {
        LpmrcntR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    pub fn sndlpm(&self) -> SndlpmR {
        SndlpmR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - LPM retry count status"]
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LpmrcntstsR {
        LpmrcntstsR::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Enable best effort service latency"]
    #[inline(always)]
    pub fn enbesl(&self) -> EnbeslR {
        EnbeslR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&mut self) -> LpmenW<OtgHsGlpmcfgSpec> {
        LpmenW::new(self, 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&mut self) -> LpmackW<OtgHsGlpmcfgSpec> {
        LpmackW::new(self, 1)
    }
    #[doc = "Bit 7 - L1 Shallow Sleep enable"]
    #[inline(always)]
    pub fn l1ssen(&mut self) -> L1ssenW<OtgHsGlpmcfgSpec> {
        L1ssenW::new(self, 7)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    pub fn beslthrs(&mut self) -> BeslthrsW<OtgHsGlpmcfgSpec> {
        BeslthrsW::new(self, 8)
    }
    #[doc = "Bit 12 - L1 deep sleep enable"]
    #[inline(always)]
    pub fn l1dsen(&mut self) -> L1dsenW<OtgHsGlpmcfgSpec> {
        L1dsenW::new(self, 12)
    }
    #[doc = "Bits 17:20 - LPM Channel Index"]
    #[inline(always)]
    pub fn lpmchidx(&mut self) -> LpmchidxW<OtgHsGlpmcfgSpec> {
        LpmchidxW::new(self, 17)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    pub fn lpmrcnt(&mut self) -> LpmrcntW<OtgHsGlpmcfgSpec> {
        LpmrcntW::new(self, 21)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    pub fn sndlpm(&mut self) -> SndlpmW<OtgHsGlpmcfgSpec> {
        SndlpmW::new(self, 24)
    }
    #[doc = "Bit 28 - Enable best effort service latency"]
    #[inline(always)]
    pub fn enbesl(&mut self) -> EnbeslW<OtgHsGlpmcfgSpec> {
        EnbeslW::new(self, 28)
    }
}
#[doc = "OTG core LPM configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_glpmcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_glpmcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsGlpmcfgSpec;
impl crate::RegisterSpec for OtgHsGlpmcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_glpmcfg::R`](R) reader structure"]
impl crate::Readable for OtgHsGlpmcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_glpmcfg::W`](W) writer structure"]
impl crate::Writable for OtgHsGlpmcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_GLPMCFG to value 0"]
impl crate::Resettable for OtgHsGlpmcfgSpec {}

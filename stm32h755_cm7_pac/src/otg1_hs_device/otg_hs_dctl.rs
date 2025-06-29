#[doc = "Register `OTG_HS_DCTL` reader"]
pub type R = crate::R<OtgHsDctlSpec>;
#[doc = "Register `OTG_HS_DCTL` writer"]
pub type W = crate::W<OtgHsDctlSpec>;
#[doc = "Field `RWUSIG` reader - Remote wakeup signaling"]
pub type RwusigR = crate::BitReader;
#[doc = "Field `RWUSIG` writer - Remote wakeup signaling"]
pub type RwusigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIS` reader - Soft disconnect"]
pub type SdisR = crate::BitReader;
#[doc = "Field `SDIS` writer - Soft disconnect"]
pub type SdisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GINSTS` reader - Global IN NAK status"]
pub type GinstsR = crate::BitReader;
#[doc = "Field `GONSTS` reader - Global OUT NAK status"]
pub type GonstsR = crate::BitReader;
#[doc = "Field `TCTL` reader - Test control"]
pub type TctlR = crate::FieldReader;
#[doc = "Field `TCTL` writer - Test control"]
pub type TctlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SGINAK` writer - Set global IN NAK"]
pub type SginakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGINAK` writer - Clear global IN NAK"]
pub type CginakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SGONAK` writer - Set global OUT NAK"]
pub type SgonakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CGONAK` writer - Clear global OUT NAK"]
pub type CgonakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POPRGDNE` reader - Power-on programming done"]
pub type PoprgdneR = crate::BitReader;
#[doc = "Field `POPRGDNE` writer - Power-on programming done"]
pub type PoprgdneW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&self) -> RwusigR {
        RwusigR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&self) -> SdisR {
        SdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Global IN NAK status"]
    #[inline(always)]
    pub fn ginsts(&self) -> GinstsR {
        GinstsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Global OUT NAK status"]
    #[inline(always)]
    pub fn gonsts(&self) -> GonstsR {
        GonstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&self) -> TctlR {
        TctlR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&self) -> PoprgdneR {
        PoprgdneR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Remote wakeup signaling"]
    #[inline(always)]
    pub fn rwusig(&mut self) -> RwusigW<OtgHsDctlSpec> {
        RwusigW::new(self, 0)
    }
    #[doc = "Bit 1 - Soft disconnect"]
    #[inline(always)]
    pub fn sdis(&mut self) -> SdisW<OtgHsDctlSpec> {
        SdisW::new(self, 1)
    }
    #[doc = "Bits 4:6 - Test control"]
    #[inline(always)]
    pub fn tctl(&mut self) -> TctlW<OtgHsDctlSpec> {
        TctlW::new(self, 4)
    }
    #[doc = "Bit 7 - Set global IN NAK"]
    #[inline(always)]
    pub fn sginak(&mut self) -> SginakW<OtgHsDctlSpec> {
        SginakW::new(self, 7)
    }
    #[doc = "Bit 8 - Clear global IN NAK"]
    #[inline(always)]
    pub fn cginak(&mut self) -> CginakW<OtgHsDctlSpec> {
        CginakW::new(self, 8)
    }
    #[doc = "Bit 9 - Set global OUT NAK"]
    #[inline(always)]
    pub fn sgonak(&mut self) -> SgonakW<OtgHsDctlSpec> {
        SgonakW::new(self, 9)
    }
    #[doc = "Bit 10 - Clear global OUT NAK"]
    #[inline(always)]
    pub fn cgonak(&mut self) -> CgonakW<OtgHsDctlSpec> {
        CgonakW::new(self, 10)
    }
    #[doc = "Bit 11 - Power-on programming done"]
    #[inline(always)]
    pub fn poprgdne(&mut self) -> PoprgdneW<OtgHsDctlSpec> {
        PoprgdneW::new(self, 11)
    }
}
#[doc = "OTG_HS device control register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_dctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_dctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDctlSpec;
impl crate::RegisterSpec for OtgHsDctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_dctl::R`](R) reader structure"]
impl crate::Readable for OtgHsDctlSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_dctl::W`](W) writer structure"]
impl crate::Writable for OtgHsDctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_DCTL to value 0"]
impl crate::Resettable for OtgHsDctlSpec {}

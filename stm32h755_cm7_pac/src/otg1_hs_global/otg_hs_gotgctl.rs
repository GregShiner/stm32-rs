#[doc = "Register `OTG_HS_GOTGCTL` reader"]
pub type R = crate::R<OtgHsGotgctlSpec>;
#[doc = "Register `OTG_HS_GOTGCTL` writer"]
pub type W = crate::W<OtgHsGotgctlSpec>;
#[doc = "Field `SRQSCS` reader - Session request success"]
pub type SrqscsR = crate::BitReader;
#[doc = "Field `SRQ` reader - Session request"]
pub type SrqR = crate::BitReader;
#[doc = "Field `SRQ` writer - Session request"]
pub type SrqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HNGSCS` reader - Host negotiation success"]
pub type HngscsR = crate::BitReader;
#[doc = "Field `HNPRQ` reader - HNP request"]
pub type HnprqR = crate::BitReader;
#[doc = "Field `HNPRQ` writer - HNP request"]
pub type HnprqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSHNPEN` reader - Host set HNP enable"]
pub type HshnpenR = crate::BitReader;
#[doc = "Field `HSHNPEN` writer - Host set HNP enable"]
pub type HshnpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHNPEN` reader - Device HNP enabled"]
pub type DhnpenR = crate::BitReader;
#[doc = "Field `DHNPEN` writer - Device HNP enabled"]
pub type DhnpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EHEN` reader - Embedded host enable"]
pub type EhenR = crate::BitReader;
#[doc = "Field `EHEN` writer - Embedded host enable"]
pub type EhenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIDSTS` reader - Connector ID status"]
pub type CidstsR = crate::BitReader;
#[doc = "Field `DBCT` reader - Long/short debounce time"]
pub type DbctR = crate::BitReader;
#[doc = "Field `ASVLD` reader - A-session valid"]
pub type AsvldR = crate::BitReader;
#[doc = "Field `BSVLD` reader - B-session valid"]
pub type BsvldR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Session request success"]
    #[inline(always)]
    pub fn srqscs(&self) -> SrqscsR {
        SrqscsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&self) -> SrqR {
        SrqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Host negotiation success"]
    #[inline(always)]
    pub fn hngscs(&self) -> HngscsR {
        HngscsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&self) -> HnprqR {
        HnprqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&self) -> HshnpenR {
        HshnpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&self) -> DhnpenR {
        DhnpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehen(&self) -> EhenR {
        EhenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Connector ID status"]
    #[inline(always)]
    pub fn cidsts(&self) -> CidstsR {
        CidstsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Long/short debounce time"]
    #[inline(always)]
    pub fn dbct(&self) -> DbctR {
        DbctR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A-session valid"]
    #[inline(always)]
    pub fn asvld(&self) -> AsvldR {
        AsvldR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B-session valid"]
    #[inline(always)]
    pub fn bsvld(&self) -> BsvldR {
        BsvldR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Session request"]
    #[inline(always)]
    pub fn srq(&mut self) -> SrqW<OtgHsGotgctlSpec> {
        SrqW::new(self, 1)
    }
    #[doc = "Bit 9 - HNP request"]
    #[inline(always)]
    pub fn hnprq(&mut self) -> HnprqW<OtgHsGotgctlSpec> {
        HnprqW::new(self, 9)
    }
    #[doc = "Bit 10 - Host set HNP enable"]
    #[inline(always)]
    pub fn hshnpen(&mut self) -> HshnpenW<OtgHsGotgctlSpec> {
        HshnpenW::new(self, 10)
    }
    #[doc = "Bit 11 - Device HNP enabled"]
    #[inline(always)]
    pub fn dhnpen(&mut self) -> DhnpenW<OtgHsGotgctlSpec> {
        DhnpenW::new(self, 11)
    }
    #[doc = "Bit 12 - Embedded host enable"]
    #[inline(always)]
    pub fn ehen(&mut self) -> EhenW<OtgHsGotgctlSpec> {
        EhenW::new(self, 12)
    }
}
#[doc = "OTG_HS control and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_gotgctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gotgctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsGotgctlSpec;
impl crate::RegisterSpec for OtgHsGotgctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_gotgctl::R`](R) reader structure"]
impl crate::Readable for OtgHsGotgctlSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_gotgctl::W`](W) writer structure"]
impl crate::Writable for OtgHsGotgctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_GOTGCTL to value 0x0800"]
impl crate::Resettable for OtgHsGotgctlSpec {
    const RESET_VALUE: u32 = 0x0800;
}

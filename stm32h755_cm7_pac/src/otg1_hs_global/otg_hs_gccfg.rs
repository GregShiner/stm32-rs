#[doc = "Register `OTG_HS_GCCFG` reader"]
pub type R = crate::R<OtgHsGccfgSpec>;
#[doc = "Register `OTG_HS_GCCFG` writer"]
pub type W = crate::W<OtgHsGccfgSpec>;
#[doc = "Field `DCDET` reader - Data contact detection (DCD) status"]
pub type DcdetR = crate::BitReader;
#[doc = "Field `DCDET` writer - Data contact detection (DCD) status"]
pub type DcdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDET` reader - Primary detection (PD) status"]
pub type PdetR = crate::BitReader;
#[doc = "Field `PDET` writer - Primary detection (PD) status"]
pub type PdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDET` reader - Secondary detection (SD) status"]
pub type SdetR = crate::BitReader;
#[doc = "Field `SDET` writer - Secondary detection (SD) status"]
pub type SdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PS2DET` reader - DM pull-up detection status"]
pub type Ps2detR = crate::BitReader;
#[doc = "Field `PS2DET` writer - DM pull-up detection status"]
pub type Ps2detW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWRDWN` reader - Power down"]
pub type PwrdwnR = crate::BitReader;
#[doc = "Field `PWRDWN` writer - Power down"]
pub type PwrdwnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCDEN` reader - Battery charging detector (BCD) enable"]
pub type BcdenR = crate::BitReader;
#[doc = "Field `BCDEN` writer - Battery charging detector (BCD) enable"]
pub type BcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDEN` reader - Data contact detection (DCD) mode enable"]
pub type DcdenR = crate::BitReader;
#[doc = "Field `DCDEN` writer - Data contact detection (DCD) mode enable"]
pub type DcdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - Primary detection (PD) mode enable"]
pub type PdenR = crate::BitReader;
#[doc = "Field `PDEN` writer - Primary detection (PD) mode enable"]
pub type PdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDEN` reader - Secondary detection (SD) mode enable"]
pub type SdenR = crate::BitReader;
#[doc = "Field `SDEN` writer - Secondary detection (SD) mode enable"]
pub type SdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBDEN` reader - USB VBUS detection enable"]
pub type VbdenR = crate::BitReader;
#[doc = "Field `VBDEN` writer - USB VBUS detection enable"]
pub type VbdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data contact detection (DCD) status"]
    #[inline(always)]
    pub fn dcdet(&self) -> DcdetR {
        DcdetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Primary detection (PD) status"]
    #[inline(always)]
    pub fn pdet(&self) -> PdetR {
        PdetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Secondary detection (SD) status"]
    #[inline(always)]
    pub fn sdet(&self) -> SdetR {
        SdetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DM pull-up detection status"]
    #[inline(always)]
    pub fn ps2det(&self) -> Ps2detR {
        Ps2detR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&self) -> PwrdwnR {
        PwrdwnR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Battery charging detector (BCD) enable"]
    #[inline(always)]
    pub fn bcden(&self) -> BcdenR {
        BcdenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Data contact detection (DCD) mode enable"]
    #[inline(always)]
    pub fn dcden(&self) -> DcdenR {
        DcdenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Primary detection (PD) mode enable"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Secondary detection (SD) mode enable"]
    #[inline(always)]
    pub fn sden(&self) -> SdenR {
        SdenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - USB VBUS detection enable"]
    #[inline(always)]
    pub fn vbden(&self) -> VbdenR {
        VbdenR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data contact detection (DCD) status"]
    #[inline(always)]
    pub fn dcdet(&mut self) -> DcdetW<OtgHsGccfgSpec> {
        DcdetW::new(self, 0)
    }
    #[doc = "Bit 1 - Primary detection (PD) status"]
    #[inline(always)]
    pub fn pdet(&mut self) -> PdetW<OtgHsGccfgSpec> {
        PdetW::new(self, 1)
    }
    #[doc = "Bit 2 - Secondary detection (SD) status"]
    #[inline(always)]
    pub fn sdet(&mut self) -> SdetW<OtgHsGccfgSpec> {
        SdetW::new(self, 2)
    }
    #[doc = "Bit 3 - DM pull-up detection status"]
    #[inline(always)]
    pub fn ps2det(&mut self) -> Ps2detW<OtgHsGccfgSpec> {
        Ps2detW::new(self, 3)
    }
    #[doc = "Bit 16 - Power down"]
    #[inline(always)]
    pub fn pwrdwn(&mut self) -> PwrdwnW<OtgHsGccfgSpec> {
        PwrdwnW::new(self, 16)
    }
    #[doc = "Bit 17 - Battery charging detector (BCD) enable"]
    #[inline(always)]
    pub fn bcden(&mut self) -> BcdenW<OtgHsGccfgSpec> {
        BcdenW::new(self, 17)
    }
    #[doc = "Bit 18 - Data contact detection (DCD) mode enable"]
    #[inline(always)]
    pub fn dcden(&mut self) -> DcdenW<OtgHsGccfgSpec> {
        DcdenW::new(self, 18)
    }
    #[doc = "Bit 19 - Primary detection (PD) mode enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PdenW<OtgHsGccfgSpec> {
        PdenW::new(self, 19)
    }
    #[doc = "Bit 20 - Secondary detection (SD) mode enable"]
    #[inline(always)]
    pub fn sden(&mut self) -> SdenW<OtgHsGccfgSpec> {
        SdenW::new(self, 20)
    }
    #[doc = "Bit 21 - USB VBUS detection enable"]
    #[inline(always)]
    pub fn vbden(&mut self) -> VbdenW<OtgHsGccfgSpec> {
        VbdenW::new(self, 21)
    }
}
#[doc = "OTG_HS general core configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_gccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_gccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsGccfgSpec;
impl crate::RegisterSpec for OtgHsGccfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_gccfg::R`](R) reader structure"]
impl crate::Readable for OtgHsGccfgSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_gccfg::W`](W) writer structure"]
impl crate::Writable for OtgHsGccfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_GCCFG to value 0"]
impl crate::Resettable for OtgHsGccfgSpec {}

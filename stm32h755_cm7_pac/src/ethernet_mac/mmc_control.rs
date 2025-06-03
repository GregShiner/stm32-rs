#[doc = "Register `MMC_CONTROL` reader"]
pub type R = crate::R<MmcControlSpec>;
#[doc = "Register `MMC_CONTROL` writer"]
pub type W = crate::W<MmcControlSpec>;
#[doc = "Field `CNTRST` reader - CNTRST"]
pub type CntrstR = crate::BitReader;
#[doc = "Field `CNTRST` writer - CNTRST"]
pub type CntrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTSTOPRO` reader - CNTSTOPRO"]
pub type CntstoproR = crate::BitReader;
#[doc = "Field `CNTSTOPRO` writer - CNTSTOPRO"]
pub type CntstoproW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTONRD` reader - RSTONRD"]
pub type RstonrdR = crate::BitReader;
#[doc = "Field `RSTONRD` writer - RSTONRD"]
pub type RstonrdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTFREEZ` reader - CNTFREEZ"]
pub type CntfreezR = crate::BitReader;
#[doc = "Field `CNTFREEZ` writer - CNTFREEZ"]
pub type CntfreezW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRST` reader - CNTPRST"]
pub type CntprstR = crate::BitReader;
#[doc = "Field `CNTPRST` writer - CNTPRST"]
pub type CntprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTPRSTLVL` reader - CNTPRSTLVL"]
pub type CntprstlvlR = crate::BitReader;
#[doc = "Field `CNTPRSTLVL` writer - CNTPRSTLVL"]
pub type CntprstlvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCDBC` reader - UCDBC"]
pub type UcdbcR = crate::BitReader;
#[doc = "Field `UCDBC` writer - UCDBC"]
pub type UcdbcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CNTRST"]
    #[inline(always)]
    pub fn cntrst(&self) -> CntrstR {
        CntrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CNTSTOPRO"]
    #[inline(always)]
    pub fn cntstopro(&self) -> CntstoproR {
        CntstoproR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RSTONRD"]
    #[inline(always)]
    pub fn rstonrd(&self) -> RstonrdR {
        RstonrdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CNTFREEZ"]
    #[inline(always)]
    pub fn cntfreez(&self) -> CntfreezR {
        CntfreezR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&self) -> CntprstR {
        CntprstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CNTPRSTLVL"]
    #[inline(always)]
    pub fn cntprstlvl(&self) -> CntprstlvlR {
        CntprstlvlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - UCDBC"]
    #[inline(always)]
    pub fn ucdbc(&self) -> UcdbcR {
        UcdbcR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CNTRST"]
    #[inline(always)]
    pub fn cntrst(&mut self) -> CntrstW<MmcControlSpec> {
        CntrstW::new(self, 0)
    }
    #[doc = "Bit 1 - CNTSTOPRO"]
    #[inline(always)]
    pub fn cntstopro(&mut self) -> CntstoproW<MmcControlSpec> {
        CntstoproW::new(self, 1)
    }
    #[doc = "Bit 2 - RSTONRD"]
    #[inline(always)]
    pub fn rstonrd(&mut self) -> RstonrdW<MmcControlSpec> {
        RstonrdW::new(self, 2)
    }
    #[doc = "Bit 3 - CNTFREEZ"]
    #[inline(always)]
    pub fn cntfreez(&mut self) -> CntfreezW<MmcControlSpec> {
        CntfreezW::new(self, 3)
    }
    #[doc = "Bit 4 - CNTPRST"]
    #[inline(always)]
    pub fn cntprst(&mut self) -> CntprstW<MmcControlSpec> {
        CntprstW::new(self, 4)
    }
    #[doc = "Bit 5 - CNTPRSTLVL"]
    #[inline(always)]
    pub fn cntprstlvl(&mut self) -> CntprstlvlW<MmcControlSpec> {
        CntprstlvlW::new(self, 5)
    }
    #[doc = "Bit 8 - UCDBC"]
    #[inline(always)]
    pub fn ucdbc(&mut self) -> UcdbcW<MmcControlSpec> {
        UcdbcW::new(self, 8)
    }
}
#[doc = "MMC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcControlSpec;
impl crate::RegisterSpec for MmcControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_control::R`](R) reader structure"]
impl crate::Readable for MmcControlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_control::W`](W) writer structure"]
impl crate::Writable for MmcControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMC_CONTROL to value 0"]
impl crate::Resettable for MmcControlSpec {}

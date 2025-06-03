#[doc = "Register `BMCR` reader"]
pub type R = crate::R<BmcrSpec>;
#[doc = "Register `BMCR` writer"]
pub type W = crate::W<BmcrSpec>;
#[doc = "Field `BME` reader - Burst Mode enable"]
pub type BmeR = crate::BitReader;
#[doc = "Field `BME` writer - Burst Mode enable"]
pub type BmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMOM` reader - Burst Mode operating mode"]
pub type BmomR = crate::BitReader;
#[doc = "Field `BMOM` writer - Burst Mode operating mode"]
pub type BmomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMCLK` reader - Burst Mode Clock source"]
pub type BmclkR = crate::FieldReader;
#[doc = "Field `BMCLK` writer - Burst Mode Clock source"]
pub type BmclkW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BMPRSC` reader - Burst Mode Prescaler"]
pub type BmprscR = crate::FieldReader;
#[doc = "Field `BMPRSC` writer - Burst Mode Prescaler"]
pub type BmprscW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BMPREN` reader - Burst Mode Preload Enable"]
pub type BmprenR = crate::BitReader;
#[doc = "Field `BMPREN` writer - Burst Mode Preload Enable"]
pub type BmprenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTBM` reader - Master Timer Burst Mode"]
pub type MtbmR = crate::BitReader;
#[doc = "Field `MTBM` writer - Master Timer Burst Mode"]
pub type MtbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TABM` reader - Timer A Burst Mode"]
pub type TabmR = crate::BitReader;
#[doc = "Field `TABM` writer - Timer A Burst Mode"]
pub type TabmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBBM` reader - Timer B Burst Mode"]
pub type TbbmR = crate::BitReader;
#[doc = "Field `TBBM` writer - Timer B Burst Mode"]
pub type TbbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCBM` reader - Timer C Burst Mode"]
pub type TcbmR = crate::BitReader;
#[doc = "Field `TCBM` writer - Timer C Burst Mode"]
pub type TcbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDBM` reader - Timer D Burst Mode"]
pub type TdbmR = crate::BitReader;
#[doc = "Field `TDBM` writer - Timer D Burst Mode"]
pub type TdbmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEBM` reader - Timer E Burst Mode"]
pub type TebmR = crate::BitReader;
#[doc = "Field `TEBM` writer - Timer E Burst Mode"]
pub type TebmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BMSTAT` reader - Burst Mode Status"]
pub type BmstatR = crate::BitReader;
#[doc = "Field `BMSTAT` writer - Burst Mode Status"]
pub type BmstatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&self) -> BmeR {
        BmeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&self) -> BmomR {
        BmomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&self) -> BmclkR {
        BmclkR::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&self) -> BmprscR {
        BmprscR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&self) -> BmprenR {
        BmprenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&self) -> MtbmR {
        MtbmR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&self) -> TabmR {
        TabmR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&self) -> TbbmR {
        TbbmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&self) -> TcbmR {
        TcbmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&self) -> TdbmR {
        TdbmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&self) -> TebmR {
        TebmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&self) -> BmstatR {
        BmstatR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Burst Mode enable"]
    #[inline(always)]
    pub fn bme(&mut self) -> BmeW<BmcrSpec> {
        BmeW::new(self, 0)
    }
    #[doc = "Bit 1 - Burst Mode operating mode"]
    #[inline(always)]
    pub fn bmom(&mut self) -> BmomW<BmcrSpec> {
        BmomW::new(self, 1)
    }
    #[doc = "Bits 2:5 - Burst Mode Clock source"]
    #[inline(always)]
    pub fn bmclk(&mut self) -> BmclkW<BmcrSpec> {
        BmclkW::new(self, 2)
    }
    #[doc = "Bits 6:9 - Burst Mode Prescaler"]
    #[inline(always)]
    pub fn bmprsc(&mut self) -> BmprscW<BmcrSpec> {
        BmprscW::new(self, 6)
    }
    #[doc = "Bit 10 - Burst Mode Preload Enable"]
    #[inline(always)]
    pub fn bmpren(&mut self) -> BmprenW<BmcrSpec> {
        BmprenW::new(self, 10)
    }
    #[doc = "Bit 16 - Master Timer Burst Mode"]
    #[inline(always)]
    pub fn mtbm(&mut self) -> MtbmW<BmcrSpec> {
        MtbmW::new(self, 16)
    }
    #[doc = "Bit 17 - Timer A Burst Mode"]
    #[inline(always)]
    pub fn tabm(&mut self) -> TabmW<BmcrSpec> {
        TabmW::new(self, 17)
    }
    #[doc = "Bit 18 - Timer B Burst Mode"]
    #[inline(always)]
    pub fn tbbm(&mut self) -> TbbmW<BmcrSpec> {
        TbbmW::new(self, 18)
    }
    #[doc = "Bit 19 - Timer C Burst Mode"]
    #[inline(always)]
    pub fn tcbm(&mut self) -> TcbmW<BmcrSpec> {
        TcbmW::new(self, 19)
    }
    #[doc = "Bit 20 - Timer D Burst Mode"]
    #[inline(always)]
    pub fn tdbm(&mut self) -> TdbmW<BmcrSpec> {
        TdbmW::new(self, 20)
    }
    #[doc = "Bit 21 - Timer E Burst Mode"]
    #[inline(always)]
    pub fn tebm(&mut self) -> TebmW<BmcrSpec> {
        TebmW::new(self, 21)
    }
    #[doc = "Bit 31 - Burst Mode Status"]
    #[inline(always)]
    pub fn bmstat(&mut self) -> BmstatW<BmcrSpec> {
        BmstatW::new(self, 31)
    }
}
#[doc = "Burst Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmcrSpec;
impl crate::RegisterSpec for BmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcr::R`](R) reader structure"]
impl crate::Readable for BmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bmcr::W`](W) writer structure"]
impl crate::Writable for BmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMCR to value 0"]
impl crate::Resettable for BmcrSpec {}

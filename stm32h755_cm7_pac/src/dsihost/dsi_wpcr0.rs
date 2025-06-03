#[doc = "Register `DSI_WPCR0` reader"]
pub type R = crate::R<DsiWpcr0Spec>;
#[doc = "Register `DSI_WPCR0` writer"]
pub type W = crate::W<DsiWpcr0Spec>;
#[doc = "Field `UIX4` reader - UIX4"]
pub type Uix4R = crate::FieldReader;
#[doc = "Field `UIX4` writer - UIX4"]
pub type Uix4W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SWCL` reader - SWCL"]
pub type SwclR = crate::BitReader;
#[doc = "Field `SWCL` writer - SWCL"]
pub type SwclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDL0` reader - SWDL0"]
pub type Swdl0R = crate::BitReader;
#[doc = "Field `SWDL0` writer - SWDL0"]
pub type Swdl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWDL1` reader - SWDL1"]
pub type Swdl1R = crate::BitReader;
#[doc = "Field `SWDL1` writer - SWDL1"]
pub type Swdl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSICL` reader - HSICL"]
pub type HsiclR = crate::BitReader;
#[doc = "Field `HSICL` writer - HSICL"]
pub type HsiclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDL0` reader - HSIDL0"]
pub type Hsidl0R = crate::BitReader;
#[doc = "Field `HSIDL0` writer - HSIDL0"]
pub type Hsidl0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDL1` reader - HSIDL1"]
pub type Hsidl1R = crate::BitReader;
#[doc = "Field `HSIDL1` writer - HSIDL1"]
pub type Hsidl1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTXSMCL` reader - FTXSMCL"]
pub type FtxsmclR = crate::BitReader;
#[doc = "Field `FTXSMCL` writer - FTXSMCL"]
pub type FtxsmclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTXSMDL` reader - FTXSMDL"]
pub type FtxsmdlR = crate::BitReader;
#[doc = "Field `FTXSMDL` writer - FTXSMDL"]
pub type FtxsmdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDOFFDL` reader - CDOFFDL"]
pub type CdoffdlR = crate::BitReader;
#[doc = "Field `CDOFFDL` writer - CDOFFDL"]
pub type CdoffdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDDL` reader - TDDL"]
pub type TddlR = crate::BitReader;
#[doc = "Field `TDDL` writer - TDDL"]
pub type TddlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDEN` reader - Pull-down enable"]
pub type PdenR = crate::BitReader;
#[doc = "Field `PDEN` writer - Pull-down enable"]
pub type PdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLKPREPEN` reader - Custom time for tCLK-PREPARE enable"]
pub type TclkprepenR = crate::BitReader;
#[doc = "Field `TCLKPREPEN` writer - Custom time for tCLK-PREPARE enable"]
pub type TclkprepenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLKZEROEN` reader - Custom time for tCLK-ZERO enable"]
pub type TclkzeroenR = crate::BitReader;
#[doc = "Field `TCLKZEROEN` writer - Custom time for tCLK-ZERO enable"]
pub type TclkzeroenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THSPREPEN` reader - Custom time for tHS-PREPARE enable"]
pub type ThsprepenR = crate::BitReader;
#[doc = "Field `THSPREPEN` writer - Custom time for tHS-PREPARE enable"]
pub type ThsprepenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THSTRAILEN` reader - Custom time for tHS-TRAIL enable"]
pub type ThstrailenR = crate::BitReader;
#[doc = "Field `THSTRAILEN` writer - Custom time for tHS-TRAIL enable"]
pub type ThstrailenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THSZEROEN` reader - Custom time for tHS-ZERO enable"]
pub type ThszeroenR = crate::BitReader;
#[doc = "Field `THSZEROEN` writer - Custom time for tHS-ZERO enable"]
pub type ThszeroenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLPXDEN` reader - Custom time for tLPX for data lanes enable"]
pub type TlpxdenR = crate::BitReader;
#[doc = "Field `TLPXDEN` writer - Custom time for tLPX for data lanes enable"]
pub type TlpxdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `THSEXITEN` reader - Custom time for tHS-EXIT enable"]
pub type ThsexitenR = crate::BitReader;
#[doc = "Field `THSEXITEN` writer - Custom time for tHS-EXIT enable"]
pub type ThsexitenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TLPXCEN` reader - Custom time for tLPX for clock lane enable"]
pub type TlpxcenR = crate::BitReader;
#[doc = "Field `TLPXCEN` writer - Custom time for tLPX for clock lane enable"]
pub type TlpxcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLKPOSTEN` reader - Custom time for tCLK-POST enable"]
pub type TclkpostenR = crate::BitReader;
#[doc = "Field `TCLKPOSTEN` writer - Custom time for tCLK-POST enable"]
pub type TclkpostenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&self) -> Uix4R {
        Uix4R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&self) -> SwclR {
        SwclR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&self) -> Swdl0R {
        Swdl0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&self) -> Swdl1R {
        Swdl1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&self) -> HsiclR {
        HsiclR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&self) -> Hsidl0R {
        Hsidl0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&self) -> Hsidl1R {
        Hsidl1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&self) -> FtxsmclR {
        FtxsmclR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&self) -> FtxsmdlR {
        FtxsmdlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&self) -> CdoffdlR {
        CdoffdlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&self) -> TddlR {
        TddlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Pull-down enable"]
    #[inline(always)]
    pub fn pden(&self) -> PdenR {
        PdenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Custom time for tCLK-PREPARE enable"]
    #[inline(always)]
    pub fn tclkprepen(&self) -> TclkprepenR {
        TclkprepenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Custom time for tCLK-ZERO enable"]
    #[inline(always)]
    pub fn tclkzeroen(&self) -> TclkzeroenR {
        TclkzeroenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Custom time for tHS-PREPARE enable"]
    #[inline(always)]
    pub fn thsprepen(&self) -> ThsprepenR {
        ThsprepenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Custom time for tHS-TRAIL enable"]
    #[inline(always)]
    pub fn thstrailen(&self) -> ThstrailenR {
        ThstrailenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Custom time for tHS-ZERO enable"]
    #[inline(always)]
    pub fn thszeroen(&self) -> ThszeroenR {
        ThszeroenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Custom time for tLPX for data lanes enable"]
    #[inline(always)]
    pub fn tlpxden(&self) -> TlpxdenR {
        TlpxdenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Custom time for tHS-EXIT enable"]
    #[inline(always)]
    pub fn thsexiten(&self) -> ThsexitenR {
        ThsexitenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Custom time for tLPX for clock lane enable"]
    #[inline(always)]
    pub fn tlpxcen(&self) -> TlpxcenR {
        TlpxcenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Custom time for tCLK-POST enable"]
    #[inline(always)]
    pub fn tclkposten(&self) -> TclkpostenR {
        TclkpostenR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - UIX4"]
    #[inline(always)]
    pub fn uix4(&mut self) -> Uix4W<DsiWpcr0Spec> {
        Uix4W::new(self, 0)
    }
    #[doc = "Bit 6 - SWCL"]
    #[inline(always)]
    pub fn swcl(&mut self) -> SwclW<DsiWpcr0Spec> {
        SwclW::new(self, 6)
    }
    #[doc = "Bit 7 - SWDL0"]
    #[inline(always)]
    pub fn swdl0(&mut self) -> Swdl0W<DsiWpcr0Spec> {
        Swdl0W::new(self, 7)
    }
    #[doc = "Bit 8 - SWDL1"]
    #[inline(always)]
    pub fn swdl1(&mut self) -> Swdl1W<DsiWpcr0Spec> {
        Swdl1W::new(self, 8)
    }
    #[doc = "Bit 9 - HSICL"]
    #[inline(always)]
    pub fn hsicl(&mut self) -> HsiclW<DsiWpcr0Spec> {
        HsiclW::new(self, 9)
    }
    #[doc = "Bit 10 - HSIDL0"]
    #[inline(always)]
    pub fn hsidl0(&mut self) -> Hsidl0W<DsiWpcr0Spec> {
        Hsidl0W::new(self, 10)
    }
    #[doc = "Bit 11 - HSIDL1"]
    #[inline(always)]
    pub fn hsidl1(&mut self) -> Hsidl1W<DsiWpcr0Spec> {
        Hsidl1W::new(self, 11)
    }
    #[doc = "Bit 12 - FTXSMCL"]
    #[inline(always)]
    pub fn ftxsmcl(&mut self) -> FtxsmclW<DsiWpcr0Spec> {
        FtxsmclW::new(self, 12)
    }
    #[doc = "Bit 13 - FTXSMDL"]
    #[inline(always)]
    pub fn ftxsmdl(&mut self) -> FtxsmdlW<DsiWpcr0Spec> {
        FtxsmdlW::new(self, 13)
    }
    #[doc = "Bit 14 - CDOFFDL"]
    #[inline(always)]
    pub fn cdoffdl(&mut self) -> CdoffdlW<DsiWpcr0Spec> {
        CdoffdlW::new(self, 14)
    }
    #[doc = "Bit 16 - TDDL"]
    #[inline(always)]
    pub fn tddl(&mut self) -> TddlW<DsiWpcr0Spec> {
        TddlW::new(self, 16)
    }
    #[doc = "Bit 18 - Pull-down enable"]
    #[inline(always)]
    pub fn pden(&mut self) -> PdenW<DsiWpcr0Spec> {
        PdenW::new(self, 18)
    }
    #[doc = "Bit 19 - Custom time for tCLK-PREPARE enable"]
    #[inline(always)]
    pub fn tclkprepen(&mut self) -> TclkprepenW<DsiWpcr0Spec> {
        TclkprepenW::new(self, 19)
    }
    #[doc = "Bit 20 - Custom time for tCLK-ZERO enable"]
    #[inline(always)]
    pub fn tclkzeroen(&mut self) -> TclkzeroenW<DsiWpcr0Spec> {
        TclkzeroenW::new(self, 20)
    }
    #[doc = "Bit 21 - Custom time for tHS-PREPARE enable"]
    #[inline(always)]
    pub fn thsprepen(&mut self) -> ThsprepenW<DsiWpcr0Spec> {
        ThsprepenW::new(self, 21)
    }
    #[doc = "Bit 22 - Custom time for tHS-TRAIL enable"]
    #[inline(always)]
    pub fn thstrailen(&mut self) -> ThstrailenW<DsiWpcr0Spec> {
        ThstrailenW::new(self, 22)
    }
    #[doc = "Bit 23 - Custom time for tHS-ZERO enable"]
    #[inline(always)]
    pub fn thszeroen(&mut self) -> ThszeroenW<DsiWpcr0Spec> {
        ThszeroenW::new(self, 23)
    }
    #[doc = "Bit 24 - Custom time for tLPX for data lanes enable"]
    #[inline(always)]
    pub fn tlpxden(&mut self) -> TlpxdenW<DsiWpcr0Spec> {
        TlpxdenW::new(self, 24)
    }
    #[doc = "Bit 25 - Custom time for tHS-EXIT enable"]
    #[inline(always)]
    pub fn thsexiten(&mut self) -> ThsexitenW<DsiWpcr0Spec> {
        ThsexitenW::new(self, 25)
    }
    #[doc = "Bit 26 - Custom time for tLPX for clock lane enable"]
    #[inline(always)]
    pub fn tlpxcen(&mut self) -> TlpxcenW<DsiWpcr0Spec> {
        TlpxcenW::new(self, 26)
    }
    #[doc = "Bit 27 - Custom time for tCLK-POST enable"]
    #[inline(always)]
    pub fn tclkposten(&mut self) -> TclkpostenW<DsiWpcr0Spec> {
        TclkpostenW::new(self, 27)
    }
}
#[doc = "DSI wrapper PHY configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWpcr0Spec;
impl crate::RegisterSpec for DsiWpcr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr0::R`](R) reader structure"]
impl crate::Readable for DsiWpcr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr0::W`](W) writer structure"]
impl crate::Writable for DsiWpcr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WPCR0 to value 0"]
impl crate::Resettable for DsiWpcr0Spec {}

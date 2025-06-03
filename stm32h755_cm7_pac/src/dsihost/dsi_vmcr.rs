#[doc = "Register `DSI_VMCR` reader"]
pub type R = crate::R<DsiVmcrSpec>;
#[doc = "Register `DSI_VMCR` writer"]
pub type W = crate::W<DsiVmcrSpec>;
#[doc = "Field `VMT` reader - VMT"]
pub type VmtR = crate::FieldReader;
#[doc = "Field `VMT` writer - VMT"]
pub type VmtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPVSAE` reader - LPVSAE"]
pub type LpvsaeR = crate::BitReader;
#[doc = "Field `LPVSAE` writer - LPVSAE"]
pub type LpvsaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVBPE` reader - LPVBPE"]
pub type LpvbpeR = crate::BitReader;
#[doc = "Field `LPVBPE` writer - LPVBPE"]
pub type LpvbpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVFPE` reader - LPVFPE"]
pub type LpvfpeR = crate::BitReader;
#[doc = "Field `LPVFPE` writer - LPVFPE"]
pub type LpvfpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPVAE` reader - LPVAE"]
pub type LpvaeR = crate::BitReader;
#[doc = "Field `LPVAE` writer - LPVAE"]
pub type LpvaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPHBPE` reader - LPHBPE"]
pub type LphbpeR = crate::BitReader;
#[doc = "Field `LPHBPE` writer - LPHBPE"]
pub type LphbpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPHFPE` reader - LPHFPE"]
pub type LphfpeR = crate::BitReader;
#[doc = "Field `LPHFPE` writer - LPHFPE"]
pub type LphfpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBTAAE` reader - FBTAAE"]
pub type FbtaaeR = crate::BitReader;
#[doc = "Field `FBTAAE` writer - FBTAAE"]
pub type FbtaaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCE` reader - LPCE"]
pub type LpceR = crate::BitReader;
#[doc = "Field `LPCE` writer - LPCE"]
pub type LpceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGE` reader - PGE"]
pub type PgeR = crate::BitReader;
#[doc = "Field `PGE` writer - PGE"]
pub type PgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGM` reader - PGM"]
pub type PgmR = crate::BitReader;
#[doc = "Field `PGM` writer - PGM"]
pub type PgmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGO` reader - PGO"]
pub type PgoR = crate::BitReader;
#[doc = "Field `PGO` writer - PGO"]
pub type PgoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - VMT"]
    #[inline(always)]
    pub fn vmt(&self) -> VmtR {
        VmtR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 8 - LPVSAE"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LpvsaeR {
        LpvsaeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPVBPE"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LpvbpeR {
        LpvbpeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPVFPE"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LpvfpeR {
        LpvfpeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPVAE"]
    #[inline(always)]
    pub fn lpvae(&self) -> LpvaeR {
        LpvaeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPHBPE"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LphbpeR {
        LphbpeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - LPHFPE"]
    #[inline(always)]
    pub fn lphfpe(&self) -> LphfpeR {
        LphfpeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FBTAAE"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FbtaaeR {
        FbtaaeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - LPCE"]
    #[inline(always)]
    pub fn lpce(&self) -> LpceR {
        LpceR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PGE"]
    #[inline(always)]
    pub fn pge(&self) -> PgeR {
        PgeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - PGM"]
    #[inline(always)]
    pub fn pgm(&self) -> PgmR {
        PgmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - PGO"]
    #[inline(always)]
    pub fn pgo(&self) -> PgoR {
        PgoR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - VMT"]
    #[inline(always)]
    pub fn vmt(&mut self) -> VmtW<DsiVmcrSpec> {
        VmtW::new(self, 0)
    }
    #[doc = "Bit 8 - LPVSAE"]
    #[inline(always)]
    pub fn lpvsae(&mut self) -> LpvsaeW<DsiVmcrSpec> {
        LpvsaeW::new(self, 8)
    }
    #[doc = "Bit 9 - LPVBPE"]
    #[inline(always)]
    pub fn lpvbpe(&mut self) -> LpvbpeW<DsiVmcrSpec> {
        LpvbpeW::new(self, 9)
    }
    #[doc = "Bit 10 - LPVFPE"]
    #[inline(always)]
    pub fn lpvfpe(&mut self) -> LpvfpeW<DsiVmcrSpec> {
        LpvfpeW::new(self, 10)
    }
    #[doc = "Bit 11 - LPVAE"]
    #[inline(always)]
    pub fn lpvae(&mut self) -> LpvaeW<DsiVmcrSpec> {
        LpvaeW::new(self, 11)
    }
    #[doc = "Bit 12 - LPHBPE"]
    #[inline(always)]
    pub fn lphbpe(&mut self) -> LphbpeW<DsiVmcrSpec> {
        LphbpeW::new(self, 12)
    }
    #[doc = "Bit 13 - LPHFPE"]
    #[inline(always)]
    pub fn lphfpe(&mut self) -> LphfpeW<DsiVmcrSpec> {
        LphfpeW::new(self, 13)
    }
    #[doc = "Bit 14 - FBTAAE"]
    #[inline(always)]
    pub fn fbtaae(&mut self) -> FbtaaeW<DsiVmcrSpec> {
        FbtaaeW::new(self, 14)
    }
    #[doc = "Bit 15 - LPCE"]
    #[inline(always)]
    pub fn lpce(&mut self) -> LpceW<DsiVmcrSpec> {
        LpceW::new(self, 15)
    }
    #[doc = "Bit 16 - PGE"]
    #[inline(always)]
    pub fn pge(&mut self) -> PgeW<DsiVmcrSpec> {
        PgeW::new(self, 16)
    }
    #[doc = "Bit 20 - PGM"]
    #[inline(always)]
    pub fn pgm(&mut self) -> PgmW<DsiVmcrSpec> {
        PgmW::new(self, 20)
    }
    #[doc = "Bit 24 - PGO"]
    #[inline(always)]
    pub fn pgo(&mut self) -> PgoW<DsiVmcrSpec> {
        PgoW::new(self, 24)
    }
}
#[doc = "DSI Host video mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVmcrSpec;
impl crate::RegisterSpec for DsiVmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vmcr::R`](R) reader structure"]
impl crate::Readable for DsiVmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vmcr::W`](W) writer structure"]
impl crate::Writable for DsiVmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VMCR to value 0"]
impl crate::Resettable for DsiVmcrSpec {}

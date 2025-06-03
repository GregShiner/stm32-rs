#[doc = "Register `DSI_VMCCR` reader"]
pub type R = crate::R<DsiVmccrSpec>;
#[doc = "Field `VMT` reader - VMT"]
pub type VmtR = crate::FieldReader;
#[doc = "Field `LPVSAE` reader - LPVSAE"]
pub type LpvsaeR = crate::BitReader;
#[doc = "Field `LPVBPE` reader - LPVBPE"]
pub type LpvbpeR = crate::BitReader;
#[doc = "Field `LPVFPE` reader - LPVFPE"]
pub type LpvfpeR = crate::BitReader;
#[doc = "Field `LPVAE` reader - LPVAE"]
pub type LpvaeR = crate::BitReader;
#[doc = "Field `LPHBPE` reader - LPHBPE"]
pub type LphbpeR = crate::BitReader;
#[doc = "Field `LPHFE` reader - LPHFE"]
pub type LphfeR = crate::BitReader;
#[doc = "Field `FBTAAE` reader - FBTAAE"]
pub type FbtaaeR = crate::BitReader;
#[doc = "Field `LPCE` reader - LPCE"]
pub type LpceR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - VMT"]
    #[inline(always)]
    pub fn vmt(&self) -> VmtR {
        VmtR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - LPVSAE"]
    #[inline(always)]
    pub fn lpvsae(&self) -> LpvsaeR {
        LpvsaeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPVBPE"]
    #[inline(always)]
    pub fn lpvbpe(&self) -> LpvbpeR {
        LpvbpeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPVFPE"]
    #[inline(always)]
    pub fn lpvfpe(&self) -> LpvfpeR {
        LpvfpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LPVAE"]
    #[inline(always)]
    pub fn lpvae(&self) -> LpvaeR {
        LpvaeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LPHBPE"]
    #[inline(always)]
    pub fn lphbpe(&self) -> LphbpeR {
        LphbpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPHFE"]
    #[inline(always)]
    pub fn lphfe(&self) -> LphfeR {
        LphfeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FBTAAE"]
    #[inline(always)]
    pub fn fbtaae(&self) -> FbtaaeR {
        FbtaaeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPCE"]
    #[inline(always)]
    pub fn lpce(&self) -> LpceR {
        LpceR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "DSI Host video mode current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vmccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVmccrSpec;
impl crate::RegisterSpec for DsiVmccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vmccr::R`](R) reader structure"]
impl crate::Readable for DsiVmccrSpec {}
#[doc = "`reset()` method sets DSI_VMCCR to value 0"]
impl crate::Resettable for DsiVmccrSpec {}

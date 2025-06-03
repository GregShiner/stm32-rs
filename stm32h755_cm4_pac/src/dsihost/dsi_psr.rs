#[doc = "Register `DSI_PSR` reader"]
pub type R = crate::R<DsiPsrSpec>;
#[doc = "Field `PD` reader - PD"]
pub type PdR = crate::BitReader;
#[doc = "Field `PSSC` reader - PSSC"]
pub type PsscR = crate::BitReader;
#[doc = "Field `UANC` reader - UANC"]
pub type UancR = crate::BitReader;
#[doc = "Field `PSS0` reader - PSS0"]
pub type Pss0R = crate::BitReader;
#[doc = "Field `UAN0` reader - UAN0"]
pub type Uan0R = crate::BitReader;
#[doc = "Field `RUE0` reader - RUE0"]
pub type Rue0R = crate::BitReader;
#[doc = "Field `PSS1` reader - PSS1"]
pub type Pss1R = crate::BitReader;
#[doc = "Field `UAN1` reader - UAN1"]
pub type Uan1R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - PD"]
    #[inline(always)]
    pub fn pd(&self) -> PdR {
        PdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PSSC"]
    #[inline(always)]
    pub fn pssc(&self) -> PsscR {
        PsscR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UANC"]
    #[inline(always)]
    pub fn uanc(&self) -> UancR {
        UancR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PSS0"]
    #[inline(always)]
    pub fn pss0(&self) -> Pss0R {
        Pss0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - UAN0"]
    #[inline(always)]
    pub fn uan0(&self) -> Uan0R {
        Uan0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RUE0"]
    #[inline(always)]
    pub fn rue0(&self) -> Rue0R {
        Rue0R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PSS1"]
    #[inline(always)]
    pub fn pss1(&self) -> Pss1R {
        Pss1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - UAN1"]
    #[inline(always)]
    pub fn uan1(&self) -> Uan1R {
        Uan1R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "DSI Host PHY status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_psr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiPsrSpec;
impl crate::RegisterSpec for DsiPsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_psr::R`](R) reader structure"]
impl crate::Readable for DsiPsrSpec {}
#[doc = "`reset()` method sets DSI_PSR to value 0x1528"]
impl crate::Resettable for DsiPsrSpec {
    const RESET_VALUE: u32 = 0x1528;
}

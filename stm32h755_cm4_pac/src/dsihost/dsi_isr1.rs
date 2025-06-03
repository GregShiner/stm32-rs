#[doc = "Register `DSI_ISR1` reader"]
pub type R = crate::R<DsiIsr1Spec>;
#[doc = "Field `TOHSTX` reader - TOHSTX"]
pub type TohstxR = crate::BitReader;
#[doc = "Field `TOLPRX` reader - TOLPRX"]
pub type TolprxR = crate::BitReader;
#[doc = "Field `ECCSE` reader - ECCSE"]
pub type EccseR = crate::BitReader;
#[doc = "Field `ECCME` reader - ECCME"]
pub type EccmeR = crate::BitReader;
#[doc = "Field `CRCE` reader - CRCE"]
pub type CrceR = crate::BitReader;
#[doc = "Field `PSE` reader - PSE"]
pub type PseR = crate::BitReader;
#[doc = "Field `EOTPE` reader - EOTPE"]
pub type EotpeR = crate::BitReader;
#[doc = "Field `LPWRE` reader - LPWRE"]
pub type LpwreR = crate::BitReader;
#[doc = "Field `GCWRE` reader - GCWRE"]
pub type GcwreR = crate::BitReader;
#[doc = "Field `GPWRE` reader - GPWRE"]
pub type GpwreR = crate::BitReader;
#[doc = "Field `GPTXE` reader - GPTXE"]
pub type GptxeR = crate::BitReader;
#[doc = "Field `GPRDE` reader - GPRDE"]
pub type GprdeR = crate::BitReader;
#[doc = "Field `GPRXE` reader - GPRXE"]
pub type GprxeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TOHSTX"]
    #[inline(always)]
    pub fn tohstx(&self) -> TohstxR {
        TohstxR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOLPRX"]
    #[inline(always)]
    pub fn tolprx(&self) -> TolprxR {
        TolprxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECCSE"]
    #[inline(always)]
    pub fn eccse(&self) -> EccseR {
        EccseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCME"]
    #[inline(always)]
    pub fn eccme(&self) -> EccmeR {
        EccmeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCE"]
    #[inline(always)]
    pub fn crce(&self) -> CrceR {
        CrceR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSE"]
    #[inline(always)]
    pub fn pse(&self) -> PseR {
        PseR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOTPE"]
    #[inline(always)]
    pub fn eotpe(&self) -> EotpeR {
        EotpeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPWRE"]
    #[inline(always)]
    pub fn lpwre(&self) -> LpwreR {
        LpwreR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GCWRE"]
    #[inline(always)]
    pub fn gcwre(&self) -> GcwreR {
        GcwreR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPWRE"]
    #[inline(always)]
    pub fn gpwre(&self) -> GpwreR {
        GpwreR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTXE"]
    #[inline(always)]
    pub fn gptxe(&self) -> GptxeR {
        GptxeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPRDE"]
    #[inline(always)]
    pub fn gprde(&self) -> GprdeR {
        GprdeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPRXE"]
    #[inline(always)]
    pub fn gprxe(&self) -> GprxeR {
        GprxeR::new(((self.bits >> 12) & 1) != 0)
    }
}
#[doc = "DSI Host interrupt and status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_isr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiIsr1Spec;
impl crate::RegisterSpec for DsiIsr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_isr1::R`](R) reader structure"]
impl crate::Readable for DsiIsr1Spec {}
#[doc = "`reset()` method sets DSI_ISR1 to value 0"]
impl crate::Resettable for DsiIsr1Spec {}

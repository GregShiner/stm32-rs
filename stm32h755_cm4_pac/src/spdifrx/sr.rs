#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `RXNE` reader - Read data register not empty"]
pub type RxneR = crate::BitReader;
#[doc = "Field `CSRNE` reader - Control Buffer register is not empty"]
pub type CsrneR = crate::BitReader;
#[doc = "Field `PERR` reader - Parity error"]
pub type PerrR = crate::BitReader;
#[doc = "Field `OVR` reader - Overrun error"]
pub type OvrR = crate::BitReader;
#[doc = "Field `SBD` reader - Synchronization Block Detected"]
pub type SbdR = crate::BitReader;
#[doc = "Field `SYNCD` reader - Synchronization Done"]
pub type SyncdR = crate::BitReader;
#[doc = "Field `FERR` reader - Framing error"]
pub type FerrR = crate::BitReader;
#[doc = "Field `SERR` reader - Synchronization error"]
pub type SerrR = crate::BitReader;
#[doc = "Field `TERR` reader - Time-out error"]
pub type TerrR = crate::BitReader;
#[doc = "Field `WIDTH5` reader - Duration of 5 symbols counted with SPDIF_CLK"]
pub type Width5R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - Read data register not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RxneR {
        RxneR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Buffer register is not empty"]
    #[inline(always)]
    pub fn csrne(&self) -> CsrneR {
        CsrneR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error"]
    #[inline(always)]
    pub fn perr(&self) -> PerrR {
        PerrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization Block Detected"]
    #[inline(always)]
    pub fn sbd(&self) -> SbdR {
        SbdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    pub fn syncd(&self) -> SyncdR {
        SyncdR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Framing error"]
    #[inline(always)]
    pub fn ferr(&self) -> FerrR {
        FerrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Synchronization error"]
    #[inline(always)]
    pub fn serr(&self) -> SerrR {
        SerrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Time-out error"]
    #[inline(always)]
    pub fn terr(&self) -> TerrR {
        TerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:30 - Duration of 5 symbols counted with SPDIF_CLK"]
    #[inline(always)]
    pub fn width5(&self) -> Width5R {
        Width5R::new(((self.bits >> 16) & 0x7fff) as u16)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}

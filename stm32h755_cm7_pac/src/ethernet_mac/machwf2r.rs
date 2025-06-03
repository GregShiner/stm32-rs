#[doc = "Register `MACHWF2R` reader"]
pub type R = crate::R<Machwf2rSpec>;
#[doc = "Field `RXQCNT` reader - RXQCNT"]
pub type RxqcntR = crate::FieldReader;
#[doc = "Field `TXQCNT` reader - TXQCNT"]
pub type TxqcntR = crate::FieldReader;
#[doc = "Field `RXCHCNT` reader - RXCHCNT"]
pub type RxchcntR = crate::FieldReader;
#[doc = "Field `TXCHCNT` reader - TXCHCNT"]
pub type TxchcntR = crate::FieldReader;
#[doc = "Field `PPSOUTNUM` reader - PPSOUTNUM"]
pub type PpsoutnumR = crate::FieldReader;
#[doc = "Field `AUXSNAPNUM` reader - AUXSNAPNUM"]
pub type AuxsnapnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - RXQCNT"]
    #[inline(always)]
    pub fn rxqcnt(&self) -> RxqcntR {
        RxqcntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - TXQCNT"]
    #[inline(always)]
    pub fn txqcnt(&self) -> TxqcntR {
        TxqcntR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RXCHCNT"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RxchcntR {
        RxchcntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - TXCHCNT"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TxchcntR {
        TxchcntR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - PPSOUTNUM"]
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PpsoutnumR {
        PpsoutnumR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - AUXSNAPNUM"]
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AuxsnapnumR {
        AuxsnapnumR::new(((self.bits >> 28) & 7) as u8)
    }
}
#[doc = "HW feature 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`machwf2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Machwf2rSpec;
impl crate::RegisterSpec for Machwf2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machwf2r::R`](R) reader structure"]
impl crate::Readable for Machwf2rSpec {}
#[doc = "`reset()` method sets MACHWF2R to value 0x4100_0000"]
impl crate::Resettable for Machwf2rSpec {
    const RESET_VALUE: u32 = 0x4100_0000;
}

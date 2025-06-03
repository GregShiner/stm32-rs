#[doc = "Register `MTLTxQDR` reader"]
pub type R = crate::R<MtltxQdrSpec>;
#[doc = "Field `TXQPAUSED` reader - TXQPAUSED"]
pub type TxqpausedR = crate::BitReader;
#[doc = "Field `TRCSTS` reader - TRCSTS"]
pub type TrcstsR = crate::FieldReader;
#[doc = "Field `TWCSTS` reader - TWCSTS"]
pub type TwcstsR = crate::BitReader;
#[doc = "Field `TXQSTS` reader - TXQSTS"]
pub type TxqstsR = crate::BitReader;
#[doc = "Field `TXSTSFSTS` reader - TXSTSFSTS"]
pub type TxstsfstsR = crate::BitReader;
#[doc = "Field `PTXQ` reader - PTXQ"]
pub type PtxqR = crate::FieldReader;
#[doc = "Field `STXSTSF` reader - STXSTSF"]
pub type StxstsfR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - TXQPAUSED"]
    #[inline(always)]
    pub fn txqpaused(&self) -> TxqpausedR {
        TxqpausedR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - TRCSTS"]
    #[inline(always)]
    pub fn trcsts(&self) -> TrcstsR {
        TrcstsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - TWCSTS"]
    #[inline(always)]
    pub fn twcsts(&self) -> TwcstsR {
        TwcstsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXQSTS"]
    #[inline(always)]
    pub fn txqsts(&self) -> TxqstsR {
        TxqstsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TXSTSFSTS"]
    #[inline(always)]
    pub fn txstsfsts(&self) -> TxstsfstsR {
        TxstsfstsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - PTXQ"]
    #[inline(always)]
    pub fn ptxq(&self) -> PtxqR {
        PtxqR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - STXSTSF"]
    #[inline(always)]
    pub fn stxstsf(&self) -> StxstsfR {
        StxstsfR::new(((self.bits >> 20) & 7) as u8)
    }
}
#[doc = "Tx queue debug Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtltx_qdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtltxQdrSpec;
impl crate::RegisterSpec for MtltxQdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtltx_qdr::R`](R) reader structure"]
impl crate::Readable for MtltxQdrSpec {}
#[doc = "`reset()` method sets MTLTxQDR to value 0"]
impl crate::Resettable for MtltxQdrSpec {}

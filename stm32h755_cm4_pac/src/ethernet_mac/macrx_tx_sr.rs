#[doc = "Register `MACRxTxSR` reader"]
pub type R = crate::R<MacrxTxSrSpec>;
#[doc = "Field `TJT` reader - TJT"]
pub type TjtR = crate::BitReader;
#[doc = "Field `NCARR` reader - NCARR"]
pub type NcarrR = crate::BitReader;
#[doc = "Field `LCARR` reader - LCARR"]
pub type LcarrR = crate::BitReader;
#[doc = "Field `EXDEF` reader - EXDEF"]
pub type ExdefR = crate::BitReader;
#[doc = "Field `LCOL` reader - LCOL"]
pub type LcolR = crate::BitReader;
#[doc = "Field `EXCOL` reader - LCOL"]
pub type ExcolR = crate::BitReader;
#[doc = "Field `RWT` reader - RWT"]
pub type RwtR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TJT"]
    #[inline(always)]
    pub fn tjt(&self) -> TjtR {
        TjtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NCARR"]
    #[inline(always)]
    pub fn ncarr(&self) -> NcarrR {
        NcarrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCARR"]
    #[inline(always)]
    pub fn lcarr(&self) -> LcarrR {
        LcarrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - EXDEF"]
    #[inline(always)]
    pub fn exdef(&self) -> ExdefR {
        ExdefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LCOL"]
    #[inline(always)]
    pub fn lcol(&self) -> LcolR {
        LcolR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - LCOL"]
    #[inline(always)]
    pub fn excol(&self) -> ExcolR {
        ExcolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - RWT"]
    #[inline(always)]
    pub fn rwt(&self) -> RwtR {
        RwtR::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "Rx Tx status register\n\nYou can [`read`](crate::Reg::read) this register and get [`macrx_tx_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacrxTxSrSpec;
impl crate::RegisterSpec for MacrxTxSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrx_tx_sr::R`](R) reader structure"]
impl crate::Readable for MacrxTxSrSpec {}
#[doc = "`reset()` method sets MACRxTxSR to value 0"]
impl crate::Resettable for MacrxTxSrSpec {}

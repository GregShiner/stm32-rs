#[doc = "Register `MACTxTSSNR` reader"]
pub type R = crate::R<MactxTssnrSpec>;
#[doc = "Field `TXTSSLO` reader - TXTSSLO"]
pub type TxtssloR = crate::FieldReader<u32>;
#[doc = "Field `TXTSSMIS` reader - TXTSSMIS"]
pub type TxtssmisR = crate::BitReader;
impl R {
    #[doc = "Bits 0:30 - TXTSSLO"]
    #[inline(always)]
    pub fn txtsslo(&self) -> TxtssloR {
        TxtssloR::new(self.bits & 0x7fff_ffff)
    }
    #[doc = "Bit 31 - TXTSSMIS"]
    #[inline(always)]
    pub fn txtssmis(&self) -> TxtssmisR {
        TxtssmisR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Tx timestamp status nanoseconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactx_tssnr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactxTssnrSpec;
impl crate::RegisterSpec for MactxTssnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactx_tssnr::R`](R) reader structure"]
impl crate::Readable for MactxTssnrSpec {}
#[doc = "`reset()` method sets MACTxTSSNR to value 0"]
impl crate::Resettable for MactxTssnrSpec {}

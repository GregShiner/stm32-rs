#[doc = "Register `MACTxTSSSR` reader"]
pub type R = crate::R<MactxTsssrSpec>;
#[doc = "Field `TXTSSHI` reader - TXTSSHI"]
pub type TxtsshiR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXTSSHI"]
    #[inline(always)]
    pub fn txtsshi(&self) -> TxtsshiR {
        TxtsshiR::new(self.bits)
    }
}
#[doc = "Tx timestamp status seconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactx_tsssr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactxTsssrSpec;
impl crate::RegisterSpec for MactxTsssrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactx_tsssr::R`](R) reader structure"]
impl crate::Readable for MactxTsssrSpec {}
#[doc = "`reset()` method sets MACTxTSSSR to value 0"]
impl crate::Resettable for MactxTsssrSpec {}

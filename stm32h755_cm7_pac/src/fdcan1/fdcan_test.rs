#[doc = "Register `FDCAN_TEST` reader"]
pub type R = crate::R<FdcanTestSpec>;
#[doc = "Field `LBCK` reader - Loop Back mode"]
pub type LbckR = crate::BitReader;
#[doc = "Field `TX` reader - Loop Back mode"]
pub type TxR = crate::FieldReader;
#[doc = "Field `RX` reader - Control of Transmit Pin"]
pub type RxR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Loop Back mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Loop Back mode"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Control of Transmit Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "FDCAN Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_test::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTestSpec;
impl crate::RegisterSpec for FdcanTestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_test::R`](R) reader structure"]
impl crate::Readable for FdcanTestSpec {}
#[doc = "`reset()` method sets FDCAN_TEST to value 0"]
impl crate::Resettable for FdcanTestSpec {}

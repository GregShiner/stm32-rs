#[doc = "Register `FDCAN_DBTP` reader"]
pub type R = crate::R<FdcanDbtpSpec>;
#[doc = "Field `DSJW` reader - Synchronization Jump Width"]
pub type DsjwR = crate::FieldReader;
#[doc = "Field `DTSEG2` reader - Data time segment after sample point"]
pub type Dtseg2R = crate::FieldReader;
#[doc = "Field `DTSEG1` reader - Data time segment after sample point"]
pub type Dtseg1R = crate::FieldReader;
#[doc = "Field `DBRP` reader - Data BIt Rate Prescaler"]
pub type DbrpR = crate::FieldReader;
#[doc = "Field `TDC` reader - Transceiver Delay Compensation"]
pub type TdcR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Synchronization Jump Width"]
    #[inline(always)]
    pub fn dsjw(&self) -> DsjwR {
        DsjwR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg2(&self) -> Dtseg2R {
        Dtseg2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:12 - Data time segment after sample point"]
    #[inline(always)]
    pub fn dtseg1(&self) -> Dtseg1R {
        Dtseg1R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Data BIt Rate Prescaler"]
    #[inline(always)]
    pub fn dbrp(&self) -> DbrpR {
        DbrpR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 23 - Transceiver Delay Compensation"]
    #[inline(always)]
    pub fn tdc(&self) -> TdcR {
        TdcR::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "FDCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_dbtp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanDbtpSpec;
impl crate::RegisterSpec for FdcanDbtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_dbtp::R`](R) reader structure"]
impl crate::Readable for FdcanDbtpSpec {}
#[doc = "`reset()` method sets FDCAN_DBTP to value 0"]
impl crate::Resettable for FdcanDbtpSpec {}

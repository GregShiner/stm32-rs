#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `IFEM` reader - Input FIFO empty"]
pub type IfemR = crate::BitReader;
#[doc = "Field `IFNF` reader - Input FIFO not full"]
pub type IfnfR = crate::BitReader;
#[doc = "Field `OFNE` reader - Output FIFO not empty"]
pub type OfneR = crate::BitReader;
#[doc = "Field `OFFU` reader - Output FIFO full"]
pub type OffuR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy bit"]
pub type BusyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input FIFO empty"]
    #[inline(always)]
    pub fn ifem(&self) -> IfemR {
        IfemR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input FIFO not full"]
    #[inline(always)]
    pub fn ifnf(&self) -> IfnfR {
        IfnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output FIFO not empty"]
    #[inline(always)]
    pub fn ofne(&self) -> OfneR {
        OfneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO full"]
    #[inline(always)]
    pub fn offu(&self) -> OffuR {
        OffuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy bit"]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x03;
}

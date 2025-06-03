#[doc = "Register `FDCAN_HPMS` reader"]
pub type R = crate::R<FdcanHpmsSpec>;
#[doc = "Field `BIDX` reader - Buffer Index"]
pub type BidxR = crate::FieldReader;
#[doc = "Field `MSI` reader - Message Storage Indicator"]
pub type MsiR = crate::FieldReader;
#[doc = "Field `FIDX` reader - Filter Index"]
pub type FidxR = crate::FieldReader;
#[doc = "Field `FLST` reader - Filter List"]
pub type FlstR = crate::BitReader;
impl R {
    #[doc = "Bits 0:5 - Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BidxR {
        BidxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Message Storage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FlstR {
        FlstR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "FDCAN High Priority Message Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_hpms::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanHpmsSpec;
impl crate::RegisterSpec for FdcanHpmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_hpms::R`](R) reader structure"]
impl crate::Readable for FdcanHpmsSpec {}
#[doc = "`reset()` method sets FDCAN_HPMS to value 0"]
impl crate::Resettable for FdcanHpmsSpec {}

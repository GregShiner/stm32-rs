#[doc = "Register `DMADSR` reader"]
pub type R = crate::R<DmadsrSpec>;
#[doc = "Field `AXWHSTS` reader - AHB Master Write Channel"]
pub type AxwhstsR = crate::BitReader;
#[doc = "Field `RPS0` reader - DMA Channel Receive Process State"]
pub type Rps0R = crate::FieldReader;
#[doc = "Field `TPS0` reader - DMA Channel Transmit Process State"]
pub type Tps0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - AHB Master Write Channel"]
    #[inline(always)]
    pub fn axwhsts(&self) -> AxwhstsR {
        AxwhstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:11 - DMA Channel Receive Process State"]
    #[inline(always)]
    pub fn rps0(&self) -> Rps0R {
        Rps0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA Channel Transmit Process State"]
    #[inline(always)]
    pub fn tps0(&self) -> Tps0R {
        Tps0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[doc = "Debug status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmadsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmadsrSpec;
impl crate::RegisterSpec for DmadsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmadsr::R`](R) reader structure"]
impl crate::Readable for DmadsrSpec {}
#[doc = "`reset()` method sets DMADSR to value 0"]
impl crate::Resettable for DmadsrSpec {}

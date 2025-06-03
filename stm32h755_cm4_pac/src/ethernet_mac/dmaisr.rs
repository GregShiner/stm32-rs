#[doc = "Register `DMAISR` reader"]
pub type R = crate::R<DmaisrSpec>;
#[doc = "Field `DC0IS` reader - DMA Channel Interrupt Status"]
pub type Dc0isR = crate::BitReader;
#[doc = "Field `MTLIS` reader - MTL Interrupt Status"]
pub type MtlisR = crate::BitReader;
#[doc = "Field `MACIS` reader - MAC Interrupt Status"]
pub type MacisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA Channel Interrupt Status"]
    #[inline(always)]
    pub fn dc0is(&self) -> Dc0isR {
        Dc0isR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Status"]
    #[inline(always)]
    pub fn mtlis(&self) -> MtlisR {
        MtlisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MAC Interrupt Status"]
    #[inline(always)]
    pub fn macis(&self) -> MacisR {
        MacisR::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaisrSpec;
impl crate::RegisterSpec for DmaisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaisr::R`](R) reader structure"]
impl crate::Readable for DmaisrSpec {}
#[doc = "`reset()` method sets DMAISR to value 0"]
impl crate::Resettable for DmaisrSpec {}

#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Field `TEIF` reader - Transfer error interrupt flag This bit is set when an error occurs during a DMA transfer (data transfer or automatic CLUT loading)."]
pub type TeifR = crate::BitReader;
#[doc = "Field `TCIF` reader - Transfer complete interrupt flag This bit is set when a DMA2D transfer operation is complete (data transfer only)."]
pub type TcifR = crate::BitReader;
#[doc = "Field `TWIF` reader - Transfer watermark interrupt flag This bit is set when the last pixel of the watermarked line has been transferred."]
pub type TwifR = crate::BitReader;
#[doc = "Field `CAEIF` reader - CLUT access error interrupt flag This bit is set when the CPU accesses the CLUT while the CLUT is being automatically copied from a system memory to the internal DMA2D."]
pub type CaeifR = crate::BitReader;
#[doc = "Field `CTCIF` reader - CLUT transfer complete interrupt flag This bit is set when the CLUT copy from a system memory area to the internal DMA2D memory is complete."]
pub type CtcifR = crate::BitReader;
#[doc = "Field `CEIF` reader - Configuration error interrupt flag This bit is set when the START bit of DMA2D_CR, DMA2DFGPFCCR or DMA2D_BGPFCCR is set and a wrong configuration has been programmed."]
pub type CeifR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transfer error interrupt flag This bit is set when an error occurs during a DMA transfer (data transfer or automatic CLUT loading)."]
    #[inline(always)]
    pub fn teif(&self) -> TeifR {
        TeifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt flag This bit is set when a DMA2D transfer operation is complete (data transfer only)."]
    #[inline(always)]
    pub fn tcif(&self) -> TcifR {
        TcifR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer watermark interrupt flag This bit is set when the last pixel of the watermarked line has been transferred."]
    #[inline(always)]
    pub fn twif(&self) -> TwifR {
        TwifR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CLUT access error interrupt flag This bit is set when the CPU accesses the CLUT while the CLUT is being automatically copied from a system memory to the internal DMA2D."]
    #[inline(always)]
    pub fn caeif(&self) -> CaeifR {
        CaeifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CLUT transfer complete interrupt flag This bit is set when the CLUT copy from a system memory area to the internal DMA2D memory is complete."]
    #[inline(always)]
    pub fn ctcif(&self) -> CtcifR {
        CtcifR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configuration error interrupt flag This bit is set when the START bit of DMA2D_CR, DMA2DFGPFCCR or DMA2D_BGPFCCR is set and a wrong configuration has been programmed."]
    #[inline(always)]
    pub fn ceif(&self) -> CeifR {
        CeifR::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "DMA2D Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}

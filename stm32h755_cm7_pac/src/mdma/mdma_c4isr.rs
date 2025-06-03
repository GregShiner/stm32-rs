#[doc = "Register `MDMA_C4ISR` reader"]
pub type R = crate::R<MdmaC4isrSpec>;
#[doc = "Field `TEIF4` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif4R = crate::BitReader;
#[doc = "Field `CTCIF4` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif4R = crate::BitReader;
#[doc = "Field `BRTIF4` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif4R = crate::BitReader;
#[doc = "Field `BTIF4` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif4R = crate::BitReader;
#[doc = "Field `TCIF4` reader - channel x buffer transfer complete"]
pub type Tcif4R = crate::BitReader;
#[doc = "Field `CRQA4` reader - channel x request active flag"]
pub type Crqa4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif4(&self) -> Teif4R {
        Teif4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif4(&self) -> Ctcif4R {
        Ctcif4R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif4(&self) -> Brtif4R {
        Brtif4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif4(&self) -> Btif4R {
        Btif4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif4(&self) -> Tcif4R {
        Tcif4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa4(&self) -> Crqa4R {
        Crqa4R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c4isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC4isrSpec;
impl crate::RegisterSpec for MdmaC4isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c4isr::R`](R) reader structure"]
impl crate::Readable for MdmaC4isrSpec {}
#[doc = "`reset()` method sets MDMA_C4ISR to value 0"]
impl crate::Resettable for MdmaC4isrSpec {}

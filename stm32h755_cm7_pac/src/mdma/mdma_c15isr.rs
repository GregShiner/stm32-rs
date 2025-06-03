#[doc = "Register `MDMA_C15ISR` reader"]
pub type R = crate::R<MdmaC15isrSpec>;
#[doc = "Field `TEIF15` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif15R = crate::BitReader;
#[doc = "Field `CTCIF15` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif15R = crate::BitReader;
#[doc = "Field `BRTIF15` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif15R = crate::BitReader;
#[doc = "Field `BTIF15` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif15R = crate::BitReader;
#[doc = "Field `TCIF15` reader - channel x buffer transfer complete"]
pub type Tcif15R = crate::BitReader;
#[doc = "Field `CRQA15` reader - channel x request active flag"]
pub type Crqa15R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif15(&self) -> Teif15R {
        Teif15R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif15(&self) -> Ctcif15R {
        Ctcif15R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif15(&self) -> Brtif15R {
        Brtif15R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif15(&self) -> Btif15R {
        Btif15R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif15(&self) -> Tcif15R {
        Tcif15R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa15(&self) -> Crqa15R {
        Crqa15R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC15isrSpec;
impl crate::RegisterSpec for MdmaC15isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c15isr::R`](R) reader structure"]
impl crate::Readable for MdmaC15isrSpec {}
#[doc = "`reset()` method sets MDMA_C15ISR to value 0"]
impl crate::Resettable for MdmaC15isrSpec {}

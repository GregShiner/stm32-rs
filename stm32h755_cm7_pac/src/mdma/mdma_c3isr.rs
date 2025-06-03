#[doc = "Register `MDMA_C3ISR` reader"]
pub type R = crate::R<MdmaC3isrSpec>;
#[doc = "Field `TEIF3` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif3R = crate::BitReader;
#[doc = "Field `CTCIF3` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif3R = crate::BitReader;
#[doc = "Field `BRTIF3` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif3R = crate::BitReader;
#[doc = "Field `BTIF3` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif3R = crate::BitReader;
#[doc = "Field `TCIF3` reader - channel x buffer transfer complete"]
pub type Tcif3R = crate::BitReader;
#[doc = "Field `CRQA3` reader - channel x request active flag"]
pub type Crqa3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif3(&self) -> Teif3R {
        Teif3R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif3(&self) -> Ctcif3R {
        Ctcif3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif3(&self) -> Brtif3R {
        Brtif3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif3(&self) -> Btif3R {
        Btif3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif3(&self) -> Tcif3R {
        Tcif3R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa3(&self) -> Crqa3R {
        Crqa3R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC3isrSpec;
impl crate::RegisterSpec for MdmaC3isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c3isr::R`](R) reader structure"]
impl crate::Readable for MdmaC3isrSpec {}
#[doc = "`reset()` method sets MDMA_C3ISR to value 0"]
impl crate::Resettable for MdmaC3isrSpec {}

#[doc = "Register `MDMA_C12ISR` reader"]
pub type R = crate::R<MdmaC12isrSpec>;
#[doc = "Field `TEIF12` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif12R = crate::BitReader;
#[doc = "Field `CTCIF12` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif12R = crate::BitReader;
#[doc = "Field `BRTIF12` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif12R = crate::BitReader;
#[doc = "Field `BTIF12` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif12R = crate::BitReader;
#[doc = "Field `TCIF12` reader - channel x buffer transfer complete"]
pub type Tcif12R = crate::BitReader;
#[doc = "Field `CRQA12` reader - channel x request active flag"]
pub type Crqa12R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif12(&self) -> Teif12R {
        Teif12R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif12(&self) -> Ctcif12R {
        Ctcif12R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif12(&self) -> Brtif12R {
        Brtif12R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif12(&self) -> Btif12R {
        Btif12R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif12(&self) -> Tcif12R {
        Tcif12R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa12(&self) -> Crqa12R {
        Crqa12R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c12isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC12isrSpec;
impl crate::RegisterSpec for MdmaC12isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c12isr::R`](R) reader structure"]
impl crate::Readable for MdmaC12isrSpec {}
#[doc = "`reset()` method sets MDMA_C12ISR to value 0"]
impl crate::Resettable for MdmaC12isrSpec {}

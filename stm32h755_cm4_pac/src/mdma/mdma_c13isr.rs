#[doc = "Register `MDMA_C13ISR` reader"]
pub type R = crate::R<MdmaC13isrSpec>;
#[doc = "Field `TEIF13` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif13R = crate::BitReader;
#[doc = "Field `CTCIF13` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif13R = crate::BitReader;
#[doc = "Field `BRTIF13` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif13R = crate::BitReader;
#[doc = "Field `BTIF13` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif13R = crate::BitReader;
#[doc = "Field `TCIF13` reader - channel x buffer transfer complete"]
pub type Tcif13R = crate::BitReader;
#[doc = "Field `CRQA13` reader - channel x request active flag"]
pub type Crqa13R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif13(&self) -> Teif13R {
        Teif13R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif13(&self) -> Ctcif13R {
        Ctcif13R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif13(&self) -> Brtif13R {
        Brtif13R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif13(&self) -> Btif13R {
        Btif13R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif13(&self) -> Tcif13R {
        Tcif13R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa13(&self) -> Crqa13R {
        Crqa13R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c13isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC13isrSpec;
impl crate::RegisterSpec for MdmaC13isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c13isr::R`](R) reader structure"]
impl crate::Readable for MdmaC13isrSpec {}
#[doc = "`reset()` method sets MDMA_C13ISR to value 0"]
impl crate::Resettable for MdmaC13isrSpec {}

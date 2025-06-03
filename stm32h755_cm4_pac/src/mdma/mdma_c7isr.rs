#[doc = "Register `MDMA_C7ISR` reader"]
pub type R = crate::R<MdmaC7isrSpec>;
#[doc = "Field `TEIF7` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif7R = crate::BitReader;
#[doc = "Field `CTCIF7` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif7R = crate::BitReader;
#[doc = "Field `BRTIF7` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif7R = crate::BitReader;
#[doc = "Field `BTIF7` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif7R = crate::BitReader;
#[doc = "Field `TCIF7` reader - channel x buffer transfer complete"]
pub type Tcif7R = crate::BitReader;
#[doc = "Field `CRQA7` reader - channel x request active flag"]
pub type Crqa7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif7(&self) -> Teif7R {
        Teif7R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif7(&self) -> Ctcif7R {
        Ctcif7R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif7(&self) -> Brtif7R {
        Brtif7R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif7(&self) -> Btif7R {
        Btif7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif7(&self) -> Tcif7R {
        Tcif7R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa7(&self) -> Crqa7R {
        Crqa7R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c7isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC7isrSpec;
impl crate::RegisterSpec for MdmaC7isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c7isr::R`](R) reader structure"]
impl crate::Readable for MdmaC7isrSpec {}
#[doc = "`reset()` method sets MDMA_C7ISR to value 0"]
impl crate::Resettable for MdmaC7isrSpec {}

#[doc = "Register `MDMA_C6ISR` reader"]
pub type R = crate::R<MdmaC6isrSpec>;
#[doc = "Field `TEIF6` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif6R = crate::BitReader;
#[doc = "Field `CTCIF6` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif6R = crate::BitReader;
#[doc = "Field `BRTIF6` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif6R = crate::BitReader;
#[doc = "Field `BTIF6` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif6R = crate::BitReader;
#[doc = "Field `TCIF6` reader - channel x buffer transfer complete"]
pub type Tcif6R = crate::BitReader;
#[doc = "Field `CRQA6` reader - channel x request active flag"]
pub type Crqa6R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif6(&self) -> Teif6R {
        Teif6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif6(&self) -> Ctcif6R {
        Ctcif6R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif6(&self) -> Brtif6R {
        Brtif6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif6(&self) -> Btif6R {
        Btif6R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif6(&self) -> Tcif6R {
        Tcif6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa6(&self) -> Crqa6R {
        Crqa6R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c6isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC6isrSpec;
impl crate::RegisterSpec for MdmaC6isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c6isr::R`](R) reader structure"]
impl crate::Readable for MdmaC6isrSpec {}
#[doc = "`reset()` method sets MDMA_C6ISR to value 0"]
impl crate::Resettable for MdmaC6isrSpec {}

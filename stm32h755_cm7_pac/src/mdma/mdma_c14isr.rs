#[doc = "Register `MDMA_C14ISR` reader"]
pub type R = crate::R<MdmaC14isrSpec>;
#[doc = "Field `TEIF14` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif14R = crate::BitReader;
#[doc = "Field `CTCIF14` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif14R = crate::BitReader;
#[doc = "Field `BRTIF14` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif14R = crate::BitReader;
#[doc = "Field `BTIF14` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif14R = crate::BitReader;
#[doc = "Field `TCIF14` reader - channel x buffer transfer complete"]
pub type Tcif14R = crate::BitReader;
#[doc = "Field `CRQA14` reader - channel x request active flag"]
pub type Crqa14R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif14(&self) -> Teif14R {
        Teif14R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif14(&self) -> Ctcif14R {
        Ctcif14R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif14(&self) -> Brtif14R {
        Brtif14R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif14(&self) -> Btif14R {
        Btif14R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif14(&self) -> Tcif14R {
        Tcif14R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa14(&self) -> Crqa14R {
        Crqa14R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c14isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC14isrSpec;
impl crate::RegisterSpec for MdmaC14isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c14isr::R`](R) reader structure"]
impl crate::Readable for MdmaC14isrSpec {}
#[doc = "`reset()` method sets MDMA_C14ISR to value 0"]
impl crate::Resettable for MdmaC14isrSpec {}

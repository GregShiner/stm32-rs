#[doc = "Register `MDMA_C1ISR` reader"]
pub type R = crate::R<MdmaC1isrSpec>;
#[doc = "Field `TEIF1` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif1R = crate::BitReader;
#[doc = "Field `CTCIF1` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif1R = crate::BitReader;
#[doc = "Field `BRTIF1` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif1R = crate::BitReader;
#[doc = "Field `BTIF1` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif1R = crate::BitReader;
#[doc = "Field `TCIF1` reader - channel x buffer transfer complete"]
pub type Tcif1R = crate::BitReader;
#[doc = "Field `CRQA1` reader - channel x request active flag"]
pub type Crqa1R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif1(&self) -> Teif1R {
        Teif1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif1(&self) -> Ctcif1R {
        Ctcif1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif1(&self) -> Brtif1R {
        Brtif1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif1(&self) -> Btif1R {
        Btif1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif1(&self) -> Tcif1R {
        Tcif1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa1(&self) -> Crqa1R {
        Crqa1R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC1isrSpec;
impl crate::RegisterSpec for MdmaC1isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c1isr::R`](R) reader structure"]
impl crate::Readable for MdmaC1isrSpec {}
#[doc = "`reset()` method sets MDMA_C1ISR to value 0"]
impl crate::Resettable for MdmaC1isrSpec {}

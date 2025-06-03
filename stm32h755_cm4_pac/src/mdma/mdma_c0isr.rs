#[doc = "Register `MDMA_C0ISR` reader"]
pub type R = crate::R<MdmaC0isrSpec>;
#[doc = "Field `TEIF0` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif0R = crate::BitReader;
#[doc = "Field `CTCIF0` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif0R = crate::BitReader;
#[doc = "Field `BRTIF0` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif0R = crate::BitReader;
#[doc = "Field `BTIF0` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif0R = crate::BitReader;
#[doc = "Field `TCIF0` reader - channel x buffer transfer complete"]
pub type Tcif0R = crate::BitReader;
#[doc = "Field `CRQA0` reader - channel x request active flag"]
pub type Crqa0R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif0(&self) -> Teif0R {
        Teif0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif0(&self) -> Ctcif0R {
        Ctcif0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif0(&self) -> Brtif0R {
        Brtif0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif0(&self) -> Btif0R {
        Btif0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif0(&self) -> Tcif0R {
        Tcif0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa0(&self) -> Crqa0R {
        Crqa0R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c0isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC0isrSpec;
impl crate::RegisterSpec for MdmaC0isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c0isr::R`](R) reader structure"]
impl crate::Readable for MdmaC0isrSpec {}
#[doc = "`reset()` method sets MDMA_C0ISR to value 0"]
impl crate::Resettable for MdmaC0isrSpec {}

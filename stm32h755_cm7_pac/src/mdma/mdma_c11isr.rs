#[doc = "Register `MDMA_C11ISR` reader"]
pub type R = crate::R<MdmaC11isrSpec>;
#[doc = "Field `TEIF11` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif11R = crate::BitReader;
#[doc = "Field `CTCIF11` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif11R = crate::BitReader;
#[doc = "Field `BRTIF11` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif11R = crate::BitReader;
#[doc = "Field `BTIF11` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif11R = crate::BitReader;
#[doc = "Field `TCIF11` reader - channel x buffer transfer complete"]
pub type Tcif11R = crate::BitReader;
#[doc = "Field `CRQA11` reader - channel x request active flag"]
pub type Crqa11R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif11(&self) -> Teif11R {
        Teif11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif11(&self) -> Ctcif11R {
        Ctcif11R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif11(&self) -> Brtif11R {
        Brtif11R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif11(&self) -> Btif11R {
        Btif11R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif11(&self) -> Tcif11R {
        Tcif11R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa11(&self) -> Crqa11R {
        Crqa11R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c11isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC11isrSpec;
impl crate::RegisterSpec for MdmaC11isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c11isr::R`](R) reader structure"]
impl crate::Readable for MdmaC11isrSpec {}
#[doc = "`reset()` method sets MDMA_C11ISR to value 0"]
impl crate::Resettable for MdmaC11isrSpec {}

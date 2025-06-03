#[doc = "Register `MDMA_C10ISR` reader"]
pub type R = crate::R<MdmaC10isrSpec>;
#[doc = "Field `TEIF10` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif10R = crate::BitReader;
#[doc = "Field `CTCIF10` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif10R = crate::BitReader;
#[doc = "Field `BRTIF10` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif10R = crate::BitReader;
#[doc = "Field `BTIF10` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif10R = crate::BitReader;
#[doc = "Field `TCIF10` reader - channel x buffer transfer complete"]
pub type Tcif10R = crate::BitReader;
#[doc = "Field `CRQA10` reader - channel x request active flag"]
pub type Crqa10R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif10(&self) -> Teif10R {
        Teif10R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif10(&self) -> Ctcif10R {
        Ctcif10R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif10(&self) -> Brtif10R {
        Brtif10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif10(&self) -> Btif10R {
        Btif10R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif10(&self) -> Tcif10R {
        Tcif10R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa10(&self) -> Crqa10R {
        Crqa10R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c10isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC10isrSpec;
impl crate::RegisterSpec for MdmaC10isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c10isr::R`](R) reader structure"]
impl crate::Readable for MdmaC10isrSpec {}
#[doc = "`reset()` method sets MDMA_C10ISR to value 0"]
impl crate::Resettable for MdmaC10isrSpec {}

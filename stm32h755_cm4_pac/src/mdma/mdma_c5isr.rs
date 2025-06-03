#[doc = "Register `MDMA_C5ISR` reader"]
pub type R = crate::R<MdmaC5isrSpec>;
#[doc = "Field `TEIF5` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif5R = crate::BitReader;
#[doc = "Field `CTCIF5` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif5R = crate::BitReader;
#[doc = "Field `BRTIF5` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif5R = crate::BitReader;
#[doc = "Field `BTIF5` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif5R = crate::BitReader;
#[doc = "Field `TCIF5` reader - channel x buffer transfer complete"]
pub type Tcif5R = crate::BitReader;
#[doc = "Field `CRQA5` reader - channel x request active flag"]
pub type Crqa5R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif5(&self) -> Teif5R {
        Teif5R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif5(&self) -> Ctcif5R {
        Ctcif5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif5(&self) -> Brtif5R {
        Brtif5R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif5(&self) -> Btif5R {
        Btif5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif5(&self) -> Tcif5R {
        Tcif5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa5(&self) -> Crqa5R {
        Crqa5R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c5isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC5isrSpec;
impl crate::RegisterSpec for MdmaC5isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c5isr::R`](R) reader structure"]
impl crate::Readable for MdmaC5isrSpec {}
#[doc = "`reset()` method sets MDMA_C5ISR to value 0"]
impl crate::Resettable for MdmaC5isrSpec {}

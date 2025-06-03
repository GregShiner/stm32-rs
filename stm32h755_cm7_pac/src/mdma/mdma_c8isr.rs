#[doc = "Register `MDMA_C8ISR` reader"]
pub type R = crate::R<MdmaC8isrSpec>;
#[doc = "Field `TEIF8` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif8R = crate::BitReader;
#[doc = "Field `CTCIF8` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif8R = crate::BitReader;
#[doc = "Field `BRTIF8` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif8R = crate::BitReader;
#[doc = "Field `BTIF8` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif8R = crate::BitReader;
#[doc = "Field `TCIF8` reader - channel x buffer transfer complete"]
pub type Tcif8R = crate::BitReader;
#[doc = "Field `CRQA8` reader - channel x request active flag"]
pub type Crqa8R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif8(&self) -> Teif8R {
        Teif8R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif8(&self) -> Ctcif8R {
        Ctcif8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif8(&self) -> Brtif8R {
        Brtif8R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif8(&self) -> Btif8R {
        Btif8R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif8(&self) -> Tcif8R {
        Tcif8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa8(&self) -> Crqa8R {
        Crqa8R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC8isrSpec;
impl crate::RegisterSpec for MdmaC8isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c8isr::R`](R) reader structure"]
impl crate::Readable for MdmaC8isrSpec {}
#[doc = "`reset()` method sets MDMA_C8ISR to value 0"]
impl crate::Resettable for MdmaC8isrSpec {}

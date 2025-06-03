#[doc = "Register `MDMA_C2ISR` reader"]
pub type R = crate::R<MdmaC2isrSpec>;
#[doc = "Field `TEIF2` reader - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Teif2R = crate::BitReader;
#[doc = "Field `CTCIF2` reader - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
pub type Ctcif2R = crate::BitReader;
#[doc = "Field `BRTIF2` reader - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Brtif2R = crate::BitReader;
#[doc = "Field `BTIF2` reader - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
pub type Btif2R = crate::BitReader;
#[doc = "Field `TCIF2` reader - channel x buffer transfer complete"]
pub type Tcif2R = crate::BitReader;
#[doc = "Field `CRQA2` reader - channel x request active flag"]
pub type Crqa2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Channel x transfer error interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn teif2(&self) -> Teif2R {
        Teif2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel x Channel Transfer Complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register. CTC is set when the last block was transferred and the channel has been automatically disabled. CTC is also set when the channel is suspended, as a result of writing EN bit to 0."]
    #[inline(always)]
    pub fn ctcif2(&self) -> Ctcif2R {
        Ctcif2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel x block repeat transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn brtif2(&self) -> Brtif2R {
        Brtif2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Channel x block transfer complete interrupt flag This bit is set by hardware. It is cleared by software writing 1 to the corresponding bit in the DMA_IFCRy register."]
    #[inline(always)]
    pub fn btif2(&self) -> Btif2R {
        Btif2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - channel x buffer transfer complete"]
    #[inline(always)]
    pub fn tcif2(&self) -> Tcif2R {
        Tcif2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - channel x request active flag"]
    #[inline(always)]
    pub fn crqa2(&self) -> Crqa2R {
        Crqa2R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "MDMA channel x interrupt/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC2isrSpec;
impl crate::RegisterSpec for MdmaC2isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c2isr::R`](R) reader structure"]
impl crate::Readable for MdmaC2isrSpec {}
#[doc = "`reset()` method sets MDMA_C2ISR to value 0"]
impl crate::Resettable for MdmaC2isrSpec {}

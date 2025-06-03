#[doc = "Register `MDMA_C6IFCR` writer"]
pub type W = crate::W<MdmaC6ifcrSpec>;
#[doc = "Field `CTEIF6` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF6` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF6` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF6` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF6` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif6W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif6(&mut self) -> Cteif6W<MdmaC6ifcrSpec> {
        Cteif6W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif6(&mut self) -> Cctcif6W<MdmaC6ifcrSpec> {
        Cctcif6W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif6(&mut self) -> Cbrtif6W<MdmaC6ifcrSpec> {
        Cbrtif6W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif6(&mut self) -> Cbtif6W<MdmaC6ifcrSpec> {
        Cbtif6W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif6(&mut self) -> Cltcif6W<MdmaC6ifcrSpec> {
        Cltcif6W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC6ifcrSpec;
impl crate::RegisterSpec for MdmaC6ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c6ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC6ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C6IFCR to value 0"]
impl crate::Resettable for MdmaC6ifcrSpec {}

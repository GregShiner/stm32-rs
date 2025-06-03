#[doc = "Register `MDMA_C7IFCR` writer"]
pub type W = crate::W<MdmaC7ifcrSpec>;
#[doc = "Field `CTEIF7` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF7` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF7` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF7` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF7` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif7(&mut self) -> Cteif7W<MdmaC7ifcrSpec> {
        Cteif7W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif7(&mut self) -> Cctcif7W<MdmaC7ifcrSpec> {
        Cctcif7W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif7(&mut self) -> Cbrtif7W<MdmaC7ifcrSpec> {
        Cbrtif7W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif7(&mut self) -> Cbtif7W<MdmaC7ifcrSpec> {
        Cbtif7W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif7(&mut self) -> Cltcif7W<MdmaC7ifcrSpec> {
        Cltcif7W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC7ifcrSpec;
impl crate::RegisterSpec for MdmaC7ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c7ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC7ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C7IFCR to value 0"]
impl crate::Resettable for MdmaC7ifcrSpec {}

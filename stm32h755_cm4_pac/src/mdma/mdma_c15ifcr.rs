#[doc = "Register `MDMA_C15IFCR` writer"]
pub type W = crate::W<MdmaC15ifcrSpec>;
#[doc = "Field `CTEIF15` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF15` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF15` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF15` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF15` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif15W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif15(&mut self) -> Cteif15W<MdmaC15ifcrSpec> {
        Cteif15W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif15(&mut self) -> Cctcif15W<MdmaC15ifcrSpec> {
        Cctcif15W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif15(&mut self) -> Cbrtif15W<MdmaC15ifcrSpec> {
        Cbrtif15W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif15(&mut self) -> Cbtif15W<MdmaC15ifcrSpec> {
        Cbtif15W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif15(&mut self) -> Cltcif15W<MdmaC15ifcrSpec> {
        Cltcif15W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC15ifcrSpec;
impl crate::RegisterSpec for MdmaC15ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c15ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC15ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C15IFCR to value 0"]
impl crate::Resettable for MdmaC15ifcrSpec {}

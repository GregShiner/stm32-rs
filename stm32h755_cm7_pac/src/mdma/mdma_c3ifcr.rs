#[doc = "Register `MDMA_C3IFCR` writer"]
pub type W = crate::W<MdmaC3ifcrSpec>;
#[doc = "Field `CTEIF3` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF3` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF3` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF3` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF3` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif3W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif3(&mut self) -> Cteif3W<MdmaC3ifcrSpec> {
        Cteif3W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif3(&mut self) -> Cctcif3W<MdmaC3ifcrSpec> {
        Cctcif3W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif3(&mut self) -> Cbrtif3W<MdmaC3ifcrSpec> {
        Cbrtif3W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif3(&mut self) -> Cbtif3W<MdmaC3ifcrSpec> {
        Cbtif3W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif3(&mut self) -> Cltcif3W<MdmaC3ifcrSpec> {
        Cltcif3W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC3ifcrSpec;
impl crate::RegisterSpec for MdmaC3ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c3ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC3ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C3IFCR to value 0"]
impl crate::Resettable for MdmaC3ifcrSpec {}

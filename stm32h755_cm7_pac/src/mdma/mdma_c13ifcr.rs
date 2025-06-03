#[doc = "Register `MDMA_C13IFCR` writer"]
pub type W = crate::W<MdmaC13ifcrSpec>;
#[doc = "Field `CTEIF13` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF13` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF13` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF13` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF13` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif13W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif13(&mut self) -> Cteif13W<MdmaC13ifcrSpec> {
        Cteif13W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif13(&mut self) -> Cctcif13W<MdmaC13ifcrSpec> {
        Cctcif13W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif13(&mut self) -> Cbrtif13W<MdmaC13ifcrSpec> {
        Cbrtif13W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif13(&mut self) -> Cbtif13W<MdmaC13ifcrSpec> {
        Cbtif13W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif13(&mut self) -> Cltcif13W<MdmaC13ifcrSpec> {
        Cltcif13W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC13ifcrSpec;
impl crate::RegisterSpec for MdmaC13ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c13ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC13ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C13IFCR to value 0"]
impl crate::Resettable for MdmaC13ifcrSpec {}

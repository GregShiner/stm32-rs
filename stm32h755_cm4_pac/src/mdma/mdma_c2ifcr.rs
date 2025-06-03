#[doc = "Register `MDMA_C2IFCR` writer"]
pub type W = crate::W<MdmaC2ifcrSpec>;
#[doc = "Field `CTEIF2` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF2` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF2` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF2` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF2` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif2(&mut self) -> Cteif2W<MdmaC2ifcrSpec> {
        Cteif2W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif2(&mut self) -> Cctcif2W<MdmaC2ifcrSpec> {
        Cctcif2W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif2(&mut self) -> Cbrtif2W<MdmaC2ifcrSpec> {
        Cbrtif2W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif2(&mut self) -> Cbtif2W<MdmaC2ifcrSpec> {
        Cbtif2W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif2(&mut self) -> Cltcif2W<MdmaC2ifcrSpec> {
        Cltcif2W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC2ifcrSpec;
impl crate::RegisterSpec for MdmaC2ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c2ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC2ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C2IFCR to value 0"]
impl crate::Resettable for MdmaC2ifcrSpec {}

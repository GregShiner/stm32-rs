#[doc = "Register `MDMA_C11IFCR` writer"]
pub type W = crate::W<MdmaC11ifcrSpec>;
#[doc = "Field `CTEIF11` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF11` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF11` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF11` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF11` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif11W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif11(&mut self) -> Cteif11W<MdmaC11ifcrSpec> {
        Cteif11W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif11(&mut self) -> Cctcif11W<MdmaC11ifcrSpec> {
        Cctcif11W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif11(&mut self) -> Cbrtif11W<MdmaC11ifcrSpec> {
        Cbrtif11W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif11(&mut self) -> Cbtif11W<MdmaC11ifcrSpec> {
        Cbtif11W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif11(&mut self) -> Cltcif11W<MdmaC11ifcrSpec> {
        Cltcif11W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC11ifcrSpec;
impl crate::RegisterSpec for MdmaC11ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c11ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC11ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C11IFCR to value 0"]
impl crate::Resettable for MdmaC11ifcrSpec {}

#[doc = "Register `MDMA_C10IFCR` writer"]
pub type W = crate::W<MdmaC10ifcrSpec>;
#[doc = "Field `CTEIF10` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF10` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF10` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF10` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF10` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif10W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif10(&mut self) -> Cteif10W<MdmaC10ifcrSpec> {
        Cteif10W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif10(&mut self) -> Cctcif10W<MdmaC10ifcrSpec> {
        Cctcif10W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif10(&mut self) -> Cbrtif10W<MdmaC10ifcrSpec> {
        Cbrtif10W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif10(&mut self) -> Cbtif10W<MdmaC10ifcrSpec> {
        Cbtif10W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif10(&mut self) -> Cltcif10W<MdmaC10ifcrSpec> {
        Cltcif10W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC10ifcrSpec;
impl crate::RegisterSpec for MdmaC10ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c10ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC10ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C10IFCR to value 0"]
impl crate::Resettable for MdmaC10ifcrSpec {}

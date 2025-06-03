#[doc = "Register `MDMA_C4IFCR` writer"]
pub type W = crate::W<MdmaC4ifcrSpec>;
#[doc = "Field `CTEIF4` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF4` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF4` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF4` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF4` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif4W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif4(&mut self) -> Cteif4W<MdmaC4ifcrSpec> {
        Cteif4W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif4(&mut self) -> Cctcif4W<MdmaC4ifcrSpec> {
        Cctcif4W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif4(&mut self) -> Cbrtif4W<MdmaC4ifcrSpec> {
        Cbrtif4W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif4(&mut self) -> Cbtif4W<MdmaC4ifcrSpec> {
        Cbtif4W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif4(&mut self) -> Cltcif4W<MdmaC4ifcrSpec> {
        Cltcif4W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC4ifcrSpec;
impl crate::RegisterSpec for MdmaC4ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c4ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC4ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C4IFCR to value 0"]
impl crate::Resettable for MdmaC4ifcrSpec {}

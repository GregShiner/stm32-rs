#[doc = "Register `MDMA_C1IFCR` writer"]
pub type W = crate::W<MdmaC1ifcrSpec>;
#[doc = "Field `CTEIF1` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF1` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF1` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF1` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF1` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif1W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif1(&mut self) -> Cteif1W<MdmaC1ifcrSpec> {
        Cteif1W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif1(&mut self) -> Cctcif1W<MdmaC1ifcrSpec> {
        Cctcif1W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif1(&mut self) -> Cbrtif1W<MdmaC1ifcrSpec> {
        Cbrtif1W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif1(&mut self) -> Cbtif1W<MdmaC1ifcrSpec> {
        Cbtif1W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif1(&mut self) -> Cltcif1W<MdmaC1ifcrSpec> {
        Cltcif1W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC1ifcrSpec;
impl crate::RegisterSpec for MdmaC1ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c1ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC1ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C1IFCR to value 0"]
impl crate::Resettable for MdmaC1ifcrSpec {}

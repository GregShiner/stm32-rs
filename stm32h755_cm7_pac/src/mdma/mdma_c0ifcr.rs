#[doc = "Register `MDMA_C0IFCR` writer"]
pub type W = crate::W<MdmaC0ifcrSpec>;
#[doc = "Field `CTEIF0` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF0` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF0` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF0` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF0` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif0W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif0(&mut self) -> Cteif0W<MdmaC0ifcrSpec> {
        Cteif0W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif0(&mut self) -> Cctcif0W<MdmaC0ifcrSpec> {
        Cctcif0W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif0(&mut self) -> Cbrtif0W<MdmaC0ifcrSpec> {
        Cbrtif0W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif0(&mut self) -> Cbtif0W<MdmaC0ifcrSpec> {
        Cbtif0W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif0(&mut self) -> Cltcif0W<MdmaC0ifcrSpec> {
        Cltcif0W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC0ifcrSpec;
impl crate::RegisterSpec for MdmaC0ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c0ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC0ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C0IFCR to value 0"]
impl crate::Resettable for MdmaC0ifcrSpec {}

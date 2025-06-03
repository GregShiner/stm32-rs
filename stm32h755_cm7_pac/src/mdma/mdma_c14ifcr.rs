#[doc = "Register `MDMA_C14IFCR` writer"]
pub type W = crate::W<MdmaC14ifcrSpec>;
#[doc = "Field `CTEIF14` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF14` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF14` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF14` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF14` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif14W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif14(&mut self) -> Cteif14W<MdmaC14ifcrSpec> {
        Cteif14W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif14(&mut self) -> Cctcif14W<MdmaC14ifcrSpec> {
        Cctcif14W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif14(&mut self) -> Cbrtif14W<MdmaC14ifcrSpec> {
        Cbrtif14W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif14(&mut self) -> Cbtif14W<MdmaC14ifcrSpec> {
        Cbtif14W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif14(&mut self) -> Cltcif14W<MdmaC14ifcrSpec> {
        Cltcif14W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC14ifcrSpec;
impl crate::RegisterSpec for MdmaC14ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c14ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC14ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C14IFCR to value 0"]
impl crate::Resettable for MdmaC14ifcrSpec {}

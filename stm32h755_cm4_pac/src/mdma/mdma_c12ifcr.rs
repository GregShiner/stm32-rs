#[doc = "Register `MDMA_C12IFCR` writer"]
pub type W = crate::W<MdmaC12ifcrSpec>;
#[doc = "Field `CTEIF12` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF12` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF12` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF12` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF12` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif12W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif12(&mut self) -> Cteif12W<MdmaC12ifcrSpec> {
        Cteif12W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif12(&mut self) -> Cctcif12W<MdmaC12ifcrSpec> {
        Cctcif12W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif12(&mut self) -> Cbrtif12W<MdmaC12ifcrSpec> {
        Cbrtif12W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif12(&mut self) -> Cbtif12W<MdmaC12ifcrSpec> {
        Cbtif12W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif12(&mut self) -> Cltcif12W<MdmaC12ifcrSpec> {
        Cltcif12W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC12ifcrSpec;
impl crate::RegisterSpec for MdmaC12ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c12ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC12ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C12IFCR to value 0"]
impl crate::Resettable for MdmaC12ifcrSpec {}

#[doc = "Register `MDMA_C5IFCR` writer"]
pub type W = crate::W<MdmaC5ifcrSpec>;
#[doc = "Field `CTEIF5` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF5` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF5` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF5` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF5` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif5W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif5(&mut self) -> Cteif5W<MdmaC5ifcrSpec> {
        Cteif5W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif5(&mut self) -> Cctcif5W<MdmaC5ifcrSpec> {
        Cctcif5W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif5(&mut self) -> Cbrtif5W<MdmaC5ifcrSpec> {
        Cbrtif5W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif5(&mut self) -> Cbtif5W<MdmaC5ifcrSpec> {
        Cbtif5W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif5(&mut self) -> Cltcif5W<MdmaC5ifcrSpec> {
        Cltcif5W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC5ifcrSpec;
impl crate::RegisterSpec for MdmaC5ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c5ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC5ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C5IFCR to value 0"]
impl crate::Resettable for MdmaC5ifcrSpec {}

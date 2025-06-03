#[doc = "Register `MDMA_C9IFCR` writer"]
pub type W = crate::W<MdmaC9ifcrSpec>;
#[doc = "Field `CTEIF9` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF9` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF9` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF9` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF9` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif9W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif9(&mut self) -> Cteif9W<MdmaC9ifcrSpec> {
        Cteif9W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif9(&mut self) -> Cctcif9W<MdmaC9ifcrSpec> {
        Cctcif9W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif9(&mut self) -> Cbrtif9W<MdmaC9ifcrSpec> {
        Cbrtif9W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif9(&mut self) -> Cbtif9W<MdmaC9ifcrSpec> {
        Cbtif9W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif9(&mut self) -> Cltcif9W<MdmaC9ifcrSpec> {
        Cltcif9W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC9ifcrSpec;
impl crate::RegisterSpec for MdmaC9ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c9ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC9ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C9IFCR to value 0"]
impl crate::Resettable for MdmaC9ifcrSpec {}

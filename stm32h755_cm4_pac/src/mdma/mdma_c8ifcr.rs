#[doc = "Register `MDMA_C8IFCR` writer"]
pub type W = crate::W<MdmaC8ifcrSpec>;
#[doc = "Field `CTEIF8` writer - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
pub type Cteif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCTCIF8` writer - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
pub type Cctcif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBRTIF8` writer - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
pub type Cbrtif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CBTIF8` writer - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
pub type Cbtif8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLTCIF8` writer - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
pub type Cltcif8W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Channel x clear transfer error interrupt flag Writing a 1 into this bit clears TEIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cteif8(&mut self) -> Cteif8W<MdmaC8ifcrSpec> {
        Cteif8W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Channel transfer complete interrupt flag for channel x Writing a 1 into this bit clears CTCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cctcif8(&mut self) -> Cctcif8W<MdmaC8ifcrSpec> {
        Cctcif8W::new(self, 1)
    }
    #[doc = "Bit 2 - Channel x clear block repeat transfer complete interrupt flag Writing a 1 into this bit clears BRTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbrtif8(&mut self) -> Cbrtif8W<MdmaC8ifcrSpec> {
        Cbrtif8W::new(self, 2)
    }
    #[doc = "Bit 3 - Channel x Clear block transfer complete interrupt flag Writing a 1 into this bit clears BTIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cbtif8(&mut self) -> Cbtif8W<MdmaC8ifcrSpec> {
        Cbtif8W::new(self, 3)
    }
    #[doc = "Bit 4 - CLear buffer Transfer Complete Interrupt Flag for channel x Writing a 1 into this bit clears TCIFx in the MDMA_ISRy register"]
    #[inline(always)]
    pub fn cltcif8(&mut self) -> Cltcif8W<MdmaC8ifcrSpec> {
        Cltcif8W::new(self, 4)
    }
}
#[doc = "MDMA channel x interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC8ifcrSpec;
impl crate::RegisterSpec for MdmaC8ifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mdma_c8ifcr::W`](W) writer structure"]
impl crate::Writable for MdmaC8ifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C8IFCR to value 0"]
impl crate::Resettable for MdmaC8ifcrSpec {}

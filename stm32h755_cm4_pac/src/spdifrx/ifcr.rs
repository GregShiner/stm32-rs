#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IfcrSpec>;
#[doc = "Field `PERRCF` writer - Clears the Parity error flag"]
pub type PerrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRCF` writer - Clears the Overrun error flag"]
pub type OvrcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBDCF` writer - Clears the Synchronization Block Detected flag"]
pub type SbdcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDCF` writer - Clears the Synchronization Done flag"]
pub type SyncdcfW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Clears the Parity error flag"]
    #[inline(always)]
    pub fn perrcf(&mut self) -> PerrcfW<IfcrSpec> {
        PerrcfW::new(self, 2)
    }
    #[doc = "Bit 3 - Clears the Overrun error flag"]
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OvrcfW<IfcrSpec> {
        OvrcfW::new(self, 3)
    }
    #[doc = "Bit 4 - Clears the Synchronization Block Detected flag"]
    #[inline(always)]
    pub fn sbdcf(&mut self) -> SbdcfW<IfcrSpec> {
        SbdcfW::new(self, 4)
    }
    #[doc = "Bit 5 - Clears the Synchronization Done flag"]
    #[inline(always)]
    pub fn syncdcf(&mut self) -> SyncdcfW<IfcrSpec> {
        SyncdcfW::new(self, 5)
    }
}
#[doc = "Interrupt Flag Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IfcrSpec;
impl crate::RegisterSpec for IfcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IfcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IfcrSpec {}

#[doc = "Register `FDCAN_TTTS` reader"]
pub type R = crate::R<FdcanTttsSpec>;
#[doc = "Register `FDCAN_TTTS` writer"]
pub type W = crate::W<FdcanTttsSpec>;
#[doc = "Field `SWTDEL` reader - Stop watch trigger input selection"]
pub type SwtdelR = crate::FieldReader;
#[doc = "Field `SWTDEL` writer - Stop watch trigger input selection"]
pub type SwtdelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EVTSEL` reader - Event trigger input selection"]
pub type EvtselR = crate::FieldReader;
#[doc = "Field `EVTSEL` writer - Event trigger input selection"]
pub type EvtselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Stop watch trigger input selection"]
    #[inline(always)]
    pub fn swtdel(&self) -> SwtdelR {
        SwtdelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Event trigger input selection"]
    #[inline(always)]
    pub fn evtsel(&self) -> EvtselR {
        EvtselR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Stop watch trigger input selection"]
    #[inline(always)]
    pub fn swtdel(&mut self) -> SwtdelW<FdcanTttsSpec> {
        SwtdelW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Event trigger input selection"]
    #[inline(always)]
    pub fn evtsel(&mut self) -> EvtselW<FdcanTttsSpec> {
        EvtselW::new(self, 4)
    }
}
#[doc = "FDCAN TT Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTttsSpec;
impl crate::RegisterSpec for FdcanTttsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttts::R`](R) reader structure"]
impl crate::Readable for FdcanTttsSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttts::W`](W) writer structure"]
impl crate::Writable for FdcanTttsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTTS to value 0"]
impl crate::Resettable for FdcanTttsSpec {}

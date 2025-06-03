#[doc = "Register `MACTSICNR` reader"]
pub type R = crate::R<MactsicnrSpec>;
#[doc = "Register `MACTSICNR` writer"]
pub type W = crate::W<MactsicnrSpec>;
#[doc = "Field `TSIC` reader - TSIC"]
pub type TsicR = crate::FieldReader<u32>;
#[doc = "Field `TSIC` writer - TSIC"]
pub type TsicW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSIC"]
    #[inline(always)]
    pub fn tsic(&self) -> TsicR {
        TsicR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSIC"]
    #[inline(always)]
    pub fn tsic(&mut self) -> TsicW<MactsicnrSpec> {
        TsicW::new(self, 0)
    }
}
#[doc = "Timestamp Ingress correction nanosecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactsicnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsicnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactsicnrSpec;
impl crate::RegisterSpec for MactsicnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsicnr::R`](R) reader structure"]
impl crate::Readable for MactsicnrSpec {}
#[doc = "`write(|w| ..)` method takes [`mactsicnr::W`](W) writer structure"]
impl crate::Writable for MactsicnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACTSICNR to value 0"]
impl crate::Resettable for MactsicnrSpec {}

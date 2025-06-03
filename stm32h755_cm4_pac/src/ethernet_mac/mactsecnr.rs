#[doc = "Register `MACTSECNR` reader"]
pub type R = crate::R<MactsecnrSpec>;
#[doc = "Register `MACTSECNR` writer"]
pub type W = crate::W<MactsecnrSpec>;
#[doc = "Field `TSEC` reader - TSEC"]
pub type TsecR = crate::FieldReader<u32>;
#[doc = "Field `TSEC` writer - TSEC"]
pub type TsecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TSEC"]
    #[inline(always)]
    pub fn tsec(&self) -> TsecR {
        TsecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - TSEC"]
    #[inline(always)]
    pub fn tsec(&mut self) -> TsecW<MactsecnrSpec> {
        TsecW::new(self, 0)
    }
}
#[doc = "Timestamp Egress correction nanosecond register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactsecnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsecnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactsecnrSpec;
impl crate::RegisterSpec for MactsecnrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsecnr::R`](R) reader structure"]
impl crate::Readable for MactsecnrSpec {}
#[doc = "`write(|w| ..)` method takes [`mactsecnr::W`](W) writer structure"]
impl crate::Writable for MactsecnrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACTSECNR to value 0"]
impl crate::Resettable for MactsecnrSpec {}

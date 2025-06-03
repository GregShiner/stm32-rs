#[doc = "Register `M5FAR` reader"]
pub type R = crate::R<M5farSpec>;
#[doc = "Register `M5FAR` writer"]
pub type W = crate::W<M5farSpec>;
#[doc = "Field `FEC` reader - Failing error code"]
pub type FecR = crate::FieldReader<u32>;
#[doc = "Field `FEC` writer - Failing error code"]
pub type FecW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&self) -> FecR {
        FecR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing error code"]
    #[inline(always)]
    pub fn fec(&mut self) -> FecW<M5farSpec> {
        FecW::new(self, 0)
    }
}
#[doc = "RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5far::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5far::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5farSpec;
impl crate::RegisterSpec for M5farSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5far::R`](R) reader structure"]
impl crate::Readable for M5farSpec {}
#[doc = "`write(|w| ..)` method takes [`m5far::W`](W) writer structure"]
impl crate::Writable for M5farSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M5FAR to value 0"]
impl crate::Resettable for M5farSpec {}

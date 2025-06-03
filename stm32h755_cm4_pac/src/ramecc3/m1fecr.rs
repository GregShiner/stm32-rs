#[doc = "Register `M1FECR` reader"]
pub type R = crate::R<M1fecrSpec>;
#[doc = "Register `M1FECR` writer"]
pub type W = crate::W<M1fecrSpec>;
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
    pub fn fec(&mut self) -> FecW<M1fecrSpec> {
        FecW::new(self, 0)
    }
}
#[doc = "RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1fecr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fecr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M1fecrSpec;
impl crate::RegisterSpec for M1fecrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m1fecr::R`](R) reader structure"]
impl crate::Readable for M1fecrSpec {}
#[doc = "`write(|w| ..)` method takes [`m1fecr::W`](W) writer structure"]
impl crate::Writable for M1fecrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M1FECR to value 0"]
impl crate::Resettable for M1fecrSpec {}

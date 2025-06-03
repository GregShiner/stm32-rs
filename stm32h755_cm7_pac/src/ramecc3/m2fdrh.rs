#[doc = "Register `M2FDRH` reader"]
pub type R = crate::R<M2fdrhSpec>;
#[doc = "Register `M2FDRH` writer"]
pub type W = crate::W<M2fdrhSpec>;
#[doc = "Field `FDATAH` reader - Failing data high (64-bit memory)"]
pub type FdatahR = crate::FieldReader<u32>;
#[doc = "Field `FDATAH` writer - Failing data high (64-bit memory)"]
pub type FdatahW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&self) -> FdatahR {
        FdatahR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Failing data high (64-bit memory)"]
    #[inline(always)]
    pub fn fdatah(&mut self) -> FdatahW<M2fdrhSpec> {
        FdatahW::new(self, 0)
    }
}
#[doc = "RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2fdrh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fdrh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2fdrhSpec;
impl crate::RegisterSpec for M2fdrhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2fdrh::R`](R) reader structure"]
impl crate::Readable for M2fdrhSpec {}
#[doc = "`write(|w| ..)` method takes [`m2fdrh::W`](W) writer structure"]
impl crate::Writable for M2fdrhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M2FDRH to value 0"]
impl crate::Resettable for M2fdrhSpec {}

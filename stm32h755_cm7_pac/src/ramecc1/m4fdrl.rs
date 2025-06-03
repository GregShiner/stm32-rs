#[doc = "Register `M4FDRL` reader"]
pub type R = crate::R<M4fdrlSpec>;
#[doc = "Register `M4FDRL` writer"]
pub type W = crate::W<M4fdrlSpec>;
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
    pub fn fdatah(&mut self) -> FdatahW<M4fdrlSpec> {
        FdatahW::new(self, 0)
    }
}
#[doc = "RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4fdrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4fdrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M4fdrlSpec;
impl crate::RegisterSpec for M4fdrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m4fdrl::R`](R) reader structure"]
impl crate::Readable for M4fdrlSpec {}
#[doc = "`write(|w| ..)` method takes [`m4fdrl::W`](W) writer structure"]
impl crate::Writable for M4fdrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M4FDRL to value 0"]
impl crate::Resettable for M4fdrlSpec {}

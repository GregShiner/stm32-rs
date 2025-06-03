#[doc = "Register `M5SR` reader"]
pub type R = crate::R<M5srSpec>;
#[doc = "Register `M5SR` writer"]
pub type W = crate::W<M5srSpec>;
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
    pub fn fec(&mut self) -> FecW<M5srSpec> {
        FecW::new(self, 0)
    }
}
#[doc = "RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5srSpec;
impl crate::RegisterSpec for M5srSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5sr::R`](R) reader structure"]
impl crate::Readable for M5srSpec {}
#[doc = "`write(|w| ..)` method takes [`m5sr::W`](W) writer structure"]
impl crate::Writable for M5srSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M5SR to value 0"]
impl crate::Resettable for M5srSpec {}

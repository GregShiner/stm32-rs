#[doc = "Register `PERCR` reader"]
pub type R = crate::R<PercrSpec>;
#[doc = "Register `PERCR` writer"]
pub type W = crate::W<PercrSpec>;
#[doc = "Field `PERx` reader - Timerx Period value"]
pub type PerxR = crate::FieldReader<u16>;
#[doc = "Field `PERx` writer - Timerx Period value"]
pub type PerxW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&self) -> PerxR {
        PerxR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timerx Period value"]
    #[inline(always)]
    pub fn perx(&mut self) -> PerxW<PercrSpec> {
        PerxW::new(self, 0)
    }
}
#[doc = "Timerx Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`percr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`percr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PercrSpec;
impl crate::RegisterSpec for PercrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`percr::R`](R) reader structure"]
impl crate::Readable for PercrSpec {}
#[doc = "`write(|w| ..)` method takes [`percr::W`](W) writer structure"]
impl crate::Writable for PercrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERCR to value 0xffff"]
impl crate::Resettable for PercrSpec {
    const RESET_VALUE: u32 = 0xffff;
}

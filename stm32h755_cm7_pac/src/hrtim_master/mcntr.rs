#[doc = "Register `MCNTR` reader"]
pub type R = crate::R<McntrSpec>;
#[doc = "Register `MCNTR` writer"]
pub type W = crate::W<McntrSpec>;
#[doc = "Field `MCNT` reader - Counter value"]
pub type McntR = crate::FieldReader<u16>;
#[doc = "Field `MCNT` writer - Counter value"]
pub type McntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn mcnt(&self) -> McntR {
        McntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Counter value"]
    #[inline(always)]
    pub fn mcnt(&mut self) -> McntW<McntrSpec> {
        McntW::new(self, 0)
    }
}
#[doc = "Master Timer Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcntr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcntr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McntrSpec;
impl crate::RegisterSpec for McntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcntr::R`](R) reader structure"]
impl crate::Readable for McntrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcntr::W`](W) writer structure"]
impl crate::Writable for McntrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCNTR to value 0"]
impl crate::Resettable for McntrSpec {}

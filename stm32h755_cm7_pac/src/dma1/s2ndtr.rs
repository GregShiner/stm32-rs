#[doc = "Register `S2NDTR` reader"]
pub type R = crate::R<S2ndtrSpec>;
#[doc = "Register `S2NDTR` writer"]
pub type W = crate::W<S2ndtrSpec>;
#[doc = "Field `NDT` reader - Number of data items to transfer"]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data items to transfer"]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<S2ndtrSpec> {
        NdtW::new(self, 0)
    }
}
#[doc = "stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2ndtrSpec;
impl crate::RegisterSpec for S2ndtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2ndtr::R`](R) reader structure"]
impl crate::Readable for S2ndtrSpec {}
#[doc = "`write(|w| ..)` method takes [`s2ndtr::W`](W) writer structure"]
impl crate::Writable for S2ndtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S2NDTR to value 0"]
impl crate::Resettable for S2ndtrSpec {}

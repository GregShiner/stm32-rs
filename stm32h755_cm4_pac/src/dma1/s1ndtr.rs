#[doc = "Register `S1NDTR` reader"]
pub type R = crate::R<S1ndtrSpec>;
#[doc = "Register `S1NDTR` writer"]
pub type W = crate::W<S1ndtrSpec>;
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
    pub fn ndt(&mut self) -> NdtW<S1ndtrSpec> {
        NdtW::new(self, 0)
    }
}
#[doc = "stream x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`s1ndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s1ndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S1ndtrSpec;
impl crate::RegisterSpec for S1ndtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s1ndtr::R`](R) reader structure"]
impl crate::Readable for S1ndtrSpec {}
#[doc = "`write(|w| ..)` method takes [`s1ndtr::W`](W) writer structure"]
impl crate::Writable for S1ndtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S1NDTR to value 0"]
impl crate::Resettable for S1ndtrSpec {}

#[doc = "Register `BMCMPR6` reader"]
pub type R = crate::R<Bmcmpr6Spec>;
#[doc = "Register `BMCMPR6` writer"]
pub type W = crate::W<Bmcmpr6Spec>;
#[doc = "Field `BMCMP` reader - BMCMP"]
pub type BmcmpR = crate::FieldReader<u16>;
#[doc = "Field `BMCMP` writer - BMCMP"]
pub type BmcmpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&self) -> BmcmpR {
        BmcmpR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BMCMP"]
    #[inline(always)]
    pub fn bmcmp(&mut self) -> BmcmpW<Bmcmpr6Spec> {
        BmcmpW::new(self, 0)
    }
}
#[doc = "BMCMPR6\n\nYou can [`read`](crate::Reg::read) this register and get [`bmcmpr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcmpr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Bmcmpr6Spec;
impl crate::RegisterSpec for Bmcmpr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmcmpr6::R`](R) reader structure"]
impl crate::Readable for Bmcmpr6Spec {}
#[doc = "`write(|w| ..)` method takes [`bmcmpr6::W`](W) writer structure"]
impl crate::Writable for Bmcmpr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BMCMPR6 to value 0"]
impl crate::Resettable for Bmcmpr6Spec {}

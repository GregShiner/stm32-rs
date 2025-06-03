#[doc = "Register `CONFR0` writer"]
pub type W = crate::W<Confr0Spec>;
#[doc = "Field `START` writer - Start This bit start or stop the encoding or decoding process. Read this register always return 0."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Start This bit start or stop the encoding or decoding process. Read this register always return 0."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<Confr0Spec> {
        StartW::new(self, 0)
    }
}
#[doc = "JPEG codec control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Confr0Spec;
impl crate::RegisterSpec for Confr0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`confr0::W`](W) writer structure"]
impl crate::Writable for Confr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFR0 to value 0"]
impl crate::Resettable for Confr0Spec {}

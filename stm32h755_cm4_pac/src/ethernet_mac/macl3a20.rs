#[doc = "Register `MACL3A20` reader"]
pub type R = crate::R<Macl3a20Spec>;
#[doc = "Register `MACL3A20` writer"]
pub type W = crate::W<Macl3a20Spec>;
#[doc = "Field `L3A20` reader - L3A20"]
pub type L3a20R = crate::FieldReader<u32>;
#[doc = "Field `L3A20` writer - L3A20"]
pub type L3a20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A20"]
    #[inline(always)]
    pub fn l3a20(&self) -> L3a20R {
        L3a20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A20"]
    #[inline(always)]
    pub fn l3a20(&mut self) -> L3a20W<Macl3a20Spec> {
        L3a20W::new(self, 0)
    }
}
#[doc = "Layer3 Address 2 filter 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3a20Spec;
impl crate::RegisterSpec for Macl3a20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a20::R`](R) reader structure"]
impl crate::Readable for Macl3a20Spec {}
#[doc = "`write(|w| ..)` method takes [`macl3a20::W`](W) writer structure"]
impl crate::Writable for Macl3a20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3A20 to value 0"]
impl crate::Resettable for Macl3a20Spec {}

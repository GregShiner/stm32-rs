#[doc = "Register `MACL3A30` reader"]
pub type R = crate::R<Macl3a30Spec>;
#[doc = "Register `MACL3A30` writer"]
pub type W = crate::W<Macl3a30Spec>;
#[doc = "Field `L3A30` reader - L3A30"]
pub type L3a30R = crate::FieldReader<u32>;
#[doc = "Field `L3A30` writer - L3A30"]
pub type L3a30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A30"]
    #[inline(always)]
    pub fn l3a30(&self) -> L3a30R {
        L3a30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A30"]
    #[inline(always)]
    pub fn l3a30(&mut self) -> L3a30W<Macl3a30Spec> {
        L3a30W::new(self, 0)
    }
}
#[doc = "Layer3 Address 3 filter 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3a30Spec;
impl crate::RegisterSpec for Macl3a30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a30::R`](R) reader structure"]
impl crate::Readable for Macl3a30Spec {}
#[doc = "`write(|w| ..)` method takes [`macl3a30::W`](W) writer structure"]
impl crate::Writable for Macl3a30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3A30 to value 0"]
impl crate::Resettable for Macl3a30Spec {}

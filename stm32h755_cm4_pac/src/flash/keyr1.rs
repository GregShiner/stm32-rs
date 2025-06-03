#[doc = "Register `KEYR1` reader"]
pub type R = crate::R<Keyr1Spec>;
#[doc = "Register `KEYR1` writer"]
pub type W = crate::W<Keyr1Spec>;
#[doc = "Field `KEYR1` reader - Bank 1 access configuration unlock key"]
pub type Keyr1R = crate::FieldReader<u32>;
#[doc = "Field `KEYR1` writer - Bank 1 access configuration unlock key"]
pub type Keyr1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bank 1 access configuration unlock key"]
    #[inline(always)]
    pub fn keyr1(&self) -> Keyr1R {
        Keyr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bank 1 access configuration unlock key"]
    #[inline(always)]
    pub fn keyr1(&mut self) -> Keyr1W<Keyr1Spec> {
        Keyr1W::new(self, 0)
    }
}
#[doc = "FLASH key register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Keyr1Spec;
impl crate::RegisterSpec for Keyr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`keyr1::R`](R) reader structure"]
impl crate::Readable for Keyr1Spec {}
#[doc = "`write(|w| ..)` method takes [`keyr1::W`](W) writer structure"]
impl crate::Writable for Keyr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets KEYR1 to value 0"]
impl crate::Resettable for Keyr1Spec {}

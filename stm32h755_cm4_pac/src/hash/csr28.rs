#[doc = "Register `CSR28` reader"]
pub type R = crate::R<Csr28Spec>;
#[doc = "Register `CSR28` writer"]
pub type W = crate::W<Csr28Spec>;
#[doc = "Field `CSR28` reader - CSR28"]
pub type Csr28R = crate::FieldReader<u32>;
#[doc = "Field `CSR28` writer - CSR28"]
pub type Csr28W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR28"]
    #[inline(always)]
    pub fn csr28(&self) -> Csr28R {
        Csr28R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR28"]
    #[inline(always)]
    pub fn csr28(&mut self) -> Csr28W<Csr28Spec> {
        Csr28W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr28Spec;
impl crate::RegisterSpec for Csr28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr28::R`](R) reader structure"]
impl crate::Readable for Csr28Spec {}
#[doc = "`write(|w| ..)` method takes [`csr28::W`](W) writer structure"]
impl crate::Writable for Csr28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR28 to value 0"]
impl crate::Resettable for Csr28Spec {}

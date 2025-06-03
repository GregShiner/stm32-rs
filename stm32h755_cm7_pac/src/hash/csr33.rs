#[doc = "Register `CSR33` reader"]
pub type R = crate::R<Csr33Spec>;
#[doc = "Register `CSR33` writer"]
pub type W = crate::W<Csr33Spec>;
#[doc = "Field `CSR33` reader - CSR33"]
pub type Csr33R = crate::FieldReader<u32>;
#[doc = "Field `CSR33` writer - CSR33"]
pub type Csr33W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR33"]
    #[inline(always)]
    pub fn csr33(&self) -> Csr33R {
        Csr33R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR33"]
    #[inline(always)]
    pub fn csr33(&mut self) -> Csr33W<Csr33Spec> {
        Csr33W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr33::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr33::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr33Spec;
impl crate::RegisterSpec for Csr33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr33::R`](R) reader structure"]
impl crate::Readable for Csr33Spec {}
#[doc = "`write(|w| ..)` method takes [`csr33::W`](W) writer structure"]
impl crate::Writable for Csr33Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR33 to value 0"]
impl crate::Resettable for Csr33Spec {}

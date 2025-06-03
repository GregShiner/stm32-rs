#[doc = "Register `CSR17` reader"]
pub type R = crate::R<Csr17Spec>;
#[doc = "Register `CSR17` writer"]
pub type W = crate::W<Csr17Spec>;
#[doc = "Field `CSR17` reader - CSR17"]
pub type Csr17R = crate::FieldReader<u32>;
#[doc = "Field `CSR17` writer - CSR17"]
pub type Csr17W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR17"]
    #[inline(always)]
    pub fn csr17(&self) -> Csr17R {
        Csr17R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR17"]
    #[inline(always)]
    pub fn csr17(&mut self) -> Csr17W<Csr17Spec> {
        Csr17W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr17Spec;
impl crate::RegisterSpec for Csr17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr17::R`](R) reader structure"]
impl crate::Readable for Csr17Spec {}
#[doc = "`write(|w| ..)` method takes [`csr17::W`](W) writer structure"]
impl crate::Writable for Csr17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR17 to value 0"]
impl crate::Resettable for Csr17Spec {}

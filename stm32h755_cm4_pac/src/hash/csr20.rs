#[doc = "Register `CSR20` reader"]
pub type R = crate::R<Csr20Spec>;
#[doc = "Register `CSR20` writer"]
pub type W = crate::W<Csr20Spec>;
#[doc = "Field `CSR20` reader - CSR20"]
pub type Csr20R = crate::FieldReader<u32>;
#[doc = "Field `CSR20` writer - CSR20"]
pub type Csr20W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR20"]
    #[inline(always)]
    pub fn csr20(&self) -> Csr20R {
        Csr20R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR20"]
    #[inline(always)]
    pub fn csr20(&mut self) -> Csr20W<Csr20Spec> {
        Csr20W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr20Spec;
impl crate::RegisterSpec for Csr20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr20::R`](R) reader structure"]
impl crate::Readable for Csr20Spec {}
#[doc = "`write(|w| ..)` method takes [`csr20::W`](W) writer structure"]
impl crate::Writable for Csr20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR20 to value 0"]
impl crate::Resettable for Csr20Spec {}

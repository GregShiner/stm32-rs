#[doc = "Register `CSR46` reader"]
pub type R = crate::R<Csr46Spec>;
#[doc = "Register `CSR46` writer"]
pub type W = crate::W<Csr46Spec>;
#[doc = "Field `CSR46` reader - CSR46"]
pub type Csr46R = crate::FieldReader<u32>;
#[doc = "Field `CSR46` writer - CSR46"]
pub type Csr46W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR46"]
    #[inline(always)]
    pub fn csr46(&self) -> Csr46R {
        Csr46R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR46"]
    #[inline(always)]
    pub fn csr46(&mut self) -> Csr46W<Csr46Spec> {
        Csr46W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr46::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr46::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr46Spec;
impl crate::RegisterSpec for Csr46Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr46::R`](R) reader structure"]
impl crate::Readable for Csr46Spec {}
#[doc = "`write(|w| ..)` method takes [`csr46::W`](W) writer structure"]
impl crate::Writable for Csr46Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR46 to value 0"]
impl crate::Resettable for Csr46Spec {}

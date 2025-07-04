#[doc = "Register `CSR3` reader"]
pub type R = crate::R<Csr3Spec>;
#[doc = "Register `CSR3` writer"]
pub type W = crate::W<Csr3Spec>;
#[doc = "Field `CSR3` reader - CSR3"]
pub type Csr3R = crate::FieldReader<u32>;
#[doc = "Field `CSR3` writer - CSR3"]
pub type Csr3W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR3"]
    #[inline(always)]
    pub fn csr3(&self) -> Csr3R {
        Csr3R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR3"]
    #[inline(always)]
    pub fn csr3(&mut self) -> Csr3W<Csr3Spec> {
        Csr3W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr3Spec;
impl crate::RegisterSpec for Csr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr3::R`](R) reader structure"]
impl crate::Readable for Csr3Spec {}
#[doc = "`write(|w| ..)` method takes [`csr3::W`](W) writer structure"]
impl crate::Writable for Csr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR3 to value 0"]
impl crate::Resettable for Csr3Spec {}

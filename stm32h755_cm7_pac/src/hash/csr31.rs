#[doc = "Register `CSR31` reader"]
pub type R = crate::R<Csr31Spec>;
#[doc = "Register `CSR31` writer"]
pub type W = crate::W<Csr31Spec>;
#[doc = "Field `CSR31` reader - CSR31"]
pub type Csr31R = crate::FieldReader<u32>;
#[doc = "Field `CSR31` writer - CSR31"]
pub type Csr31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR31"]
    #[inline(always)]
    pub fn csr31(&self) -> Csr31R {
        Csr31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR31"]
    #[inline(always)]
    pub fn csr31(&mut self) -> Csr31W<Csr31Spec> {
        Csr31W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr31Spec;
impl crate::RegisterSpec for Csr31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr31::R`](R) reader structure"]
impl crate::Readable for Csr31Spec {}
#[doc = "`write(|w| ..)` method takes [`csr31::W`](W) writer structure"]
impl crate::Writable for Csr31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR31 to value 0"]
impl crate::Resettable for Csr31Spec {}

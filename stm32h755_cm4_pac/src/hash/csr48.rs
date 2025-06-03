#[doc = "Register `CSR48` reader"]
pub type R = crate::R<Csr48Spec>;
#[doc = "Register `CSR48` writer"]
pub type W = crate::W<Csr48Spec>;
#[doc = "Field `CSR48` reader - CSR48"]
pub type Csr48R = crate::FieldReader<u32>;
#[doc = "Field `CSR48` writer - CSR48"]
pub type Csr48W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR48"]
    #[inline(always)]
    pub fn csr48(&self) -> Csr48R {
        Csr48R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR48"]
    #[inline(always)]
    pub fn csr48(&mut self) -> Csr48W<Csr48Spec> {
        Csr48W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr48::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr48::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr48Spec;
impl crate::RegisterSpec for Csr48Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr48::R`](R) reader structure"]
impl crate::Readable for Csr48Spec {}
#[doc = "`write(|w| ..)` method takes [`csr48::W`](W) writer structure"]
impl crate::Writable for Csr48Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR48 to value 0"]
impl crate::Resettable for Csr48Spec {}

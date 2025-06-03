#[doc = "Register `CSR34` reader"]
pub type R = crate::R<Csr34Spec>;
#[doc = "Register `CSR34` writer"]
pub type W = crate::W<Csr34Spec>;
#[doc = "Field `CSR34` reader - CSR34"]
pub type Csr34R = crate::FieldReader<u32>;
#[doc = "Field `CSR34` writer - CSR34"]
pub type Csr34W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR34"]
    #[inline(always)]
    pub fn csr34(&self) -> Csr34R {
        Csr34R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR34"]
    #[inline(always)]
    pub fn csr34(&mut self) -> Csr34W<Csr34Spec> {
        Csr34W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr34::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr34::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr34Spec;
impl crate::RegisterSpec for Csr34Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr34::R`](R) reader structure"]
impl crate::Readable for Csr34Spec {}
#[doc = "`write(|w| ..)` method takes [`csr34::W`](W) writer structure"]
impl crate::Writable for Csr34Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR34 to value 0"]
impl crate::Resettable for Csr34Spec {}

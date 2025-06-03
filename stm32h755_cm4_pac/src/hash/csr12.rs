#[doc = "Register `CSR12` reader"]
pub type R = crate::R<Csr12Spec>;
#[doc = "Register `CSR12` writer"]
pub type W = crate::W<Csr12Spec>;
#[doc = "Field `CSR12` reader - CSR12"]
pub type Csr12R = crate::FieldReader<u32>;
#[doc = "Field `CSR12` writer - CSR12"]
pub type Csr12W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR12"]
    #[inline(always)]
    pub fn csr12(&self) -> Csr12R {
        Csr12R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR12"]
    #[inline(always)]
    pub fn csr12(&mut self) -> Csr12W<Csr12Spec> {
        Csr12W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr12Spec;
impl crate::RegisterSpec for Csr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr12::R`](R) reader structure"]
impl crate::Readable for Csr12Spec {}
#[doc = "`write(|w| ..)` method takes [`csr12::W`](W) writer structure"]
impl crate::Writable for Csr12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR12 to value 0"]
impl crate::Resettable for Csr12Spec {}

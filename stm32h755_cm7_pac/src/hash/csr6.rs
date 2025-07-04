#[doc = "Register `CSR6` reader"]
pub type R = crate::R<Csr6Spec>;
#[doc = "Register `CSR6` writer"]
pub type W = crate::W<Csr6Spec>;
#[doc = "Field `CSR6` reader - CSR6"]
pub type Csr6R = crate::FieldReader<u32>;
#[doc = "Field `CSR6` writer - CSR6"]
pub type Csr6W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR6"]
    #[inline(always)]
    pub fn csr6(&self) -> Csr6R {
        Csr6R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR6"]
    #[inline(always)]
    pub fn csr6(&mut self) -> Csr6W<Csr6Spec> {
        Csr6W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr6Spec;
impl crate::RegisterSpec for Csr6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr6::R`](R) reader structure"]
impl crate::Readable for Csr6Spec {}
#[doc = "`write(|w| ..)` method takes [`csr6::W`](W) writer structure"]
impl crate::Writable for Csr6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR6 to value 0"]
impl crate::Resettable for Csr6Spec {}

#[doc = "Register `CSR5` reader"]
pub type R = crate::R<Csr5Spec>;
#[doc = "Register `CSR5` writer"]
pub type W = crate::W<Csr5Spec>;
#[doc = "Field `CSR5` reader - CSR5"]
pub type Csr5R = crate::FieldReader<u32>;
#[doc = "Field `CSR5` writer - CSR5"]
pub type Csr5W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR5"]
    #[inline(always)]
    pub fn csr5(&self) -> Csr5R {
        Csr5R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR5"]
    #[inline(always)]
    pub fn csr5(&mut self) -> Csr5W<Csr5Spec> {
        Csr5W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr5Spec;
impl crate::RegisterSpec for Csr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr5::R`](R) reader structure"]
impl crate::Readable for Csr5Spec {}
#[doc = "`write(|w| ..)` method takes [`csr5::W`](W) writer structure"]
impl crate::Writable for Csr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR5 to value 0"]
impl crate::Resettable for Csr5Spec {}

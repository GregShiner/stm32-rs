#[doc = "Register `CSR26` reader"]
pub type R = crate::R<Csr26Spec>;
#[doc = "Register `CSR26` writer"]
pub type W = crate::W<Csr26Spec>;
#[doc = "Field `CSR26` reader - CSR26"]
pub type Csr26R = crate::FieldReader<u32>;
#[doc = "Field `CSR26` writer - CSR26"]
pub type Csr26W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR26"]
    #[inline(always)]
    pub fn csr26(&self) -> Csr26R {
        Csr26R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR26"]
    #[inline(always)]
    pub fn csr26(&mut self) -> Csr26W<Csr26Spec> {
        Csr26W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr26Spec;
impl crate::RegisterSpec for Csr26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr26::R`](R) reader structure"]
impl crate::Readable for Csr26Spec {}
#[doc = "`write(|w| ..)` method takes [`csr26::W`](W) writer structure"]
impl crate::Writable for Csr26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR26 to value 0"]
impl crate::Resettable for Csr26Spec {}

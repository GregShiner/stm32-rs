#[doc = "Register `CSR23` reader"]
pub type R = crate::R<Csr23Spec>;
#[doc = "Register `CSR23` writer"]
pub type W = crate::W<Csr23Spec>;
#[doc = "Field `CSR23` reader - CSR23"]
pub type Csr23R = crate::FieldReader<u32>;
#[doc = "Field `CSR23` writer - CSR23"]
pub type Csr23W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR23"]
    #[inline(always)]
    pub fn csr23(&self) -> Csr23R {
        Csr23R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR23"]
    #[inline(always)]
    pub fn csr23(&mut self) -> Csr23W<Csr23Spec> {
        Csr23W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr23Spec;
impl crate::RegisterSpec for Csr23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr23::R`](R) reader structure"]
impl crate::Readable for Csr23Spec {}
#[doc = "`write(|w| ..)` method takes [`csr23::W`](W) writer structure"]
impl crate::Writable for Csr23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR23 to value 0"]
impl crate::Resettable for Csr23Spec {}

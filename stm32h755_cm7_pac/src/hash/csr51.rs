#[doc = "Register `CSR51` reader"]
pub type R = crate::R<Csr51Spec>;
#[doc = "Register `CSR51` writer"]
pub type W = crate::W<Csr51Spec>;
#[doc = "Field `CSR51` reader - CSR51"]
pub type Csr51R = crate::FieldReader<u32>;
#[doc = "Field `CSR51` writer - CSR51"]
pub type Csr51W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR51"]
    #[inline(always)]
    pub fn csr51(&self) -> Csr51R {
        Csr51R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR51"]
    #[inline(always)]
    pub fn csr51(&mut self) -> Csr51W<Csr51Spec> {
        Csr51W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr51::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr51::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr51Spec;
impl crate::RegisterSpec for Csr51Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr51::R`](R) reader structure"]
impl crate::Readable for Csr51Spec {}
#[doc = "`write(|w| ..)` method takes [`csr51::W`](W) writer structure"]
impl crate::Writable for Csr51Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR51 to value 0"]
impl crate::Resettable for Csr51Spec {}

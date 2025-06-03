#[doc = "Register `CSR22` reader"]
pub type R = crate::R<Csr22Spec>;
#[doc = "Register `CSR22` writer"]
pub type W = crate::W<Csr22Spec>;
#[doc = "Field `CSR22` reader - CSR22"]
pub type Csr22R = crate::FieldReader<u32>;
#[doc = "Field `CSR22` writer - CSR22"]
pub type Csr22W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR22"]
    #[inline(always)]
    pub fn csr22(&self) -> Csr22R {
        Csr22R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR22"]
    #[inline(always)]
    pub fn csr22(&mut self) -> Csr22W<Csr22Spec> {
        Csr22W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr22Spec;
impl crate::RegisterSpec for Csr22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr22::R`](R) reader structure"]
impl crate::Readable for Csr22Spec {}
#[doc = "`write(|w| ..)` method takes [`csr22::W`](W) writer structure"]
impl crate::Writable for Csr22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR22 to value 0"]
impl crate::Resettable for Csr22Spec {}

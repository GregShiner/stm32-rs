#[doc = "Register `CSR50` reader"]
pub type R = crate::R<Csr50Spec>;
#[doc = "Register `CSR50` writer"]
pub type W = crate::W<Csr50Spec>;
#[doc = "Field `CSR50` reader - CSR50"]
pub type Csr50R = crate::FieldReader<u32>;
#[doc = "Field `CSR50` writer - CSR50"]
pub type Csr50W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR50"]
    #[inline(always)]
    pub fn csr50(&self) -> Csr50R {
        Csr50R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR50"]
    #[inline(always)]
    pub fn csr50(&mut self) -> Csr50W<Csr50Spec> {
        Csr50W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr50::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr50::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr50Spec;
impl crate::RegisterSpec for Csr50Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr50::R`](R) reader structure"]
impl crate::Readable for Csr50Spec {}
#[doc = "`write(|w| ..)` method takes [`csr50::W`](W) writer structure"]
impl crate::Writable for Csr50Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR50 to value 0"]
impl crate::Resettable for Csr50Spec {}

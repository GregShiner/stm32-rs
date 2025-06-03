#[doc = "Register `CSR27` reader"]
pub type R = crate::R<Csr27Spec>;
#[doc = "Register `CSR27` writer"]
pub type W = crate::W<Csr27Spec>;
#[doc = "Field `CSR27` reader - CSR27"]
pub type Csr27R = crate::FieldReader<u32>;
#[doc = "Field `CSR27` writer - CSR27"]
pub type Csr27W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR27"]
    #[inline(always)]
    pub fn csr27(&self) -> Csr27R {
        Csr27R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR27"]
    #[inline(always)]
    pub fn csr27(&mut self) -> Csr27W<Csr27Spec> {
        Csr27W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr27Spec;
impl crate::RegisterSpec for Csr27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr27::R`](R) reader structure"]
impl crate::Readable for Csr27Spec {}
#[doc = "`write(|w| ..)` method takes [`csr27::W`](W) writer structure"]
impl crate::Writable for Csr27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR27 to value 0"]
impl crate::Resettable for Csr27Spec {}

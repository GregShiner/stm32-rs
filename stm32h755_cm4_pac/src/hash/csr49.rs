#[doc = "Register `CSR49` reader"]
pub type R = crate::R<Csr49Spec>;
#[doc = "Register `CSR49` writer"]
pub type W = crate::W<Csr49Spec>;
#[doc = "Field `CSR49` reader - CSR49"]
pub type Csr49R = crate::FieldReader<u32>;
#[doc = "Field `CSR49` writer - CSR49"]
pub type Csr49W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR49"]
    #[inline(always)]
    pub fn csr49(&self) -> Csr49R {
        Csr49R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR49"]
    #[inline(always)]
    pub fn csr49(&mut self) -> Csr49W<Csr49Spec> {
        Csr49W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr49::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr49::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr49Spec;
impl crate::RegisterSpec for Csr49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr49::R`](R) reader structure"]
impl crate::Readable for Csr49Spec {}
#[doc = "`write(|w| ..)` method takes [`csr49::W`](W) writer structure"]
impl crate::Writable for Csr49Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR49 to value 0"]
impl crate::Resettable for Csr49Spec {}

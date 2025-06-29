#[doc = "Register `CSR47` reader"]
pub type R = crate::R<Csr47Spec>;
#[doc = "Register `CSR47` writer"]
pub type W = crate::W<Csr47Spec>;
#[doc = "Field `CSR47` reader - CSR47"]
pub type Csr47R = crate::FieldReader<u32>;
#[doc = "Field `CSR47` writer - CSR47"]
pub type Csr47W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR47"]
    #[inline(always)]
    pub fn csr47(&self) -> Csr47R {
        Csr47R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR47"]
    #[inline(always)]
    pub fn csr47(&mut self) -> Csr47W<Csr47Spec> {
        Csr47W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr47::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr47::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr47Spec;
impl crate::RegisterSpec for Csr47Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr47::R`](R) reader structure"]
impl crate::Readable for Csr47Spec {}
#[doc = "`write(|w| ..)` method takes [`csr47::W`](W) writer structure"]
impl crate::Writable for Csr47Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR47 to value 0"]
impl crate::Resettable for Csr47Spec {}

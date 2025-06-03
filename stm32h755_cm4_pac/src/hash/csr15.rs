#[doc = "Register `CSR15` reader"]
pub type R = crate::R<Csr15Spec>;
#[doc = "Register `CSR15` writer"]
pub type W = crate::W<Csr15Spec>;
#[doc = "Field `CSR15` reader - CSR15"]
pub type Csr15R = crate::FieldReader<u32>;
#[doc = "Field `CSR15` writer - CSR15"]
pub type Csr15W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR15"]
    #[inline(always)]
    pub fn csr15(&self) -> Csr15R {
        Csr15R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR15"]
    #[inline(always)]
    pub fn csr15(&mut self) -> Csr15W<Csr15Spec> {
        Csr15W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr15Spec;
impl crate::RegisterSpec for Csr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr15::R`](R) reader structure"]
impl crate::Readable for Csr15Spec {}
#[doc = "`write(|w| ..)` method takes [`csr15::W`](W) writer structure"]
impl crate::Writable for Csr15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR15 to value 0"]
impl crate::Resettable for Csr15Spec {}

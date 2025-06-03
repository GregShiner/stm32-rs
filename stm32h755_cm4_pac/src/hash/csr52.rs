#[doc = "Register `CSR52` reader"]
pub type R = crate::R<Csr52Spec>;
#[doc = "Register `CSR52` writer"]
pub type W = crate::W<Csr52Spec>;
#[doc = "Field `CSR52` reader - CSR52"]
pub type Csr52R = crate::FieldReader<u32>;
#[doc = "Field `CSR52` writer - CSR52"]
pub type Csr52W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR52"]
    #[inline(always)]
    pub fn csr52(&self) -> Csr52R {
        Csr52R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR52"]
    #[inline(always)]
    pub fn csr52(&mut self) -> Csr52W<Csr52Spec> {
        Csr52W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr52::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr52::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr52Spec;
impl crate::RegisterSpec for Csr52Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr52::R`](R) reader structure"]
impl crate::Readable for Csr52Spec {}
#[doc = "`write(|w| ..)` method takes [`csr52::W`](W) writer structure"]
impl crate::Writable for Csr52Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR52 to value 0"]
impl crate::Resettable for Csr52Spec {}

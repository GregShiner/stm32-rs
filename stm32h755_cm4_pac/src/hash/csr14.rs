#[doc = "Register `CSR14` reader"]
pub type R = crate::R<Csr14Spec>;
#[doc = "Register `CSR14` writer"]
pub type W = crate::W<Csr14Spec>;
#[doc = "Field `CSR14` reader - CSR14"]
pub type Csr14R = crate::FieldReader<u32>;
#[doc = "Field `CSR14` writer - CSR14"]
pub type Csr14W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR14"]
    #[inline(always)]
    pub fn csr14(&self) -> Csr14R {
        Csr14R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR14"]
    #[inline(always)]
    pub fn csr14(&mut self) -> Csr14W<Csr14Spec> {
        Csr14W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr14Spec;
impl crate::RegisterSpec for Csr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr14::R`](R) reader structure"]
impl crate::Readable for Csr14Spec {}
#[doc = "`write(|w| ..)` method takes [`csr14::W`](W) writer structure"]
impl crate::Writable for Csr14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR14 to value 0"]
impl crate::Resettable for Csr14Spec {}

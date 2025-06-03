#[doc = "Register `CSR40` reader"]
pub type R = crate::R<Csr40Spec>;
#[doc = "Register `CSR40` writer"]
pub type W = crate::W<Csr40Spec>;
#[doc = "Field `CSR40` reader - CSR40"]
pub type Csr40R = crate::FieldReader<u32>;
#[doc = "Field `CSR40` writer - CSR40"]
pub type Csr40W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR40"]
    #[inline(always)]
    pub fn csr40(&self) -> Csr40R {
        Csr40R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR40"]
    #[inline(always)]
    pub fn csr40(&mut self) -> Csr40W<Csr40Spec> {
        Csr40W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr40::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr40::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr40Spec;
impl crate::RegisterSpec for Csr40Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr40::R`](R) reader structure"]
impl crate::Readable for Csr40Spec {}
#[doc = "`write(|w| ..)` method takes [`csr40::W`](W) writer structure"]
impl crate::Writable for Csr40Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR40 to value 0"]
impl crate::Resettable for Csr40Spec {}

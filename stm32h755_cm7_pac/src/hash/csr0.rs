#[doc = "Register `CSR0` reader"]
pub type R = crate::R<Csr0Spec>;
#[doc = "Register `CSR0` writer"]
pub type W = crate::W<Csr0Spec>;
#[doc = "Field `CSR0` reader - CSR0"]
pub type Csr0R = crate::FieldReader<u32>;
#[doc = "Field `CSR0` writer - CSR0"]
pub type Csr0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR0"]
    #[inline(always)]
    pub fn csr0(&self) -> Csr0R {
        Csr0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR0"]
    #[inline(always)]
    pub fn csr0(&mut self) -> Csr0W<Csr0Spec> {
        Csr0W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr0Spec;
impl crate::RegisterSpec for Csr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr0::R`](R) reader structure"]
impl crate::Readable for Csr0Spec {}
#[doc = "`write(|w| ..)` method takes [`csr0::W`](W) writer structure"]
impl crate::Writable for Csr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR0 to value 0"]
impl crate::Resettable for Csr0Spec {}

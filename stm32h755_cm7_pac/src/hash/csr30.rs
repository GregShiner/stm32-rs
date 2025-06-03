#[doc = "Register `CSR30` reader"]
pub type R = crate::R<Csr30Spec>;
#[doc = "Register `CSR30` writer"]
pub type W = crate::W<Csr30Spec>;
#[doc = "Field `CSR30` reader - CSR30"]
pub type Csr30R = crate::FieldReader<u32>;
#[doc = "Field `CSR30` writer - CSR30"]
pub type Csr30W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR30"]
    #[inline(always)]
    pub fn csr30(&self) -> Csr30R {
        Csr30R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR30"]
    #[inline(always)]
    pub fn csr30(&mut self) -> Csr30W<Csr30Spec> {
        Csr30W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr30Spec;
impl crate::RegisterSpec for Csr30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr30::R`](R) reader structure"]
impl crate::Readable for Csr30Spec {}
#[doc = "`write(|w| ..)` method takes [`csr30::W`](W) writer structure"]
impl crate::Writable for Csr30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR30 to value 0"]
impl crate::Resettable for Csr30Spec {}

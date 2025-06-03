#[doc = "Register `CSR2` reader"]
pub type R = crate::R<Csr2Spec>;
#[doc = "Register `CSR2` writer"]
pub type W = crate::W<Csr2Spec>;
#[doc = "Field `CSR2` reader - CSR2"]
pub type Csr2R = crate::FieldReader<u32>;
#[doc = "Field `CSR2` writer - CSR2"]
pub type Csr2W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR2"]
    #[inline(always)]
    pub fn csr2(&self) -> Csr2R {
        Csr2R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR2"]
    #[inline(always)]
    pub fn csr2(&mut self) -> Csr2W<Csr2Spec> {
        Csr2W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr2Spec;
impl crate::RegisterSpec for Csr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr2::R`](R) reader structure"]
impl crate::Readable for Csr2Spec {}
#[doc = "`write(|w| ..)` method takes [`csr2::W`](W) writer structure"]
impl crate::Writable for Csr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR2 to value 0"]
impl crate::Resettable for Csr2Spec {}

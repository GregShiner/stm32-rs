#[doc = "Register `CSR18` reader"]
pub type R = crate::R<Csr18Spec>;
#[doc = "Register `CSR18` writer"]
pub type W = crate::W<Csr18Spec>;
#[doc = "Field `CSR18` reader - CSR18"]
pub type Csr18R = crate::FieldReader<u32>;
#[doc = "Field `CSR18` writer - CSR18"]
pub type Csr18W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR18"]
    #[inline(always)]
    pub fn csr18(&self) -> Csr18R {
        Csr18R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR18"]
    #[inline(always)]
    pub fn csr18(&mut self) -> Csr18W<Csr18Spec> {
        Csr18W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr18Spec;
impl crate::RegisterSpec for Csr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr18::R`](R) reader structure"]
impl crate::Readable for Csr18Spec {}
#[doc = "`write(|w| ..)` method takes [`csr18::W`](W) writer structure"]
impl crate::Writable for Csr18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR18 to value 0"]
impl crate::Resettable for Csr18Spec {}

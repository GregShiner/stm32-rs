#[doc = "Register `CSR43` reader"]
pub type R = crate::R<Csr43Spec>;
#[doc = "Register `CSR43` writer"]
pub type W = crate::W<Csr43Spec>;
#[doc = "Field `CSR43` reader - CSR43"]
pub type Csr43R = crate::FieldReader<u32>;
#[doc = "Field `CSR43` writer - CSR43"]
pub type Csr43W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR43"]
    #[inline(always)]
    pub fn csr43(&self) -> Csr43R {
        Csr43R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR43"]
    #[inline(always)]
    pub fn csr43(&mut self) -> Csr43W<Csr43Spec> {
        Csr43W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr43::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr43::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr43Spec;
impl crate::RegisterSpec for Csr43Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr43::R`](R) reader structure"]
impl crate::Readable for Csr43Spec {}
#[doc = "`write(|w| ..)` method takes [`csr43::W`](W) writer structure"]
impl crate::Writable for Csr43Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR43 to value 0"]
impl crate::Resettable for Csr43Spec {}

#[doc = "Register `CSR19` reader"]
pub type R = crate::R<Csr19Spec>;
#[doc = "Register `CSR19` writer"]
pub type W = crate::W<Csr19Spec>;
#[doc = "Field `CSR19` reader - CSR19"]
pub type Csr19R = crate::FieldReader<u32>;
#[doc = "Field `CSR19` writer - CSR19"]
pub type Csr19W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR19"]
    #[inline(always)]
    pub fn csr19(&self) -> Csr19R {
        Csr19R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR19"]
    #[inline(always)]
    pub fn csr19(&mut self) -> Csr19W<Csr19Spec> {
        Csr19W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr19Spec;
impl crate::RegisterSpec for Csr19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr19::R`](R) reader structure"]
impl crate::Readable for Csr19Spec {}
#[doc = "`write(|w| ..)` method takes [`csr19::W`](W) writer structure"]
impl crate::Writable for Csr19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR19 to value 0"]
impl crate::Resettable for Csr19Spec {}

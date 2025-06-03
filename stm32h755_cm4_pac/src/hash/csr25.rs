#[doc = "Register `CSR25` reader"]
pub type R = crate::R<Csr25Spec>;
#[doc = "Register `CSR25` writer"]
pub type W = crate::W<Csr25Spec>;
#[doc = "Field `CSR25` reader - CSR25"]
pub type Csr25R = crate::FieldReader<u32>;
#[doc = "Field `CSR25` writer - CSR25"]
pub type Csr25W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    pub fn csr25(&self) -> Csr25R {
        Csr25R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR25"]
    #[inline(always)]
    pub fn csr25(&mut self) -> Csr25W<Csr25Spec> {
        Csr25W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr25Spec;
impl crate::RegisterSpec for Csr25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr25::R`](R) reader structure"]
impl crate::Readable for Csr25Spec {}
#[doc = "`write(|w| ..)` method takes [`csr25::W`](W) writer structure"]
impl crate::Writable for Csr25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR25 to value 0"]
impl crate::Resettable for Csr25Spec {}

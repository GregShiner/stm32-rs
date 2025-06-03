#[doc = "Register `CSR35` reader"]
pub type R = crate::R<Csr35Spec>;
#[doc = "Register `CSR35` writer"]
pub type W = crate::W<Csr35Spec>;
#[doc = "Field `CSR35` reader - CSR35"]
pub type Csr35R = crate::FieldReader<u32>;
#[doc = "Field `CSR35` writer - CSR35"]
pub type Csr35W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR35"]
    #[inline(always)]
    pub fn csr35(&self) -> Csr35R {
        Csr35R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR35"]
    #[inline(always)]
    pub fn csr35(&mut self) -> Csr35W<Csr35Spec> {
        Csr35W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr35::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr35::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr35Spec;
impl crate::RegisterSpec for Csr35Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr35::R`](R) reader structure"]
impl crate::Readable for Csr35Spec {}
#[doc = "`write(|w| ..)` method takes [`csr35::W`](W) writer structure"]
impl crate::Writable for Csr35Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR35 to value 0"]
impl crate::Resettable for Csr35Spec {}

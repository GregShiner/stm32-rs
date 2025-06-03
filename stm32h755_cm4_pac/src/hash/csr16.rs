#[doc = "Register `CSR16` reader"]
pub type R = crate::R<Csr16Spec>;
#[doc = "Register `CSR16` writer"]
pub type W = crate::W<Csr16Spec>;
#[doc = "Field `CSR16` reader - CSR16"]
pub type Csr16R = crate::FieldReader<u32>;
#[doc = "Field `CSR16` writer - CSR16"]
pub type Csr16W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR16"]
    #[inline(always)]
    pub fn csr16(&self) -> Csr16R {
        Csr16R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR16"]
    #[inline(always)]
    pub fn csr16(&mut self) -> Csr16W<Csr16Spec> {
        Csr16W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr16Spec;
impl crate::RegisterSpec for Csr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr16::R`](R) reader structure"]
impl crate::Readable for Csr16Spec {}
#[doc = "`write(|w| ..)` method takes [`csr16::W`](W) writer structure"]
impl crate::Writable for Csr16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR16 to value 0"]
impl crate::Resettable for Csr16Spec {}

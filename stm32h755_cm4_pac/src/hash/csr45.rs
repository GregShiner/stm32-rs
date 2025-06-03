#[doc = "Register `CSR45` reader"]
pub type R = crate::R<Csr45Spec>;
#[doc = "Register `CSR45` writer"]
pub type W = crate::W<Csr45Spec>;
#[doc = "Field `CSR45` reader - CSR45"]
pub type Csr45R = crate::FieldReader<u32>;
#[doc = "Field `CSR45` writer - CSR45"]
pub type Csr45W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CSR45"]
    #[inline(always)]
    pub fn csr45(&self) -> Csr45R {
        Csr45R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR45"]
    #[inline(always)]
    pub fn csr45(&mut self) -> Csr45W<Csr45Spec> {
        Csr45W::new(self, 0)
    }
}
#[doc = "context swap registers\n\nYou can [`read`](crate::Reg::read) this register and get [`csr45::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr45::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr45Spec;
impl crate::RegisterSpec for Csr45Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr45::R`](R) reader structure"]
impl crate::Readable for Csr45Spec {}
#[doc = "`write(|w| ..)` method takes [`csr45::W`](W) writer structure"]
impl crate::Writable for Csr45Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR45 to value 0"]
impl crate::Resettable for Csr45Spec {}

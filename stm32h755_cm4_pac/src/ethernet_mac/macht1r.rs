#[doc = "Register `MACHT1R` reader"]
pub type R = crate::R<Macht1rSpec>;
#[doc = "Register `MACHT1R` writer"]
pub type W = crate::W<Macht1rSpec>;
#[doc = "Field `HT63T32` reader - HT63T32"]
pub type Ht63t32R = crate::FieldReader<u32>;
#[doc = "Field `HT63T32` writer - HT63T32"]
pub type Ht63t32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HT63T32"]
    #[inline(always)]
    pub fn ht63t32(&self) -> Ht63t32R {
        Ht63t32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HT63T32"]
    #[inline(always)]
    pub fn ht63t32(&mut self) -> Ht63t32W<Macht1rSpec> {
        Ht63t32W::new(self, 0)
    }
}
#[doc = "Hash Table 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macht1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macht1rSpec;
impl crate::RegisterSpec for Macht1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macht1r::R`](R) reader structure"]
impl crate::Readable for Macht1rSpec {}
#[doc = "`write(|w| ..)` method takes [`macht1r::W`](W) writer structure"]
impl crate::Writable for Macht1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACHT1R to value 0"]
impl crate::Resettable for Macht1rSpec {}

#[doc = "Register `MACHT0R` reader"]
pub type R = crate::R<Macht0rSpec>;
#[doc = "Register `MACHT0R` writer"]
pub type W = crate::W<Macht0rSpec>;
#[doc = "Field `HT31T0` reader - HT31T0"]
pub type Ht31t0R = crate::FieldReader<u32>;
#[doc = "Field `HT31T0` writer - HT31T0"]
pub type Ht31t0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    pub fn ht31t0(&self) -> Ht31t0R {
        Ht31t0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HT31T0"]
    #[inline(always)]
    pub fn ht31t0(&mut self) -> Ht31t0W<Macht0rSpec> {
        Ht31t0W::new(self, 0)
    }
}
#[doc = "Hash Table 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macht0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macht0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macht0rSpec;
impl crate::RegisterSpec for Macht0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macht0r::R`](R) reader structure"]
impl crate::Readable for Macht0rSpec {}
#[doc = "`write(|w| ..)` method takes [`macht0r::W`](W) writer structure"]
impl crate::Writable for Macht0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACHT0R to value 0"]
impl crate::Resettable for Macht0rSpec {}

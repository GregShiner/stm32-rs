#[doc = "Register `MACL3A31R` reader"]
pub type R = crate::R<Macl3a31rSpec>;
#[doc = "Register `MACL3A31R` writer"]
pub type W = crate::W<Macl3a31rSpec>;
#[doc = "Field `L3A31` reader - L3A31"]
pub type L3a31R = crate::FieldReader<u32>;
#[doc = "Field `L3A31` writer - L3A31"]
pub type L3a31W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A31"]
    #[inline(always)]
    pub fn l3a31(&self) -> L3a31R {
        L3a31R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A31"]
    #[inline(always)]
    pub fn l3a31(&mut self) -> L3a31W<Macl3a31rSpec> {
        L3a31W::new(self, 0)
    }
}
#[doc = "Layer3 address 3 filter 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a31r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a31r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3a31rSpec;
impl crate::RegisterSpec for Macl3a31rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a31r::R`](R) reader structure"]
impl crate::Readable for Macl3a31rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl3a31r::W`](W) writer structure"]
impl crate::Writable for Macl3a31rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3A31R to value 0"]
impl crate::Resettable for Macl3a31rSpec {}

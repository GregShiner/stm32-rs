#[doc = "Register `MACL3A11R` reader"]
pub type R = crate::R<Macl3a11rSpec>;
#[doc = "Register `MACL3A11R` writer"]
pub type W = crate::W<Macl3a11rSpec>;
#[doc = "Field `L3A11` reader - L3A11"]
pub type L3a11R = crate::FieldReader<u32>;
#[doc = "Field `L3A11` writer - L3A11"]
pub type L3a11W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A11"]
    #[inline(always)]
    pub fn l3a11(&self) -> L3a11R {
        L3a11R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A11"]
    #[inline(always)]
    pub fn l3a11(&mut self) -> L3a11W<Macl3a11rSpec> {
        L3a11W::new(self, 0)
    }
}
#[doc = "Layer3 address 1 filter 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a11r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a11r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3a11rSpec;
impl crate::RegisterSpec for Macl3a11rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a11r::R`](R) reader structure"]
impl crate::Readable for Macl3a11rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl3a11r::W`](W) writer structure"]
impl crate::Writable for Macl3a11rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3A11R to value 0"]
impl crate::Resettable for Macl3a11rSpec {}

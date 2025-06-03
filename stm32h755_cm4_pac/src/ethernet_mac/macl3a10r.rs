#[doc = "Register `MACL3A10R` reader"]
pub type R = crate::R<Macl3a10rSpec>;
#[doc = "Register `MACL3A10R` writer"]
pub type W = crate::W<Macl3a10rSpec>;
#[doc = "Field `L3A10` reader - L3A10"]
pub type L3a10R = crate::FieldReader<u32>;
#[doc = "Field `L3A10` writer - L3A10"]
pub type L3a10W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A10"]
    #[inline(always)]
    pub fn l3a10(&self) -> L3a10R {
        L3a10R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A10"]
    #[inline(always)]
    pub fn l3a10(&mut self) -> L3a10W<Macl3a10rSpec> {
        L3a10W::new(self, 0)
    }
}
#[doc = "Layer3 address 1 filter 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a10r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a10r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3a10rSpec;
impl crate::RegisterSpec for Macl3a10rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a10r::R`](R) reader structure"]
impl crate::Readable for Macl3a10rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl3a10r::W`](W) writer structure"]
impl crate::Writable for Macl3a10rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3A10R to value 0"]
impl crate::Resettable for Macl3a10rSpec {}

#[doc = "Register `MACL3A21R` reader"]
pub type R = crate::R<Macl3a21rSpec>;
#[doc = "Register `MACL3A21R` writer"]
pub type W = crate::W<Macl3a21rSpec>;
#[doc = "Field `L3A21` reader - L3A21"]
pub type L3a21R = crate::FieldReader<u32>;
#[doc = "Field `L3A21` writer - L3A21"]
pub type L3a21W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A21"]
    #[inline(always)]
    pub fn l3a21(&self) -> L3a21R {
        L3a21R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A21"]
    #[inline(always)]
    pub fn l3a21(&mut self) -> L3a21W<Macl3a21rSpec> {
        L3a21W::new(self, 0)
    }
}
#[doc = "Layer3 address 2 filter 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a21r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a21r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3a21rSpec;
impl crate::RegisterSpec for Macl3a21rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a21r::R`](R) reader structure"]
impl crate::Readable for Macl3a21rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl3a21r::W`](W) writer structure"]
impl crate::Writable for Macl3a21rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3A21R to value 0"]
impl crate::Resettable for Macl3a21rSpec {}

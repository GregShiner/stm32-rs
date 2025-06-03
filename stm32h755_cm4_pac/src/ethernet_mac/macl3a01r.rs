#[doc = "Register `MACL3A01R` reader"]
pub type R = crate::R<Macl3a01rSpec>;
#[doc = "Register `MACL3A01R` writer"]
pub type W = crate::W<Macl3a01rSpec>;
#[doc = "Field `L3A01` reader - L3A01"]
pub type L3a01R = crate::FieldReader<u32>;
#[doc = "Field `L3A01` writer - L3A01"]
pub type L3a01W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A01"]
    #[inline(always)]
    pub fn l3a01(&self) -> L3a01R {
        L3a01R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A01"]
    #[inline(always)]
    pub fn l3a01(&mut self) -> L3a01W<Macl3a01rSpec> {
        L3a01W::new(self, 0)
    }
}
#[doc = "Layer3 address 0 filter 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a01r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a01r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3a01rSpec;
impl crate::RegisterSpec for Macl3a01rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a01r::R`](R) reader structure"]
impl crate::Readable for Macl3a01rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl3a01r::W`](W) writer structure"]
impl crate::Writable for Macl3a01rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3A01R to value 0"]
impl crate::Resettable for Macl3a01rSpec {}

#[doc = "Register `MACL3A00R` reader"]
pub type R = crate::R<Macl3a00rSpec>;
#[doc = "Register `MACL3A00R` writer"]
pub type W = crate::W<Macl3a00rSpec>;
#[doc = "Field `L3A00` reader - L3A00"]
pub type L3a00R = crate::FieldReader<u32>;
#[doc = "Field `L3A00` writer - L3A00"]
pub type L3a00W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - L3A00"]
    #[inline(always)]
    pub fn l3a00(&self) -> L3a00R {
        L3a00R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - L3A00"]
    #[inline(always)]
    pub fn l3a00(&mut self) -> L3a00W<Macl3a00rSpec> {
        L3a00W::new(self, 0)
    }
}
#[doc = "MACL3A00R\n\nYou can [`read`](crate::Reg::read) this register and get [`macl3a00r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl3a00r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl3a00rSpec;
impl crate::RegisterSpec for Macl3a00rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl3a00r::R`](R) reader structure"]
impl crate::Readable for Macl3a00rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl3a00r::W`](W) writer structure"]
impl crate::Writable for Macl3a00rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL3A00R to value 0"]
impl crate::Resettable for Macl3a00rSpec {}

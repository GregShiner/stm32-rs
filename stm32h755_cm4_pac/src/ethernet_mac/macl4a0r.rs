#[doc = "Register `MACL4A0R` reader"]
pub type R = crate::R<Macl4a0rSpec>;
#[doc = "Register `MACL4A0R` writer"]
pub type W = crate::W<Macl4a0rSpec>;
#[doc = "Field `L4SP0` reader - L4SP0"]
pub type L4sp0R = crate::FieldReader<u16>;
#[doc = "Field `L4SP0` writer - L4SP0"]
pub type L4sp0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `L4DP0` reader - L4DP0"]
pub type L4dp0R = crate::FieldReader<u16>;
#[doc = "Field `L4DP0` writer - L4DP0"]
pub type L4dp0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    pub fn l4sp0(&self) -> L4sp0R {
        L4sp0R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    pub fn l4dp0(&self) -> L4dp0R {
        L4dp0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP0"]
    #[inline(always)]
    pub fn l4sp0(&mut self) -> L4sp0W<Macl4a0rSpec> {
        L4sp0W::new(self, 0)
    }
    #[doc = "Bits 16:31 - L4DP0"]
    #[inline(always)]
    pub fn l4dp0(&mut self) -> L4dp0W<Macl4a0rSpec> {
        L4dp0W::new(self, 16)
    }
}
#[doc = "Layer4 address filter 0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl4a0r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a0r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl4a0rSpec;
impl crate::RegisterSpec for Macl4a0rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl4a0r::R`](R) reader structure"]
impl crate::Readable for Macl4a0rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl4a0r::W`](W) writer structure"]
impl crate::Writable for Macl4a0rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL4A0R to value 0"]
impl crate::Resettable for Macl4a0rSpec {}

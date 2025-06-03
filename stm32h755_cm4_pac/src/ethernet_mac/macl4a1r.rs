#[doc = "Register `MACL4A1R` reader"]
pub type R = crate::R<Macl4a1rSpec>;
#[doc = "Register `MACL4A1R` writer"]
pub type W = crate::W<Macl4a1rSpec>;
#[doc = "Field `L4SP1` reader - L4SP1"]
pub type L4sp1R = crate::FieldReader<u16>;
#[doc = "Field `L4SP1` writer - L4SP1"]
pub type L4sp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `L4DP1` reader - L4DP1"]
pub type L4dp1R = crate::FieldReader<u16>;
#[doc = "Field `L4DP1` writer - L4DP1"]
pub type L4dp1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - L4SP1"]
    #[inline(always)]
    pub fn l4sp1(&self) -> L4sp1R {
        L4sp1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - L4DP1"]
    #[inline(always)]
    pub fn l4dp1(&self) -> L4dp1R {
        L4dp1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - L4SP1"]
    #[inline(always)]
    pub fn l4sp1(&mut self) -> L4sp1W<Macl4a1rSpec> {
        L4sp1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - L4DP1"]
    #[inline(always)]
    pub fn l4dp1(&mut self) -> L4dp1W<Macl4a1rSpec> {
        L4dp1W::new(self, 16)
    }
}
#[doc = "Layer 4 address filter 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`macl4a1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macl4a1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Macl4a1rSpec;
impl crate::RegisterSpec for Macl4a1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macl4a1r::R`](R) reader structure"]
impl crate::Readable for Macl4a1rSpec {}
#[doc = "`write(|w| ..)` method takes [`macl4a1r::W`](W) writer structure"]
impl crate::Writable for Macl4a1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACL4A1R to value 0"]
impl crate::Resettable for Macl4a1rSpec {}

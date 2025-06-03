#[doc = "Register `ICFR` writer"]
pub type W = crate::W<IcfrSpec>;
#[doc = "Field `CC1IF` writer - Clear COMP channel 1 Interrupt Flag"]
pub type Cc1ifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CC2IF` writer - Clear COMP channel 2 Interrupt Flag"]
pub type Cc2ifW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 16 - Clear COMP channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1if(&mut self) -> Cc1ifW<IcfrSpec> {
        Cc1ifW::new(self, 16)
    }
    #[doc = "Bit 17 - Clear COMP channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2if(&mut self) -> Cc2ifW<IcfrSpec> {
        Cc2ifW::new(self, 17)
    }
}
#[doc = "Comparator interrupt clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcfrSpec;
impl crate::RegisterSpec for IcfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icfr::W`](W) writer structure"]
impl crate::Writable for IcfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICFR to value 0"]
impl crate::Resettable for IcfrSpec {}

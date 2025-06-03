#[doc = "Register `D2CFGR` reader"]
pub type R = crate::R<D2cfgrSpec>;
#[doc = "Register `D2CFGR` writer"]
pub type W = crate::W<D2cfgrSpec>;
#[doc = "Field `D2PPRE1` reader - D2 domain APB1 prescaler"]
pub type D2ppre1R = crate::FieldReader;
#[doc = "Field `D2PPRE1` writer - D2 domain APB1 prescaler"]
pub type D2ppre1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D2PPRE2` reader - D2 domain APB2 prescaler"]
pub type D2ppre2R = crate::FieldReader;
#[doc = "Field `D2PPRE2` writer - D2 domain APB2 prescaler"]
pub type D2ppre2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:6 - D2 domain APB1 prescaler"]
    #[inline(always)]
    pub fn d2ppre1(&self) -> D2ppre1R {
        D2ppre1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - D2 domain APB2 prescaler"]
    #[inline(always)]
    pub fn d2ppre2(&self) -> D2ppre2R {
        D2ppre2R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D2 domain APB1 prescaler"]
    #[inline(always)]
    pub fn d2ppre1(&mut self) -> D2ppre1W<D2cfgrSpec> {
        D2ppre1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - D2 domain APB2 prescaler"]
    #[inline(always)]
    pub fn d2ppre2(&mut self) -> D2ppre2W<D2cfgrSpec> {
        D2ppre2W::new(self, 8)
    }
}
#[doc = "RCC Domain 2 Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d2cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2cfgrSpec;
impl crate::RegisterSpec for D2cfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2cfgr::R`](R) reader structure"]
impl crate::Readable for D2cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`d2cfgr::W`](W) writer structure"]
impl crate::Writable for D2cfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D2CFGR to value 0"]
impl crate::Resettable for D2cfgrSpec {}

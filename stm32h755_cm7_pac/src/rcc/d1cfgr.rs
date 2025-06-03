#[doc = "Register `D1CFGR` reader"]
pub type R = crate::R<D1cfgrSpec>;
#[doc = "Register `D1CFGR` writer"]
pub type W = crate::W<D1cfgrSpec>;
#[doc = "Field `HPRE` reader - D1 domain AHB prescaler"]
pub type HpreR = crate::FieldReader;
#[doc = "Field `HPRE` writer - D1 domain AHB prescaler"]
pub type HpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `D1PPRE` reader - D1 domain APB3 prescaler"]
pub type D1ppreR = crate::FieldReader;
#[doc = "Field `D1PPRE` writer - D1 domain APB3 prescaler"]
pub type D1ppreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `D1CPRE` reader - D1 domain Core prescaler"]
pub type D1cpreR = crate::FieldReader;
#[doc = "Field `D1CPRE` writer - D1 domain Core prescaler"]
pub type D1cpreW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - D1 domain AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HpreR {
        HpreR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - D1 domain APB3 prescaler"]
    #[inline(always)]
    pub fn d1ppre(&self) -> D1ppreR {
        D1ppreR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - D1 domain Core prescaler"]
    #[inline(always)]
    pub fn d1cpre(&self) -> D1cpreR {
        D1cpreR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - D1 domain AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HpreW<D1cfgrSpec> {
        HpreW::new(self, 0)
    }
    #[doc = "Bits 4:6 - D1 domain APB3 prescaler"]
    #[inline(always)]
    pub fn d1ppre(&mut self) -> D1ppreW<D1cfgrSpec> {
        D1ppreW::new(self, 4)
    }
    #[doc = "Bits 8:11 - D1 domain Core prescaler"]
    #[inline(always)]
    pub fn d1cpre(&mut self) -> D1cpreW<D1cfgrSpec> {
        D1cpreW::new(self, 8)
    }
}
#[doc = "RCC Domain 1 Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1cfgrSpec;
impl crate::RegisterSpec for D1cfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1cfgr::R`](R) reader structure"]
impl crate::Readable for D1cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`d1cfgr::W`](W) writer structure"]
impl crate::Writable for D1cfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1CFGR to value 0"]
impl crate::Resettable for D1cfgrSpec {}

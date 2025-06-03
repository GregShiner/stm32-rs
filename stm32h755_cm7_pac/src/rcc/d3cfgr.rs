#[doc = "Register `D3CFGR` reader"]
pub type R = crate::R<D3cfgrSpec>;
#[doc = "Register `D3CFGR` writer"]
pub type W = crate::W<D3cfgrSpec>;
#[doc = "Field `D3PPRE` reader - D3 domain APB4 prescaler"]
pub type D3ppreR = crate::FieldReader;
#[doc = "Field `D3PPRE` writer - D3 domain APB4 prescaler"]
pub type D3ppreW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    pub fn d3ppre(&self) -> D3ppreR {
        D3ppreR::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - D3 domain APB4 prescaler"]
    #[inline(always)]
    pub fn d3ppre(&mut self) -> D3ppreW<D3cfgrSpec> {
        D3ppreW::new(self, 4)
    }
}
#[doc = "RCC Domain 3 Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3cfgrSpec;
impl crate::RegisterSpec for D3cfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3cfgr::R`](R) reader structure"]
impl crate::Readable for D3cfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`d3cfgr::W`](W) writer structure"]
impl crate::Writable for D3cfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3CFGR to value 0"]
impl crate::Resettable for D3cfgrSpec {}

#[doc = "Register `C1_APB3LPENR` reader"]
pub type R = crate::R<C1Apb3lpenrSpec>;
#[doc = "Register `C1_APB3LPENR` writer"]
pub type W = crate::W<C1Apb3lpenrSpec>;
#[doc = "Field `LTDCLPEN` reader - LTDC peripheral clock enable during CSleep mode"]
pub type LtdclpenR = crate::BitReader;
#[doc = "Field `LTDCLPEN` writer - LTDC peripheral clock enable during CSleep mode"]
pub type LtdclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG1LPEN` reader - WWDG1 Clock Enable During CSleep Mode"]
pub type Wwdg1lpenR = crate::BitReader;
#[doc = "Field `WWDG1LPEN` writer - WWDG1 Clock Enable During CSleep Mode"]
pub type Wwdg1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - LTDC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn ltdclpen(&self) -> LtdclpenR {
        LtdclpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn wwdg1lpen(&self) -> Wwdg1lpenR {
        Wwdg1lpenR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn ltdclpen(&mut self) -> LtdclpenW<C1Apb3lpenrSpec> {
        LtdclpenW::new(self, 3)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn wwdg1lpen(&mut self) -> Wwdg1lpenW<C1Apb3lpenrSpec> {
        Wwdg1lpenW::new(self, 6)
    }
}
#[doc = "RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Apb3lpenrSpec;
impl crate::RegisterSpec for C1Apb3lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb3lpenr::R`](R) reader structure"]
impl crate::Readable for C1Apb3lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1_apb3lpenr::W`](W) writer structure"]
impl crate::Writable for C1Apb3lpenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1_APB3LPENR to value 0"]
impl crate::Resettable for C1Apb3lpenrSpec {}

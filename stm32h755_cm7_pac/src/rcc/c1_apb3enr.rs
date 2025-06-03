#[doc = "Register `C1_APB3ENR` reader"]
pub type R = crate::R<C1Apb3enrSpec>;
#[doc = "Register `C1_APB3ENR` writer"]
pub type W = crate::W<C1Apb3enrSpec>;
#[doc = "Field `LTDCEN` reader - LTDC peripheral clock enable"]
pub type LtdcenR = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LTDC peripheral clock enable"]
pub type LtdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG1EN` reader - WWDG1 Clock Enable"]
pub type Wwdg1enR = crate::BitReader;
#[doc = "Field `WWDG1EN` writer - WWDG1 Clock Enable"]
pub type Wwdg1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - LTDC peripheral clock enable"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LtdcenR {
        LtdcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable"]
    #[inline(always)]
    pub fn wwdg1en(&self) -> Wwdg1enR {
        Wwdg1enR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC peripheral clock enable"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LtdcenW<C1Apb3enrSpec> {
        LtdcenW::new(self, 3)
    }
    #[doc = "Bit 6 - WWDG1 Clock Enable"]
    #[inline(always)]
    pub fn wwdg1en(&mut self) -> Wwdg1enW<C1Apb3enrSpec> {
        Wwdg1enW::new(self, 6)
    }
}
#[doc = "RCC APB3 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Apb3enrSpec;
impl crate::RegisterSpec for C1Apb3enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb3enr::R`](R) reader structure"]
impl crate::Readable for C1Apb3enrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1_apb3enr::W`](W) writer structure"]
impl crate::Writable for C1Apb3enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1_APB3ENR to value 0"]
impl crate::Resettable for C1Apb3enrSpec {}

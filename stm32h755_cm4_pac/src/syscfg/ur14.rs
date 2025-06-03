#[doc = "Register `UR14` reader"]
pub type R = crate::R<Ur14Spec>;
#[doc = "Register `UR14` writer"]
pub type W = crate::W<Ur14Spec>;
#[doc = "Field `D1STPRST` reader - D1 Stop Reset"]
pub type D1stprstR = crate::BitReader;
#[doc = "Field `D1STPRST` writer - D1 Stop Reset"]
pub type D1stprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2SBRST` reader - D2 Standby Reset"]
pub type D2sbrstR = crate::BitReader;
#[doc = "Field `D2SBRST` writer - D2 Standby Reset"]
pub type D2sbrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    pub fn d1stprst(&self) -> D1stprstR {
        D1stprstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - D2 Standby Reset"]
    #[inline(always)]
    pub fn d2sbrst(&self) -> D2sbrstR {
        D2sbrstR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 Stop Reset"]
    #[inline(always)]
    pub fn d1stprst(&mut self) -> D1stprstW<Ur14Spec> {
        D1stprstW::new(self, 0)
    }
    #[doc = "Bit 16 - D2 Standby Reset"]
    #[inline(always)]
    pub fn d2sbrst(&mut self) -> D2sbrstW<Ur14Spec> {
        D2sbrstW::new(self, 16)
    }
}
#[doc = "SYSCFG user register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`ur14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur14Spec;
impl crate::RegisterSpec for Ur14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur14::R`](R) reader structure"]
impl crate::Readable for Ur14Spec {}
#[doc = "`write(|w| ..)` method takes [`ur14::W`](W) writer structure"]
impl crate::Writable for Ur14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UR14 to value 0"]
impl crate::Resettable for Ur14Spec {}

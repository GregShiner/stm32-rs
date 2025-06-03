#[doc = "Register `UR15` reader"]
pub type R = crate::R<Ur15Spec>;
#[doc = "Register `UR15` writer"]
pub type W = crate::W<Ur15Spec>;
#[doc = "Field `D2STPRST` reader - D2 Stop Reset"]
pub type D2stprstR = crate::BitReader;
#[doc = "Field `D2STPRST` writer - D2 Stop Reset"]
pub type D2stprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FZIWDGSTB` reader - Freeze independent watchdog in Standby mode"]
pub type FziwdgstbR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - D2 Stop Reset"]
    #[inline(always)]
    pub fn d2stprst(&self) -> D2stprstR {
        D2stprstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Freeze independent watchdog in Standby mode"]
    #[inline(always)]
    pub fn fziwdgstb(&self) -> FziwdgstbR {
        FziwdgstbR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D2 Stop Reset"]
    #[inline(always)]
    pub fn d2stprst(&mut self) -> D2stprstW<Ur15Spec> {
        D2stprstW::new(self, 0)
    }
}
#[doc = "SYSCFG user register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`ur15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur15Spec;
impl crate::RegisterSpec for Ur15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur15::R`](R) reader structure"]
impl crate::Readable for Ur15Spec {}
#[doc = "`write(|w| ..)` method takes [`ur15::W`](W) writer structure"]
impl crate::Writable for Ur15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UR15 to value 0"]
impl crate::Resettable for Ur15Spec {}

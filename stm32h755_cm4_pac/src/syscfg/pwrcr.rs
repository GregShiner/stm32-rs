#[doc = "Register `PWRCR` reader"]
pub type R = crate::R<PwrcrSpec>;
#[doc = "Register `PWRCR` writer"]
pub type W = crate::W<PwrcrSpec>;
#[doc = "Field `ODEN` reader - Overdrive enable"]
pub type OdenR = crate::BitReader;
#[doc = "Field `ODEN` writer - Overdrive enable"]
pub type OdenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&self) -> OdenR {
        OdenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Overdrive enable"]
    #[inline(always)]
    pub fn oden(&mut self) -> OdenW<PwrcrSpec> {
        OdenW::new(self, 0)
    }
}
#[doc = "SYSCFG power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrcrSpec;
impl crate::RegisterSpec for PwrcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrcr::R`](R) reader structure"]
impl crate::Readable for PwrcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrcr::W`](W) writer structure"]
impl crate::Writable for PwrcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCR to value 0"]
impl crate::Resettable for PwrcrSpec {}

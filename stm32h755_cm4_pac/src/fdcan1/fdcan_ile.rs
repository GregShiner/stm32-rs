#[doc = "Register `FDCAN_ILE` reader"]
pub type R = crate::R<FdcanIleSpec>;
#[doc = "Register `FDCAN_ILE` writer"]
pub type W = crate::W<FdcanIleSpec>;
#[doc = "Field `EINT0` reader - Enable Interrupt Line 0"]
pub type Eint0R = crate::BitReader;
#[doc = "Field `EINT0` writer - Enable Interrupt Line 0"]
pub type Eint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINT1` reader - Enable Interrupt Line 1"]
pub type Eint1R = crate::BitReader;
#[doc = "Field `EINT1` writer - Enable Interrupt Line 1"]
pub type Eint1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    pub fn eint0(&self) -> Eint0R {
        Eint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    pub fn eint1(&self) -> Eint1R {
        Eint1R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Interrupt Line 0"]
    #[inline(always)]
    pub fn eint0(&mut self) -> Eint0W<FdcanIleSpec> {
        Eint0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Interrupt Line 1"]
    #[inline(always)]
    pub fn eint1(&mut self) -> Eint1W<FdcanIleSpec> {
        Eint1W::new(self, 1)
    }
}
#[doc = "FDCAN Interrupt Line Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ile::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ile::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanIleSpec;
impl crate::RegisterSpec for FdcanIleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ile::R`](R) reader structure"]
impl crate::Readable for FdcanIleSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ile::W`](W) writer structure"]
impl crate::Writable for FdcanIleSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_ILE to value 0"]
impl crate::Resettable for FdcanIleSpec {}

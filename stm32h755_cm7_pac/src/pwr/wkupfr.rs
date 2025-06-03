#[doc = "Register `WKUPFR` reader"]
pub type R = crate::R<WkupfrSpec>;
#[doc = "Register `WKUPFR` writer"]
pub type W = crate::W<WkupfrSpec>;
#[doc = "Field `WKUPF1` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf1R = crate::BitReader;
#[doc = "Field `WKUPF1` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF2` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf2R = crate::BitReader;
#[doc = "Field `WKUPF2` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF3` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf3R = crate::BitReader;
#[doc = "Field `WKUPF3` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF4` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf4R = crate::BitReader;
#[doc = "Field `WKUPF4` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF5` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf5R = crate::BitReader;
#[doc = "Field `WKUPF5` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WKUPF6` reader - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf6R = crate::BitReader;
#[doc = "Field `WKUPF6` writer - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
pub type Wkupf6W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf1(&self) -> Wkupf1R {
        Wkupf1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf2(&self) -> Wkupf2R {
        Wkupf2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf3(&self) -> Wkupf3R {
        Wkupf3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf4(&self) -> Wkupf4R {
        Wkupf4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf5(&self) -> Wkupf5R {
        Wkupf5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf6(&self) -> Wkupf6R {
        Wkupf6R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf1(&mut self) -> Wkupf1W<WkupfrSpec> {
        Wkupf1W::new(self, 0)
    }
    #[doc = "Bit 1 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf2(&mut self) -> Wkupf2W<WkupfrSpec> {
        Wkupf2W::new(self, 1)
    }
    #[doc = "Bit 2 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf3(&mut self) -> Wkupf3W<WkupfrSpec> {
        Wkupf3W::new(self, 2)
    }
    #[doc = "Bit 3 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf4(&mut self) -> Wkupf4W<WkupfrSpec> {
        Wkupf4W::new(self, 3)
    }
    #[doc = "Bit 4 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf5(&mut self) -> Wkupf5W<WkupfrSpec> {
        Wkupf5W::new(self, 4)
    }
    #[doc = "Bit 5 - Wakeup pin WKUPF flag. This bit is set by hardware and cleared only by a Reset pin or by setting the WKUPCn+1 bit in the PWR wakeup clear register (PWR_WKUPCR)."]
    #[inline(always)]
    pub fn wkupf6(&mut self) -> Wkupf6W<WkupfrSpec> {
        Wkupf6W::new(self, 5)
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wkupfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkupfrSpec;
impl crate::RegisterSpec for WkupfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkupfr::R`](R) reader structure"]
impl crate::Readable for WkupfrSpec {}
#[doc = "`write(|w| ..)` method takes [`wkupfr::W`](W) writer structure"]
impl crate::Writable for WkupfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WKUPFR to value 0"]
impl crate::Resettable for WkupfrSpec {}

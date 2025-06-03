#[doc = "Register `BDCR` reader"]
pub type R = crate::R<BdcrSpec>;
#[doc = "Register `BDCR` writer"]
pub type W = crate::W<BdcrSpec>;
#[doc = "Field `LSEON` reader - LSE oscillator enabled"]
pub type LseonR = crate::BitReader;
#[doc = "Field `LSEON` writer - LSE oscillator enabled"]
pub type LseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDY` reader - LSE oscillator ready"]
pub type LserdyR = crate::BitReader;
#[doc = "Field `LSERDY` writer - LSE oscillator ready"]
pub type LserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEBYP` reader - LSE oscillator bypass"]
pub type LsebypR = crate::BitReader;
#[doc = "Field `LSEBYP` writer - LSE oscillator bypass"]
pub type LsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSEDRV` reader - LSE oscillator driving capability"]
pub type LsedrvR = crate::FieldReader;
#[doc = "Field `LSEDRV` writer - LSE oscillator driving capability"]
pub type LsedrvW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LSECSSON` reader - LSE clock security system enable"]
pub type LsecssonR = crate::BitReader;
#[doc = "Field `LSECSSON` writer - LSE clock security system enable"]
pub type LsecssonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSD` reader - LSE clock security system failure detection"]
pub type LsecssdR = crate::BitReader;
#[doc = "Field `LSECSSD` writer - LSE clock security system failure detection"]
pub type LsecssdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCSRC` reader - RTC clock source selection"]
pub type RtcsrcR = crate::FieldReader;
#[doc = "Field `RTCSRC` writer - RTC clock source selection"]
pub type RtcsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTCEN` reader - RTC clock enable"]
pub type RtcenR = crate::BitReader;
#[doc = "Field `RTCEN` writer - RTC clock enable"]
pub type RtcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWRST` reader - VSwitch domain software reset"]
pub type VswrstR = crate::BitReader;
#[doc = "Field `VSWRST` writer - VSwitch domain software reset"]
pub type VswrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSE oscillator enabled"]
    #[inline(always)]
    pub fn lseon(&self) -> LseonR {
        LseonR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&self) -> LserdyR {
        LserdyR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&self) -> LsebypR {
        LsebypR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability"]
    #[inline(always)]
    pub fn lsedrv(&self) -> LsedrvR {
        LsedrvR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - LSE clock security system enable"]
    #[inline(always)]
    pub fn lsecsson(&self) -> LsecssonR {
        LsecssonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection"]
    #[inline(always)]
    pub fn lsecssd(&self) -> LsecssdR {
        LsecssdR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsrc(&self) -> RtcsrcR {
        RtcsrcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&self) -> RtcenR {
        RtcenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - VSwitch domain software reset"]
    #[inline(always)]
    pub fn vswrst(&self) -> VswrstR {
        VswrstR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSE oscillator enabled"]
    #[inline(always)]
    pub fn lseon(&mut self) -> LseonW<BdcrSpec> {
        LseonW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE oscillator ready"]
    #[inline(always)]
    pub fn lserdy(&mut self) -> LserdyW<BdcrSpec> {
        LserdyW::new(self, 1)
    }
    #[doc = "Bit 2 - LSE oscillator bypass"]
    #[inline(always)]
    pub fn lsebyp(&mut self) -> LsebypW<BdcrSpec> {
        LsebypW::new(self, 2)
    }
    #[doc = "Bits 3:4 - LSE oscillator driving capability"]
    #[inline(always)]
    pub fn lsedrv(&mut self) -> LsedrvW<BdcrSpec> {
        LsedrvW::new(self, 3)
    }
    #[doc = "Bit 5 - LSE clock security system enable"]
    #[inline(always)]
    pub fn lsecsson(&mut self) -> LsecssonW<BdcrSpec> {
        LsecssonW::new(self, 5)
    }
    #[doc = "Bit 6 - LSE clock security system failure detection"]
    #[inline(always)]
    pub fn lsecssd(&mut self) -> LsecssdW<BdcrSpec> {
        LsecssdW::new(self, 6)
    }
    #[doc = "Bits 8:9 - RTC clock source selection"]
    #[inline(always)]
    pub fn rtcsrc(&mut self) -> RtcsrcW<BdcrSpec> {
        RtcsrcW::new(self, 8)
    }
    #[doc = "Bit 15 - RTC clock enable"]
    #[inline(always)]
    pub fn rtcen(&mut self) -> RtcenW<BdcrSpec> {
        RtcenW::new(self, 15)
    }
    #[doc = "Bit 16 - VSwitch domain software reset"]
    #[inline(always)]
    pub fn vswrst(&mut self) -> VswrstW<BdcrSpec> {
        VswrstW::new(self, 16)
    }
}
#[doc = "RCC Backup Domain Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BdcrSpec;
impl crate::RegisterSpec for BdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdcr::R`](R) reader structure"]
impl crate::Readable for BdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`bdcr::W`](W) writer structure"]
impl crate::Writable for BdcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BDCR to value 0"]
impl crate::Resettable for BdcrSpec {}

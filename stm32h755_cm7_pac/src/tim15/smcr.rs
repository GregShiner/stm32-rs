#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SmcrSpec>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SmcrSpec>;
#[doc = "Field `SMS` reader - Slave mode selection"]
pub type SmsR = crate::FieldReader;
#[doc = "Field `SMS` writer - Slave mode selection"]
pub type SmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TS_2_0` reader - Trigger selection"]
pub type Ts2_0R = crate::FieldReader;
#[doc = "Field `TS_2_0` writer - Trigger selection"]
pub type Ts2_0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MsmR = crate::BitReader;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS_3` reader - Slave mode selection bit 3"]
pub type Sms3R = crate::BitReader;
#[doc = "Field `SMS_3` writer - Slave mode selection bit 3"]
pub type Sms3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS_4_3` reader - Trigger selection - bit 4:3"]
pub type Ts4_3R = crate::FieldReader;
#[doc = "Field `TS_4_3` writer - Trigger selection - bit 4:3"]
pub type Ts4_3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts_2_0(&self) -> Ts2_0R {
        Ts2_0R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MsmR {
        MsmR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection bit 3"]
    #[inline(always)]
    pub fn sms_3(&self) -> Sms3R {
        Sms3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection - bit 4:3"]
    #[inline(always)]
    pub fn ts_4_3(&self) -> Ts4_3R {
        Ts4_3R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection"]
    #[inline(always)]
    pub fn sms(&mut self) -> SmsW<SmcrSpec> {
        SmsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Trigger selection"]
    #[inline(always)]
    pub fn ts_2_0(&mut self) -> Ts2_0W<SmcrSpec> {
        Ts2_0W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&mut self) -> MsmW<SmcrSpec> {
        MsmW::new(self, 7)
    }
    #[doc = "Bit 16 - Slave mode selection bit 3"]
    #[inline(always)]
    pub fn sms_3(&mut self) -> Sms3W<SmcrSpec> {
        Sms3W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection - bit 4:3"]
    #[inline(always)]
    pub fn ts_4_3(&mut self) -> Ts4_3W<SmcrSpec> {
        Ts4_3W::new(self, 20)
    }
}
#[doc = "slave mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SmcrSpec;
impl crate::RegisterSpec for SmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SmcrSpec {}

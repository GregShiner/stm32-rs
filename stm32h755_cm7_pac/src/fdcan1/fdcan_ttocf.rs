#[doc = "Register `FDCAN_TTOCF` reader"]
pub type R = crate::R<FdcanTtocfSpec>;
#[doc = "Register `FDCAN_TTOCF` writer"]
pub type W = crate::W<FdcanTtocfSpec>;
#[doc = "Field `OM` reader - Operation Mode"]
pub type OmR = crate::FieldReader;
#[doc = "Field `OM` writer - Operation Mode"]
pub type OmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GEN` reader - Gap Enable"]
pub type GenR = crate::BitReader;
#[doc = "Field `GEN` writer - Gap Enable"]
pub type GenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TM` reader - Time Master"]
pub type TmR = crate::BitReader;
#[doc = "Field `TM` writer - Time Master"]
pub type TmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDSDL` reader - LD of Synchronization Deviation Limit"]
pub type LdsdlR = crate::FieldReader;
#[doc = "Field `LDSDL` writer - LD of Synchronization Deviation Limit"]
pub type LdsdlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IRTO` reader - Initial Reference Trigger Offset"]
pub type IrtoR = crate::FieldReader;
#[doc = "Field `IRTO` writer - Initial Reference Trigger Offset"]
pub type IrtoW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EECS` reader - Enable External Clock Synchronization"]
pub type EecsR = crate::BitReader;
#[doc = "Field `EECS` writer - Enable External Clock Synchronization"]
pub type EecsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWL` reader - Application Watchdog Limit"]
pub type AwlR = crate::FieldReader;
#[doc = "Field `AWL` writer - Application Watchdog Limit"]
pub type AwlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EGTF` reader - Enable Global Time Filtering"]
pub type EgtfR = crate::BitReader;
#[doc = "Field `EGTF` writer - Enable Global Time Filtering"]
pub type EgtfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC` reader - Enable Clock Calibration"]
pub type EccR = crate::BitReader;
#[doc = "Field `ECC` writer - Enable Clock Calibration"]
pub type EccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVTP` reader - Event Trigger Polarity"]
pub type EvtpR = crate::BitReader;
#[doc = "Field `EVTP` writer - Event Trigger Polarity"]
pub type EvtpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn om(&self) -> OmR {
        OmR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Gap Enable"]
    #[inline(always)]
    pub fn gen_(&self) -> GenR {
        GenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Time Master"]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit"]
    #[inline(always)]
    pub fn ldsdl(&self) -> LdsdlR {
        LdsdlR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset"]
    #[inline(always)]
    pub fn irto(&self) -> IrtoR {
        IrtoR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization"]
    #[inline(always)]
    pub fn eecs(&self) -> EecsR {
        EecsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit"]
    #[inline(always)]
    pub fn awl(&self) -> AwlR {
        AwlR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering"]
    #[inline(always)]
    pub fn egtf(&self) -> EgtfR {
        EgtfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable Clock Calibration"]
    #[inline(always)]
    pub fn ecc(&self) -> EccR {
        EccR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Event Trigger Polarity"]
    #[inline(always)]
    pub fn evtp(&self) -> EvtpR {
        EvtpR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operation Mode"]
    #[inline(always)]
    pub fn om(&mut self) -> OmW<FdcanTtocfSpec> {
        OmW::new(self, 0)
    }
    #[doc = "Bit 3 - Gap Enable"]
    #[inline(always)]
    pub fn gen_(&mut self) -> GenW<FdcanTtocfSpec> {
        GenW::new(self, 3)
    }
    #[doc = "Bit 4 - Time Master"]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<FdcanTtocfSpec> {
        TmW::new(self, 4)
    }
    #[doc = "Bits 5:7 - LD of Synchronization Deviation Limit"]
    #[inline(always)]
    pub fn ldsdl(&mut self) -> LdsdlW<FdcanTtocfSpec> {
        LdsdlW::new(self, 5)
    }
    #[doc = "Bits 8:14 - Initial Reference Trigger Offset"]
    #[inline(always)]
    pub fn irto(&mut self) -> IrtoW<FdcanTtocfSpec> {
        IrtoW::new(self, 8)
    }
    #[doc = "Bit 15 - Enable External Clock Synchronization"]
    #[inline(always)]
    pub fn eecs(&mut self) -> EecsW<FdcanTtocfSpec> {
        EecsW::new(self, 15)
    }
    #[doc = "Bits 16:23 - Application Watchdog Limit"]
    #[inline(always)]
    pub fn awl(&mut self) -> AwlW<FdcanTtocfSpec> {
        AwlW::new(self, 16)
    }
    #[doc = "Bit 24 - Enable Global Time Filtering"]
    #[inline(always)]
    pub fn egtf(&mut self) -> EgtfW<FdcanTtocfSpec> {
        EgtfW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable Clock Calibration"]
    #[inline(always)]
    pub fn ecc(&mut self) -> EccW<FdcanTtocfSpec> {
        EccW::new(self, 25)
    }
    #[doc = "Bit 26 - Event Trigger Polarity"]
    #[inline(always)]
    pub fn evtp(&mut self) -> EvtpW<FdcanTtocfSpec> {
        EvtpW::new(self, 26)
    }
}
#[doc = "FDCAN TT Operation Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttocf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttocf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtocfSpec;
impl crate::RegisterSpec for FdcanTtocfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttocf::R`](R) reader structure"]
impl crate::Readable for FdcanTtocfSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttocf::W`](W) writer structure"]
impl crate::Writable for FdcanTtocfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTOCF to value 0"]
impl crate::Resettable for FdcanTtocfSpec {}

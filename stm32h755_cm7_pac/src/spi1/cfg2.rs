#[doc = "Register `CFG2` reader"]
pub type R = crate::R<Cfg2Spec>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<Cfg2Spec>;
#[doc = "Field `MSSI` reader - Master SS Idleness"]
pub type MssiR = crate::FieldReader;
#[doc = "Field `MSSI` writer - Master SS Idleness"]
pub type MssiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MIDI` reader - Master Inter-Data Idleness"]
pub type MidiR = crate::FieldReader;
#[doc = "Field `MIDI` writer - Master Inter-Data Idleness"]
pub type MidiW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IOSWP` reader - Swap functionality of MISO and MOSI pins"]
pub type IoswpR = crate::BitReader;
#[doc = "Field `IOSWP` writer - Swap functionality of MISO and MOSI pins"]
pub type IoswpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMM` reader - SPI Communication Mode"]
pub type CommR = crate::FieldReader;
#[doc = "Field `COMM` writer - SPI Communication Mode"]
pub type CommW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SP` reader - Serial Protocol"]
pub type SpR = crate::FieldReader;
#[doc = "Field `SP` writer - Serial Protocol"]
pub type SpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MASTER` reader - SPI Master"]
pub type MasterR = crate::BitReader;
#[doc = "Field `MASTER` writer - SPI Master"]
pub type MasterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSBFRST` reader - Data frame format"]
pub type LsbfrstR = crate::BitReader;
#[doc = "Field `LSBFRST` writer - Data frame format"]
pub type LsbfrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CphaR = crate::BitReader;
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CpolR = crate::BitReader;
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSM` reader - Software management of SS signal input"]
pub type SsmR = crate::BitReader;
#[doc = "Field `SSM` writer - Software management of SS signal input"]
pub type SsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSIOP` reader - SS input/output polarity"]
pub type SsiopR = crate::BitReader;
#[doc = "Field `SSIOP` writer - SS input/output polarity"]
pub type SsiopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOE` reader - SS output enable"]
pub type SsoeR = crate::BitReader;
#[doc = "Field `SSOE` writer - SS output enable"]
pub type SsoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOM` reader - SS output management in master mode"]
pub type SsomR = crate::BitReader;
#[doc = "Field `SSOM` writer - SS output management in master mode"]
pub type SsomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AFCNTR` reader - Alternate function GPIOs control"]
pub type AfcntrR = crate::BitReader;
#[doc = "Field `AFCNTR` writer - Alternate function GPIOs control"]
pub type AfcntrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Master SS Idleness"]
    #[inline(always)]
    pub fn mssi(&self) -> MssiR {
        MssiR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Master Inter-Data Idleness"]
    #[inline(always)]
    pub fn midi(&self) -> MidiR {
        MidiR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Swap functionality of MISO and MOSI pins"]
    #[inline(always)]
    pub fn ioswp(&self) -> IoswpR {
        IoswpR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn comm(&self) -> CommR {
        CommR::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - Serial Protocol"]
    #[inline(always)]
    pub fn sp(&self) -> SpR {
        SpR::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn master(&self) -> MasterR {
        MasterR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Data frame format"]
    #[inline(always)]
    pub fn lsbfrst(&self) -> LsbfrstR {
        LsbfrstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Software management of SS signal input"]
    #[inline(always)]
    pub fn ssm(&self) -> SsmR {
        SsmR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn ssiop(&self) -> SsiopR {
        SsiopR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SsoeR {
        SsoeR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SS output management in master mode"]
    #[inline(always)]
    pub fn ssom(&self) -> SsomR {
        SsomR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Alternate function GPIOs control"]
    #[inline(always)]
    pub fn afcntr(&self) -> AfcntrR {
        AfcntrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Master SS Idleness"]
    #[inline(always)]
    pub fn mssi(&mut self) -> MssiW<Cfg2Spec> {
        MssiW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Master Inter-Data Idleness"]
    #[inline(always)]
    pub fn midi(&mut self) -> MidiW<Cfg2Spec> {
        MidiW::new(self, 4)
    }
    #[doc = "Bit 15 - Swap functionality of MISO and MOSI pins"]
    #[inline(always)]
    pub fn ioswp(&mut self) -> IoswpW<Cfg2Spec> {
        IoswpW::new(self, 15)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn comm(&mut self) -> CommW<Cfg2Spec> {
        CommW::new(self, 17)
    }
    #[doc = "Bits 19:21 - Serial Protocol"]
    #[inline(always)]
    pub fn sp(&mut self) -> SpW<Cfg2Spec> {
        SpW::new(self, 19)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn master(&mut self) -> MasterW<Cfg2Spec> {
        MasterW::new(self, 22)
    }
    #[doc = "Bit 23 - Data frame format"]
    #[inline(always)]
    pub fn lsbfrst(&mut self) -> LsbfrstW<Cfg2Spec> {
        LsbfrstW::new(self, 23)
    }
    #[doc = "Bit 24 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<Cfg2Spec> {
        CphaW::new(self, 24)
    }
    #[doc = "Bit 25 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<Cfg2Spec> {
        CpolW::new(self, 25)
    }
    #[doc = "Bit 26 - Software management of SS signal input"]
    #[inline(always)]
    pub fn ssm(&mut self) -> SsmW<Cfg2Spec> {
        SsmW::new(self, 26)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn ssiop(&mut self) -> SsiopW<Cfg2Spec> {
        SsiopW::new(self, 28)
    }
    #[doc = "Bit 29 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&mut self) -> SsoeW<Cfg2Spec> {
        SsoeW::new(self, 29)
    }
    #[doc = "Bit 30 - SS output management in master mode"]
    #[inline(always)]
    pub fn ssom(&mut self) -> SsomW<Cfg2Spec> {
        SsomW::new(self, 30)
    }
    #[doc = "Bit 31 - Alternate function GPIOs control"]
    #[inline(always)]
    pub fn afcntr(&mut self) -> AfcntrW<Cfg2Spec> {
        AfcntrW::new(self, 31)
    }
}
#[doc = "configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg2Spec;
impl crate::RegisterSpec for Cfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for Cfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for Cfg2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for Cfg2Spec {}

#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `SPE` reader - Serial Peripheral Enable"]
pub type SpeR = crate::BitReader;
#[doc = "Field `SPE` writer - Serial Peripheral Enable"]
pub type SpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MASRX` reader - Master automatic SUSP in Receive mode"]
pub type MasrxR = crate::BitReader;
#[doc = "Field `MASRX` writer - Master automatic SUSP in Receive mode"]
pub type MasrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSTART` reader - Master transfer start"]
pub type CstartR = crate::BitReader;
#[doc = "Field `CSUSP` writer - Master SUSPend request"]
pub type CsuspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDDIR` reader - Rx/Tx direction at Half-duplex mode"]
pub type HddirR = crate::BitReader;
#[doc = "Field `HDDIR` writer - Rx/Tx direction at Half-duplex mode"]
pub type HddirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSI` reader - Internal SS signal input level"]
pub type SsiR = crate::BitReader;
#[doc = "Field `SSI` writer - Internal SS signal input level"]
pub type SsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC33_17` reader - 32-bit CRC polynomial configuration"]
pub type Crc33_17R = crate::BitReader;
#[doc = "Field `CRC33_17` writer - 32-bit CRC polynomial configuration"]
pub type Crc33_17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCRCI` reader - CRC calculation initialization pattern control for receiver"]
pub type RcrciR = crate::BitReader;
#[doc = "Field `RCRCI` writer - CRC calculation initialization pattern control for receiver"]
pub type RcrciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCRCI` reader - CRC calculation initialization pattern control for transmitter"]
pub type TcrciR = crate::BitReader;
#[doc = "Field `TCRCI` writer - CRC calculation initialization pattern control for transmitter"]
pub type TcrciW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOLOCK` reader - Locking the AF configuration of associated IOs"]
pub type IolockR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Serial Peripheral Enable"]
    #[inline(always)]
    pub fn spe(&self) -> SpeR {
        SpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - Master automatic SUSP in Receive mode"]
    #[inline(always)]
    pub fn masrx(&self) -> MasrxR {
        MasrxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Master transfer start"]
    #[inline(always)]
    pub fn cstart(&self) -> CstartR {
        CstartR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    pub fn hddir(&self) -> HddirR {
        HddirR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Internal SS signal input level"]
    #[inline(always)]
    pub fn ssi(&self) -> SsiR {
        SsiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn crc33_17(&self) -> Crc33_17R {
        Crc33_17R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn rcrci(&self) -> RcrciR {
        RcrciR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn tcrci(&self) -> TcrciR {
        TcrciR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Locking the AF configuration of associated IOs"]
    #[inline(always)]
    pub fn iolock(&self) -> IolockR {
        IolockR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Serial Peripheral Enable"]
    #[inline(always)]
    pub fn spe(&mut self) -> SpeW<Cr1Spec> {
        SpeW::new(self, 0)
    }
    #[doc = "Bit 8 - Master automatic SUSP in Receive mode"]
    #[inline(always)]
    pub fn masrx(&mut self) -> MasrxW<Cr1Spec> {
        MasrxW::new(self, 8)
    }
    #[doc = "Bit 10 - Master SUSPend request"]
    #[inline(always)]
    pub fn csusp(&mut self) -> CsuspW<Cr1Spec> {
        CsuspW::new(self, 10)
    }
    #[doc = "Bit 11 - Rx/Tx direction at Half-duplex mode"]
    #[inline(always)]
    pub fn hddir(&mut self) -> HddirW<Cr1Spec> {
        HddirW::new(self, 11)
    }
    #[doc = "Bit 12 - Internal SS signal input level"]
    #[inline(always)]
    pub fn ssi(&mut self) -> SsiW<Cr1Spec> {
        SsiW::new(self, 12)
    }
    #[doc = "Bit 13 - 32-bit CRC polynomial configuration"]
    #[inline(always)]
    pub fn crc33_17(&mut self) -> Crc33_17W<Cr1Spec> {
        Crc33_17W::new(self, 13)
    }
    #[doc = "Bit 14 - CRC calculation initialization pattern control for receiver"]
    #[inline(always)]
    pub fn rcrci(&mut self) -> RcrciW<Cr1Spec> {
        RcrciW::new(self, 14)
    }
    #[doc = "Bit 15 - CRC calculation initialization pattern control for transmitter"]
    #[inline(always)]
    pub fn tcrci(&mut self) -> TcrciW<Cr1Spec> {
        TcrciW::new(self, 15)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}

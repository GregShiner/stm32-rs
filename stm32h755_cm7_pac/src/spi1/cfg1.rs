#[doc = "Register `CFG1` reader"]
pub type R = crate::R<Cfg1Spec>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<Cfg1Spec>;
#[doc = "Field `DSIZE` reader - Number of bits in at single SPI data frame"]
pub type DsizeR = crate::FieldReader;
#[doc = "Field `DSIZE` writer - Number of bits in at single SPI data frame"]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `FTHVL` reader - threshold level"]
pub type FthvlR = crate::FieldReader;
#[doc = "Field `FTHVL` writer - threshold level"]
pub type FthvlW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UDRCFG` reader - Behavior of slave transmitter at underrun condition"]
pub type UdrcfgR = crate::FieldReader;
#[doc = "Field `UDRCFG` writer - Behavior of slave transmitter at underrun condition"]
pub type UdrcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UDRDET` reader - Detection of underrun condition at slave transmitter"]
pub type UdrdetR = crate::FieldReader;
#[doc = "Field `UDRDET` writer - Detection of underrun condition at slave transmitter"]
pub type UdrdetW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXDMAEN` reader - Rx DMA stream enable"]
pub type RxdmaenR = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Rx DMA stream enable"]
pub type RxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - Tx DMA stream enable"]
pub type TxdmaenR = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - Tx DMA stream enable"]
pub type TxdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCSIZE` reader - Length of CRC frame to be transacted and compared"]
pub type CrcsizeR = crate::FieldReader;
#[doc = "Field `CRCSIZE` writer - Length of CRC frame to be transacted and compared"]
pub type CrcsizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CRCEN` reader - Hardware CRC computation enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - Hardware CRC computation enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MBR` reader - Master baud rate"]
pub type MbrR = crate::FieldReader;
#[doc = "Field `MBR` writer - Master baud rate"]
pub type MbrW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    pub fn fthvl(&self) -> FthvlR {
        FthvlR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    pub fn udrcfg(&self) -> UdrcfgR {
        UdrcfgR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    pub fn udrdet(&self) -> UdrdetR {
        UdrdetR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RxdmaenR {
        RxdmaenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TxdmaenR {
        TxdmaenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub fn crcsize(&self) -> CrcsizeR {
        CrcsizeR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    pub fn mbr(&self) -> MbrR {
        MbrR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of bits in at single SPI data frame"]
    #[inline(always)]
    pub fn dsize(&mut self) -> DsizeW<Cfg1Spec> {
        DsizeW::new(self, 0)
    }
    #[doc = "Bits 5:8 - threshold level"]
    #[inline(always)]
    pub fn fthvl(&mut self) -> FthvlW<Cfg1Spec> {
        FthvlW::new(self, 5)
    }
    #[doc = "Bits 9:10 - Behavior of slave transmitter at underrun condition"]
    #[inline(always)]
    pub fn udrcfg(&mut self) -> UdrcfgW<Cfg1Spec> {
        UdrcfgW::new(self, 9)
    }
    #[doc = "Bits 11:12 - Detection of underrun condition at slave transmitter"]
    #[inline(always)]
    pub fn udrdet(&mut self) -> UdrdetW<Cfg1Spec> {
        UdrdetW::new(self, 11)
    }
    #[doc = "Bit 14 - Rx DMA stream enable"]
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RxdmaenW<Cfg1Spec> {
        RxdmaenW::new(self, 14)
    }
    #[doc = "Bit 15 - Tx DMA stream enable"]
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TxdmaenW<Cfg1Spec> {
        TxdmaenW::new(self, 15)
    }
    #[doc = "Bits 16:20 - Length of CRC frame to be transacted and compared"]
    #[inline(always)]
    pub fn crcsize(&mut self) -> CrcsizeW<Cfg1Spec> {
        CrcsizeW::new(self, 16)
    }
    #[doc = "Bit 22 - Hardware CRC computation enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<Cfg1Spec> {
        CrcenW::new(self, 22)
    }
    #[doc = "Bits 28:30 - Master baud rate"]
    #[inline(always)]
    pub fn mbr(&mut self) -> MbrW<Cfg1Spec> {
        MbrW::new(self, 28)
    }
}
#[doc = "configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfg1Spec;
impl crate::RegisterSpec for Cfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for Cfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for Cfg1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0x0007_0007"]
impl crate::Resettable for Cfg1Spec {
    const RESET_VALUE: u32 = 0x0007_0007;
}

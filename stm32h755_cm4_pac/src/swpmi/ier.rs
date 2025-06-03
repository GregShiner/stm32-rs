#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RXBFIE` reader - Receive buffer full interrupt enable"]
pub type RxbfieR = crate::BitReader;
#[doc = "Field `RXBFIE` writer - Receive buffer full interrupt enable"]
pub type RxbfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBEIE` reader - Transmit buffer empty interrupt enable"]
pub type TxbeieR = crate::BitReader;
#[doc = "Field `TXBEIE` writer - Transmit buffer empty interrupt enable"]
pub type TxbeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBERIE` reader - Receive CRC error interrupt enable"]
pub type RxberieR = crate::BitReader;
#[doc = "Field `RXBERIE` writer - Receive CRC error interrupt enable"]
pub type RxberieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRIE` reader - Receive overrun error interrupt enable"]
pub type RxovrieR = crate::BitReader;
#[doc = "Field `RXOVRIE` writer - Receive overrun error interrupt enable"]
pub type RxovrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNRIE` reader - Transmit underrun error interrupt enable"]
pub type TxunrieR = crate::BitReader;
#[doc = "Field `TXUNRIE` writer - Transmit underrun error interrupt enable"]
pub type TxunrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive interrupt enable"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive interrupt enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE` reader - Transmit interrupt enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transmit complete interrupt enable"]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transmit complete interrupt enable"]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRIE` reader - Slave resume interrupt enable"]
pub type SrieR = crate::BitReader;
#[doc = "Field `SRIE` writer - Slave resume interrupt enable"]
pub type SrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDYIE` reader - Transceiver ready interrupt enable"]
pub type RdyieR = crate::BitReader;
#[doc = "Field `RDYIE` writer - Transceiver ready interrupt enable"]
pub type RdyieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receive buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbfie(&self) -> RxbfieR {
        RxbfieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbeie(&self) -> TxbeieR {
        TxbeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive CRC error interrupt enable"]
    #[inline(always)]
    pub fn rxberie(&self) -> RxberieR {
        RxberieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxovrie(&self) -> RxovrieR {
        RxovrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunrie(&self) -> TxunrieR {
        TxunrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Slave resume interrupt enable"]
    #[inline(always)]
    pub fn srie(&self) -> SrieR {
        SrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Transceiver ready interrupt enable"]
    #[inline(always)]
    pub fn rdyie(&self) -> RdyieR {
        RdyieR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receive buffer full interrupt enable"]
    #[inline(always)]
    pub fn rxbfie(&mut self) -> RxbfieW<IerSpec> {
        RxbfieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txbeie(&mut self) -> TxbeieW<IerSpec> {
        TxbeieW::new(self, 1)
    }
    #[doc = "Bit 2 - Receive CRC error interrupt enable"]
    #[inline(always)]
    pub fn rxberie(&mut self) -> RxberieW<IerSpec> {
        RxberieW::new(self, 2)
    }
    #[doc = "Bit 3 - Receive overrun error interrupt enable"]
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RxovrieW<IerSpec> {
        RxovrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit underrun error interrupt enable"]
    #[inline(always)]
    pub fn txunrie(&mut self) -> TxunrieW<IerSpec> {
        TxunrieW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive interrupt enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<IerSpec> {
        RieW::new(self, 5)
    }
    #[doc = "Bit 6 - Transmit interrupt enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<IerSpec> {
        TieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<IerSpec> {
        TcieW::new(self, 7)
    }
    #[doc = "Bit 8 - Slave resume interrupt enable"]
    #[inline(always)]
    pub fn srie(&mut self) -> SrieW<IerSpec> {
        SrieW::new(self, 8)
    }
    #[doc = "Bit 11 - Transceiver ready interrupt enable"]
    #[inline(always)]
    pub fn rdyie(&mut self) -> RdyieW<IerSpec> {
        RdyieW::new(self, 11)
    }
}
#[doc = "SWPMI Interrupt Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}

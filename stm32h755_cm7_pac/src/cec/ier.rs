#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RXBRIE` reader - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
pub type RxbrieR = crate::BitReader;
#[doc = "Field `RXBRIE` writer - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
pub type RxbrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXENDIE` reader - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
pub type RxendieR = crate::BitReader;
#[doc = "Field `RXENDIE` writer - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
pub type RxendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVRIE` reader - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
pub type RxovrieR = crate::BitReader;
#[doc = "Field `RXOVRIE` writer - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
pub type RxovrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BREIE` reader - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
pub type BreieR = crate::BitReader;
#[doc = "Field `BREIE` writer - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
pub type BreieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBPEIE` reader - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
pub type SbpeieR = crate::BitReader;
#[doc = "Field `SBPEIE` writer - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
pub type SbpeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LBPEIE` reader - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
pub type LbpeieR = crate::BitReader;
#[doc = "Field `LBPEIE` writer - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
pub type LbpeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXACKIE` reader - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
pub type RxackieR = crate::BitReader;
#[doc = "Field `RXACKIE` writer - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
pub type RxackieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARBLSTIE` reader - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
pub type ArblstieR = crate::BitReader;
#[doc = "Field `ARBLSTIE` writer - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
pub type ArblstieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXBRIE` reader - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
pub type TxbrieR = crate::BitReader;
#[doc = "Field `TXBRIE` writer - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
pub type TxbrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENDIE` reader - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
pub type TxendieR = crate::BitReader;
#[doc = "Field `TXENDIE` writer - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
pub type TxendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUDRIE` reader - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
pub type TxudrieR = crate::BitReader;
#[doc = "Field `TXUDRIE` writer - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
pub type TxudrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERRIE` reader - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
pub type TxerrieR = crate::BitReader;
#[doc = "Field `TXERRIE` writer - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
pub type TxerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXACKIE` reader - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
pub type TxackieR = crate::BitReader;
#[doc = "Field `TXACKIE` writer - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
pub type TxackieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxbrie(&self) -> RxbrieR {
        RxbrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxendie(&self) -> RxendieR {
        RxendieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxovrie(&self) -> RxovrieR {
        RxovrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn breie(&self) -> BreieR {
        BreieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn sbpeie(&self) -> SbpeieR {
        SbpeieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn lbpeie(&self) -> LbpeieR {
        LbpeieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxackie(&self) -> RxackieR {
        RxackieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn arblstie(&self) -> ArblstieR {
        ArblstieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txbrie(&self) -> TxbrieR {
        TxbrieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txendie(&self) -> TxendieR {
        TxendieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txudrie(&self) -> TxudrieR {
        TxudrieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txerrie(&self) -> TxerrieR {
        TxerrieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txackie(&self) -> TxackieR {
        TxackieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx-Byte Received Interrupt Enable The RXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxbrie(&mut self) -> RxbrieW<IerSpec> {
        RxbrieW::new(self, 0)
    }
    #[doc = "Bit 1 - End Of Reception Interrupt Enable The RXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxendie(&mut self) -> RxendieW<IerSpec> {
        RxendieW::new(self, 1)
    }
    #[doc = "Bit 2 - Rx-Buffer Overrun Interrupt Enable The RXOVRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxovrie(&mut self) -> RxovrieW<IerSpec> {
        RxovrieW::new(self, 2)
    }
    #[doc = "Bit 3 - Bit Rising Error Interrupt Enable The BREIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn breie(&mut self) -> BreieW<IerSpec> {
        BreieW::new(self, 3)
    }
    #[doc = "Bit 4 - Short Bit Period Error Interrupt Enable The SBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn sbpeie(&mut self) -> SbpeieW<IerSpec> {
        SbpeieW::new(self, 4)
    }
    #[doc = "Bit 5 - Long Bit Period Error Interrupt Enable The LBPEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn lbpeie(&mut self) -> LbpeieW<IerSpec> {
        LbpeieW::new(self, 5)
    }
    #[doc = "Bit 6 - Rx-Missing Acknowledge Error Interrupt Enable The RXACKIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn rxackie(&mut self) -> RxackieW<IerSpec> {
        RxackieW::new(self, 6)
    }
    #[doc = "Bit 7 - Arbitration Lost Interrupt Enable The ARBLSTIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn arblstie(&mut self) -> ArblstieW<IerSpec> {
        ArblstieW::new(self, 7)
    }
    #[doc = "Bit 8 - Tx-Byte Request Interrupt Enable The TXBRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txbrie(&mut self) -> TxbrieW<IerSpec> {
        TxbrieW::new(self, 8)
    }
    #[doc = "Bit 9 - Tx-End Of Message Interrupt Enable The TXENDIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txendie(&mut self) -> TxendieW<IerSpec> {
        TxendieW::new(self, 9)
    }
    #[doc = "Bit 10 - Tx-Underrun Interrupt Enable The TXUDRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txudrie(&mut self) -> TxudrieW<IerSpec> {
        TxudrieW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx-Error Interrupt Enable The TXERRIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txerrie(&mut self) -> TxerrieW<IerSpec> {
        TxerrieW::new(self, 11)
    }
    #[doc = "Bit 12 - Tx-Missing Acknowledge Error Interrupt Enable The TXACKEIE bit is set and cleared by software."]
    #[inline(always)]
    pub fn txackie(&mut self) -> TxackieW<IerSpec> {
        TxackieW::new(self, 12)
    }
}
#[doc = "CEC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

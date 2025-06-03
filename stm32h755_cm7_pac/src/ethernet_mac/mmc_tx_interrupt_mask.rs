#[doc = "Register `MMC_TX_INTERRUPT_MASK` reader"]
pub type R = crate::R<MmcTxInterruptMaskSpec>;
#[doc = "Register `MMC_TX_INTERRUPT_MASK` writer"]
pub type W = crate::W<MmcTxInterruptMaskSpec>;
#[doc = "Field `TXSCOLGPIM` reader - TXSCOLGPIM"]
pub type TxscolgpimR = crate::BitReader;
#[doc = "Field `TXSCOLGPIM` writer - TXSCOLGPIM"]
pub type TxscolgpimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCOLGPIM` reader - TXMCOLGPIM"]
pub type TxmcolgpimR = crate::BitReader;
#[doc = "Field `TXMCOLGPIM` writer - TXMCOLGPIM"]
pub type TxmcolgpimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXGPKTIM` reader - TXGPKTIM"]
pub type TxgpktimR = crate::BitReader;
#[doc = "Field `TXGPKTIM` writer - TXGPKTIM"]
pub type TxgpktimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLPIUSCIM` reader - TXLPIUSCIM"]
pub type TxlpiuscimR = crate::BitReader;
#[doc = "Field `TXLPIUSCIM` writer - TXLPIUSCIM"]
pub type TxlpiuscimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXLPITRCIM` reader - TXLPITRCIM"]
pub type TxlpitrcimR = crate::BitReader;
impl R {
    #[doc = "Bit 14 - TXSCOLGPIM"]
    #[inline(always)]
    pub fn txscolgpim(&self) -> TxscolgpimR {
        TxscolgpimR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TXMCOLGPIM"]
    #[inline(always)]
    pub fn txmcolgpim(&self) -> TxmcolgpimR {
        TxmcolgpimR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - TXGPKTIM"]
    #[inline(always)]
    pub fn txgpktim(&self) -> TxgpktimR {
        TxgpktimR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 26 - TXLPIUSCIM"]
    #[inline(always)]
    pub fn txlpiuscim(&self) -> TxlpiuscimR {
        TxlpiuscimR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TXLPITRCIM"]
    #[inline(always)]
    pub fn txlpitrcim(&self) -> TxlpitrcimR {
        TxlpitrcimR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - TXSCOLGPIM"]
    #[inline(always)]
    pub fn txscolgpim(&mut self) -> TxscolgpimW<MmcTxInterruptMaskSpec> {
        TxscolgpimW::new(self, 14)
    }
    #[doc = "Bit 15 - TXMCOLGPIM"]
    #[inline(always)]
    pub fn txmcolgpim(&mut self) -> TxmcolgpimW<MmcTxInterruptMaskSpec> {
        TxmcolgpimW::new(self, 15)
    }
    #[doc = "Bit 21 - TXGPKTIM"]
    #[inline(always)]
    pub fn txgpktim(&mut self) -> TxgpktimW<MmcTxInterruptMaskSpec> {
        TxgpktimW::new(self, 21)
    }
    #[doc = "Bit 26 - TXLPIUSCIM"]
    #[inline(always)]
    pub fn txlpiuscim(&mut self) -> TxlpiuscimW<MmcTxInterruptMaskSpec> {
        TxlpiuscimW::new(self, 26)
    }
}
#[doc = "MMC Tx interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_tx_interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_tx_interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcTxInterruptMaskSpec;
impl crate::RegisterSpec for MmcTxInterruptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_tx_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MmcTxInterruptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_tx_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MmcTxInterruptMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMC_TX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MmcTxInterruptMaskSpec {}

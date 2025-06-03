#[doc = "Register `MMC_RX_INTERRUPT_MASK` reader"]
pub type R = crate::R<MmcRxInterruptMaskSpec>;
#[doc = "Register `MMC_RX_INTERRUPT_MASK` writer"]
pub type W = crate::W<MmcRxInterruptMaskSpec>;
#[doc = "Field `RXCRCERPIM` reader - RXCRCERPIM"]
pub type RxcrcerpimR = crate::BitReader;
#[doc = "Field `RXCRCERPIM` writer - RXCRCERPIM"]
pub type RxcrcerpimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXALGNERPIM` reader - RXALGNERPIM"]
pub type RxalgnerpimR = crate::BitReader;
#[doc = "Field `RXALGNERPIM` writer - RXALGNERPIM"]
pub type RxalgnerpimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXUCGPIM` reader - RXUCGPIM"]
pub type RxucgpimR = crate::BitReader;
#[doc = "Field `RXUCGPIM` writer - RXUCGPIM"]
pub type RxucgpimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPIUSCIM` reader - RXLPIUSCIM"]
pub type RxlpiuscimR = crate::BitReader;
#[doc = "Field `RXLPIUSCIM` writer - RXLPIUSCIM"]
pub type RxlpiuscimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXLPITRCIM` reader - RXLPITRCIM"]
pub type RxlpitrcimR = crate::BitReader;
impl R {
    #[doc = "Bit 5 - RXCRCERPIM"]
    #[inline(always)]
    pub fn rxcrcerpim(&self) -> RxcrcerpimR {
        RxcrcerpimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RXALGNERPIM"]
    #[inline(always)]
    pub fn rxalgnerpim(&self) -> RxalgnerpimR {
        RxalgnerpimR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 17 - RXUCGPIM"]
    #[inline(always)]
    pub fn rxucgpim(&self) -> RxucgpimR {
        RxucgpimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 26 - RXLPIUSCIM"]
    #[inline(always)]
    pub fn rxlpiuscim(&self) -> RxlpiuscimR {
        RxlpiuscimR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - RXLPITRCIM"]
    #[inline(always)]
    pub fn rxlpitrcim(&self) -> RxlpitrcimR {
        RxlpitrcimR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - RXCRCERPIM"]
    #[inline(always)]
    pub fn rxcrcerpim(&mut self) -> RxcrcerpimW<MmcRxInterruptMaskSpec> {
        RxcrcerpimW::new(self, 5)
    }
    #[doc = "Bit 6 - RXALGNERPIM"]
    #[inline(always)]
    pub fn rxalgnerpim(&mut self) -> RxalgnerpimW<MmcRxInterruptMaskSpec> {
        RxalgnerpimW::new(self, 6)
    }
    #[doc = "Bit 17 - RXUCGPIM"]
    #[inline(always)]
    pub fn rxucgpim(&mut self) -> RxucgpimW<MmcRxInterruptMaskSpec> {
        RxucgpimW::new(self, 17)
    }
    #[doc = "Bit 26 - RXLPIUSCIM"]
    #[inline(always)]
    pub fn rxlpiuscim(&mut self) -> RxlpiuscimW<MmcRxInterruptMaskSpec> {
        RxlpiuscimW::new(self, 26)
    }
}
#[doc = "MMC Rx interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`mmc_rx_interrupt_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmc_rx_interrupt_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmcRxInterruptMaskSpec;
impl crate::RegisterSpec for MmcRxInterruptMaskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_rx_interrupt_mask::R`](R) reader structure"]
impl crate::Readable for MmcRxInterruptMaskSpec {}
#[doc = "`write(|w| ..)` method takes [`mmc_rx_interrupt_mask::W`](W) writer structure"]
impl crate::Writable for MmcRxInterruptMaskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMC_RX_INTERRUPT_MASK to value 0"]
impl crate::Resettable for MmcRxInterruptMaskSpec {}

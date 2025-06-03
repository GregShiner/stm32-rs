#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `RXDMA` reader - Reception DMA enable"]
pub type RxdmaR = crate::BitReader;
#[doc = "Field `RXDMA` writer - Reception DMA enable"]
pub type RxdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMA` reader - Transmission DMA enable"]
pub type TxdmaR = crate::BitReader;
#[doc = "Field `TXDMA` writer - Transmission DMA enable"]
pub type TxdmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMODE` reader - Reception buffering mode"]
pub type RxmodeR = crate::BitReader;
#[doc = "Field `RXMODE` writer - Reception buffering mode"]
pub type RxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMODE` reader - Transmission buffering mode"]
pub type TxmodeR = crate::BitReader;
#[doc = "Field `TXMODE` writer - Transmission buffering mode"]
pub type TxmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPBK` reader - Loopback mode enable"]
pub type LpbkR = crate::BitReader;
#[doc = "Field `LPBK` writer - Loopback mode enable"]
pub type LpbkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPACT` reader - Single wire protocol master interface activate"]
pub type SwpactR = crate::BitReader;
#[doc = "Field `SWPACT` writer - Single wire protocol master interface activate"]
pub type SwpactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEACT` reader - Single wire protocol master interface deactivate"]
pub type DeactR = crate::BitReader;
#[doc = "Field `DEACT` writer - Single wire protocol master interface deactivate"]
pub type DeactW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPTEN` reader - Single wire protocol master transceiver enable"]
pub type SwptenR = crate::BitReader;
#[doc = "Field `SWPTEN` writer - Single wire protocol master transceiver enable"]
pub type SwptenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    pub fn rxdma(&self) -> RxdmaR {
        RxdmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    pub fn txdma(&self) -> TxdmaR {
        TxdmaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    pub fn rxmode(&self) -> RxmodeR {
        RxmodeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    pub fn txmode(&self) -> TxmodeR {
        TxmodeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    pub fn lpbk(&self) -> LpbkR {
        LpbkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Single wire protocol master interface activate"]
    #[inline(always)]
    pub fn swpact(&self) -> SwpactR {
        SwpactR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    pub fn deact(&self) -> DeactR {
        DeactR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Single wire protocol master transceiver enable"]
    #[inline(always)]
    pub fn swpten(&self) -> SwptenR {
        SwptenR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    pub fn rxdma(&mut self) -> RxdmaW<CrSpec> {
        RxdmaW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    pub fn txdma(&mut self) -> TxdmaW<CrSpec> {
        TxdmaW::new(self, 1)
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    pub fn rxmode(&mut self) -> RxmodeW<CrSpec> {
        RxmodeW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    pub fn txmode(&mut self) -> TxmodeW<CrSpec> {
        TxmodeW::new(self, 3)
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    pub fn lpbk(&mut self) -> LpbkW<CrSpec> {
        LpbkW::new(self, 4)
    }
    #[doc = "Bit 5 - Single wire protocol master interface activate"]
    #[inline(always)]
    pub fn swpact(&mut self) -> SwpactW<CrSpec> {
        SwpactW::new(self, 5)
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    pub fn deact(&mut self) -> DeactW<CrSpec> {
        DeactW::new(self, 10)
    }
    #[doc = "Bit 11 - Single wire protocol master transceiver enable"]
    #[inline(always)]
    pub fn swpten(&mut self) -> SwptenW<CrSpec> {
        SwptenW::new(self, 11)
    }
}
#[doc = "SWPMI Configuration/Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}

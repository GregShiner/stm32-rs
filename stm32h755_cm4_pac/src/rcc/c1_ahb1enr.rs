#[doc = "Register `C1_AHB1ENR` reader"]
pub type R = crate::R<C1Ahb1enrSpec>;
#[doc = "Register `C1_AHB1ENR` writer"]
pub type W = crate::W<C1Ahb1enrSpec>;
#[doc = "Field `DMA1EN` reader - DMA1 Clock Enable"]
pub type Dma1enR = crate::BitReader;
#[doc = "Field `DMA1EN` writer - DMA1 Clock Enable"]
pub type Dma1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2EN` reader - DMA2 Clock Enable"]
pub type Dma2enR = crate::BitReader;
#[doc = "Field `DMA2EN` writer - DMA2 Clock Enable"]
pub type Dma2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12EN` reader - ADC1/2 Peripheral Clocks Enable"]
pub type Adc12enR = crate::BitReader;
#[doc = "Field `ADC12EN` writer - ADC1/2 Peripheral Clocks Enable"]
pub type Adc12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH1MACEN` reader - Ethernet MAC bus interface Clock Enable"]
pub type Eth1macenR = crate::BitReader;
#[doc = "Field `ETH1MACEN` writer - Ethernet MAC bus interface Clock Enable"]
pub type Eth1macenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH1TXEN` reader - Ethernet Transmission Clock Enable"]
pub type Eth1txenR = crate::BitReader;
#[doc = "Field `ETH1TXEN` writer - Ethernet Transmission Clock Enable"]
pub type Eth1txenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH1RXEN` reader - Ethernet Reception Clock Enable"]
pub type Eth1rxenR = crate::BitReader;
#[doc = "Field `ETH1RXEN` writer - Ethernet Reception Clock Enable"]
pub type Eth1rxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1OTGEN` reader - USB1OTG Peripheral Clocks Enable"]
pub type Usb1otgenR = crate::BitReader;
#[doc = "Field `USB1OTGEN` writer - USB1OTG Peripheral Clocks Enable"]
pub type Usb1otgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1ULPIEN` reader - USB_PHY1 Clocks Enable"]
pub type Usb1ulpienR = crate::BitReader;
#[doc = "Field `USB1ULPIEN` writer - USB_PHY1 Clocks Enable"]
pub type Usb1ulpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2OTGEN` reader - USB2OTG Peripheral Clocks Enable"]
pub type Usb2otgenR = crate::BitReader;
#[doc = "Field `USB2OTGEN` writer - USB2OTG Peripheral Clocks Enable"]
pub type Usb2otgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2ULPIEN` reader - USB_PHY2 Clocks Enable"]
pub type Usb2ulpienR = crate::BitReader;
#[doc = "Field `USB2ULPIEN` writer - USB_PHY2 Clocks Enable"]
pub type Usb2ulpienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 Clock Enable"]
    #[inline(always)]
    pub fn dma1en(&self) -> Dma1enR {
        Dma1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable"]
    #[inline(always)]
    pub fn dma2en(&self) -> Dma2enR {
        Dma2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc12en(&self) -> Adc12enR {
        Adc12enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable"]
    #[inline(always)]
    pub fn eth1macen(&self) -> Eth1macenR {
        Eth1macenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable"]
    #[inline(always)]
    pub fn eth1txen(&self) -> Eth1txenR {
        Eth1txenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable"]
    #[inline(always)]
    pub fn eth1rxen(&self) -> Eth1rxenR {
        Eth1rxenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb1otgen(&self) -> Usb1otgenR {
        Usb1otgenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 Clocks Enable"]
    #[inline(always)]
    pub fn usb1ulpien(&self) -> Usb1ulpienR {
        Usb1ulpienR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USB2OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb2otgen(&self) -> Usb2otgenR {
        Usb2otgenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB_PHY2 Clocks Enable"]
    #[inline(always)]
    pub fn usb2ulpien(&self) -> Usb2ulpienR {
        Usb2ulpienR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 Clock Enable"]
    #[inline(always)]
    pub fn dma1en(&mut self) -> Dma1enW<C1Ahb1enrSpec> {
        Dma1enW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable"]
    #[inline(always)]
    pub fn dma2en(&mut self) -> Dma2enW<C1Ahb1enrSpec> {
        Dma2enW::new(self, 1)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc12en(&mut self) -> Adc12enW<C1Ahb1enrSpec> {
        Adc12enW::new(self, 5)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable"]
    #[inline(always)]
    pub fn eth1macen(&mut self) -> Eth1macenW<C1Ahb1enrSpec> {
        Eth1macenW::new(self, 15)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable"]
    #[inline(always)]
    pub fn eth1txen(&mut self) -> Eth1txenW<C1Ahb1enrSpec> {
        Eth1txenW::new(self, 16)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable"]
    #[inline(always)]
    pub fn eth1rxen(&mut self) -> Eth1rxenW<C1Ahb1enrSpec> {
        Eth1rxenW::new(self, 17)
    }
    #[doc = "Bit 25 - USB1OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb1otgen(&mut self) -> Usb1otgenW<C1Ahb1enrSpec> {
        Usb1otgenW::new(self, 25)
    }
    #[doc = "Bit 26 - USB_PHY1 Clocks Enable"]
    #[inline(always)]
    pub fn usb1ulpien(&mut self) -> Usb1ulpienW<C1Ahb1enrSpec> {
        Usb1ulpienW::new(self, 26)
    }
    #[doc = "Bit 27 - USB2OTG Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usb2otgen(&mut self) -> Usb2otgenW<C1Ahb1enrSpec> {
        Usb2otgenW::new(self, 27)
    }
    #[doc = "Bit 28 - USB_PHY2 Clocks Enable"]
    #[inline(always)]
    pub fn usb2ulpien(&mut self) -> Usb2ulpienW<C1Ahb1enrSpec> {
        Usb2ulpienW::new(self, 28)
    }
}
#[doc = "RCC AHB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Ahb1enrSpec;
impl crate::RegisterSpec for C1Ahb1enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb1enr::R`](R) reader structure"]
impl crate::Readable for C1Ahb1enrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb1enr::W`](W) writer structure"]
impl crate::Writable for C1Ahb1enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1_AHB1ENR to value 0"]
impl crate::Resettable for C1Ahb1enrSpec {}

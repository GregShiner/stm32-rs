#[doc = "Register `AHB1LPENR` reader"]
pub type R = crate::R<Ahb1lpenrSpec>;
#[doc = "Register `AHB1LPENR` writer"]
pub type W = crate::W<Ahb1lpenrSpec>;
#[doc = "Field `DMA1LPEN` reader - DMA1 Clock Enable During CSleep Mode"]
pub type Dma1lpenR = crate::BitReader;
#[doc = "Field `DMA1LPEN` writer - DMA1 Clock Enable During CSleep Mode"]
pub type Dma1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2LPEN` reader - DMA2 Clock Enable During CSleep Mode"]
pub type Dma2lpenR = crate::BitReader;
#[doc = "Field `DMA2LPEN` writer - DMA2 Clock Enable During CSleep Mode"]
pub type Dma2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12LPEN` reader - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub type Adc12lpenR = crate::BitReader;
#[doc = "Field `ADC12LPEN` writer - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
pub type Adc12lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH1MACLPEN` reader - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub type Eth1maclpenR = crate::BitReader;
#[doc = "Field `ETH1MACLPEN` writer - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
pub type Eth1maclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH1TXLPEN` reader - Ethernet Transmission Clock Enable During CSleep Mode"]
pub type Eth1txlpenR = crate::BitReader;
#[doc = "Field `ETH1TXLPEN` writer - Ethernet Transmission Clock Enable During CSleep Mode"]
pub type Eth1txlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH1RXLPEN` reader - Ethernet Reception Clock Enable During CSleep Mode"]
pub type Eth1rxlpenR = crate::BitReader;
#[doc = "Field `ETH1RXLPEN` writer - Ethernet Reception Clock Enable During CSleep Mode"]
pub type Eth1rxlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1OTGLPEN` reader - USB1OTG peripheral clock enable during CSleep mode"]
pub type Usb1otglpenR = crate::BitReader;
#[doc = "Field `USB1OTGLPEN` writer - USB1OTG peripheral clock enable during CSleep mode"]
pub type Usb1otglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1ULPILPEN` reader - USB_PHY1 clock enable during CSleep mode"]
pub type Usb1ulpilpenR = crate::BitReader;
#[doc = "Field `USB1ULPILPEN` writer - USB_PHY1 clock enable during CSleep mode"]
pub type Usb1ulpilpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2OTGLPEN` reader - USB2OTG peripheral clock enable during CSleep mode"]
pub type Usb2otglpenR = crate::BitReader;
#[doc = "Field `USB2OTGLPEN` writer - USB2OTG peripheral clock enable during CSleep mode"]
pub type Usb2otglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2ULPILPEN` reader - USB_PHY2 clocks enable during CSleep mode"]
pub type Usb2ulpilpenR = crate::BitReader;
#[doc = "Field `USB2ULPILPEN` writer - USB_PHY2 clocks enable during CSleep mode"]
pub type Usb2ulpilpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma1lpen(&self) -> Dma1lpenR {
        Dma1lpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2lpen(&self) -> Dma2lpenR {
        Dma2lpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc12lpen(&self) -> Adc12lpenR {
        Adc12lpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1maclpen(&self) -> Eth1maclpenR {
        Eth1maclpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1txlpen(&self) -> Eth1txlpenR {
        Eth1txlpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1rxlpen(&self) -> Eth1rxlpenR {
        Eth1rxlpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1otglpen(&self) -> Usb1otglpenR {
        Usb1otglpenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB_PHY1 clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1ulpilpen(&self) -> Usb1ulpilpenR {
        Usb1ulpilpenR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USB2OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2otglpen(&self) -> Usb2otglpenR {
        Usb2otglpenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - USB_PHY2 clocks enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2ulpilpen(&self) -> Usb2ulpilpenR {
        Usb2ulpilpenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma1lpen(&mut self) -> Dma1lpenW<Ahb1lpenrSpec> {
        Dma1lpenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2lpen(&mut self) -> Dma2lpenW<Ahb1lpenrSpec> {
        Dma2lpenW::new(self, 1)
    }
    #[doc = "Bit 5 - ADC1/2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc12lpen(&mut self) -> Adc12lpenW<Ahb1lpenrSpec> {
        Adc12lpenW::new(self, 5)
    }
    #[doc = "Bit 15 - Ethernet MAC bus interface Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1maclpen(&mut self) -> Eth1maclpenW<Ahb1lpenrSpec> {
        Eth1maclpenW::new(self, 15)
    }
    #[doc = "Bit 16 - Ethernet Transmission Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1txlpen(&mut self) -> Eth1txlpenW<Ahb1lpenrSpec> {
        Eth1txlpenW::new(self, 16)
    }
    #[doc = "Bit 17 - Ethernet Reception Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn eth1rxlpen(&mut self) -> Eth1rxlpenW<Ahb1lpenrSpec> {
        Eth1rxlpenW::new(self, 17)
    }
    #[doc = "Bit 25 - USB1OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1otglpen(&mut self) -> Usb1otglpenW<Ahb1lpenrSpec> {
        Usb1otglpenW::new(self, 25)
    }
    #[doc = "Bit 26 - USB_PHY1 clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb1ulpilpen(&mut self) -> Usb1ulpilpenW<Ahb1lpenrSpec> {
        Usb1ulpilpenW::new(self, 26)
    }
    #[doc = "Bit 27 - USB2OTG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2otglpen(&mut self) -> Usb2otglpenW<Ahb1lpenrSpec> {
        Usb2otglpenW::new(self, 27)
    }
    #[doc = "Bit 28 - USB_PHY2 clocks enable during CSleep mode"]
    #[inline(always)]
    pub fn usb2ulpilpen(&mut self) -> Usb2ulpilpenW<Ahb1lpenrSpec> {
        Usb2ulpilpenW::new(self, 28)
    }
}
#[doc = "RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1lpenrSpec;
impl crate::RegisterSpec for Ahb1lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1lpenr::R`](R) reader structure"]
impl crate::Readable for Ahb1lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1lpenr::W`](W) writer structure"]
impl crate::Writable for Ahb1lpenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1LPENR to value 0"]
impl crate::Resettable for Ahb1lpenrSpec {}

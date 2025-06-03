#[doc = "Register `AHB1RSTR` reader"]
pub type R = crate::R<Ahb1rstrSpec>;
#[doc = "Register `AHB1RSTR` writer"]
pub type W = crate::W<Ahb1rstrSpec>;
#[doc = "Field `DMA1RST` reader - DMA1 block reset"]
pub type Dma1rstR = crate::BitReader;
#[doc = "Field `DMA1RST` writer - DMA1 block reset"]
pub type Dma1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2RST` reader - DMA2 block reset"]
pub type Dma2rstR = crate::BitReader;
#[doc = "Field `DMA2RST` writer - DMA2 block reset"]
pub type Dma2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC12RST` reader - ADC1&2 block reset"]
pub type Adc12rstR = crate::BitReader;
#[doc = "Field `ADC12RST` writer - ADC1&2 block reset"]
pub type Adc12rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETH1MACRST` reader - ETH1MAC block reset"]
pub type Eth1macrstR = crate::BitReader;
#[doc = "Field `ETH1MACRST` writer - ETH1MAC block reset"]
pub type Eth1macrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB1OTGRST` reader - USB1OTG block reset"]
pub type Usb1otgrstR = crate::BitReader;
#[doc = "Field `USB1OTGRST` writer - USB1OTG block reset"]
pub type Usb1otgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB2OTGRST` reader - USB2OTG block reset"]
pub type Usb2otgrstR = crate::BitReader;
#[doc = "Field `USB2OTGRST` writer - USB2OTG block reset"]
pub type Usb2otgrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    pub fn dma1rst(&self) -> Dma1rstR {
        Dma1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    pub fn dma2rst(&self) -> Dma2rstR {
        Dma2rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC1&2 block reset"]
    #[inline(always)]
    pub fn adc12rst(&self) -> Adc12rstR {
        Adc12rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    pub fn eth1macrst(&self) -> Eth1macrstR {
        Eth1macrstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    pub fn usb1otgrst(&self) -> Usb1otgrstR {
        Usb1otgrstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    pub fn usb2otgrst(&self) -> Usb2otgrstR {
        Usb2otgrstR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA1 block reset"]
    #[inline(always)]
    pub fn dma1rst(&mut self) -> Dma1rstW<Ahb1rstrSpec> {
        Dma1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA2 block reset"]
    #[inline(always)]
    pub fn dma2rst(&mut self) -> Dma2rstW<Ahb1rstrSpec> {
        Dma2rstW::new(self, 1)
    }
    #[doc = "Bit 5 - ADC1&2 block reset"]
    #[inline(always)]
    pub fn adc12rst(&mut self) -> Adc12rstW<Ahb1rstrSpec> {
        Adc12rstW::new(self, 5)
    }
    #[doc = "Bit 15 - ETH1MAC block reset"]
    #[inline(always)]
    pub fn eth1macrst(&mut self) -> Eth1macrstW<Ahb1rstrSpec> {
        Eth1macrstW::new(self, 15)
    }
    #[doc = "Bit 25 - USB1OTG block reset"]
    #[inline(always)]
    pub fn usb1otgrst(&mut self) -> Usb1otgrstW<Ahb1rstrSpec> {
        Usb1otgrstW::new(self, 25)
    }
    #[doc = "Bit 27 - USB2OTG block reset"]
    #[inline(always)]
    pub fn usb2otgrst(&mut self) -> Usb2otgrstW<Ahb1rstrSpec> {
        Usb2otgrstW::new(self, 27)
    }
}
#[doc = "RCC AHB1 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb1rstrSpec;
impl crate::RegisterSpec for Ahb1rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb1rstr::R`](R) reader structure"]
impl crate::Readable for Ahb1rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb1rstr::W`](W) writer structure"]
impl crate::Writable for Ahb1rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB1RSTR to value 0"]
impl crate::Resettable for Ahb1rstrSpec {}

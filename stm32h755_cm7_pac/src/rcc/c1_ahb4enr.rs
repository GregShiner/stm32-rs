#[doc = "Register `C1_AHB4ENR` reader"]
pub type R = crate::R<C1Ahb4enrSpec>;
#[doc = "Register `C1_AHB4ENR` writer"]
pub type W = crate::W<C1Ahb4enrSpec>;
#[doc = "Field `GPIOAEN` reader - 0GPIO peripheral clock enable"]
pub type GpioaenR = crate::BitReader;
#[doc = "Field `GPIOAEN` writer - 0GPIO peripheral clock enable"]
pub type GpioaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBEN` reader - 0GPIO peripheral clock enable"]
pub type GpiobenR = crate::BitReader;
#[doc = "Field `GPIOBEN` writer - 0GPIO peripheral clock enable"]
pub type GpiobenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCEN` reader - 0GPIO peripheral clock enable"]
pub type GpiocenR = crate::BitReader;
#[doc = "Field `GPIOCEN` writer - 0GPIO peripheral clock enable"]
pub type GpiocenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODEN` reader - 0GPIO peripheral clock enable"]
pub type GpiodenR = crate::BitReader;
#[doc = "Field `GPIODEN` writer - 0GPIO peripheral clock enable"]
pub type GpiodenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOEEN` reader - 0GPIO peripheral clock enable"]
pub type GpioeenR = crate::BitReader;
#[doc = "Field `GPIOEEN` writer - 0GPIO peripheral clock enable"]
pub type GpioeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFEN` reader - 0GPIO peripheral clock enable"]
pub type GpiofenR = crate::BitReader;
#[doc = "Field `GPIOFEN` writer - 0GPIO peripheral clock enable"]
pub type GpiofenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGEN` reader - 0GPIO peripheral clock enable"]
pub type GpiogenR = crate::BitReader;
#[doc = "Field `GPIOGEN` writer - 0GPIO peripheral clock enable"]
pub type GpiogenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHEN` reader - 0GPIO peripheral clock enable"]
pub type GpiohenR = crate::BitReader;
#[doc = "Field `GPIOHEN` writer - 0GPIO peripheral clock enable"]
pub type GpiohenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOIEN` reader - 0GPIO peripheral clock enable"]
pub type GpioienR = crate::BitReader;
#[doc = "Field `GPIOIEN` writer - 0GPIO peripheral clock enable"]
pub type GpioienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOJEN` reader - 0GPIO peripheral clock enable"]
pub type GpiojenR = crate::BitReader;
#[doc = "Field `GPIOJEN` writer - 0GPIO peripheral clock enable"]
pub type GpiojenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOKEN` reader - 0GPIO peripheral clock enable"]
pub type GpiokenR = crate::BitReader;
#[doc = "Field `GPIOKEN` writer - 0GPIO peripheral clock enable"]
pub type GpiokenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC peripheral clock enable"]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC peripheral clock enable"]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDMAEN` reader - BDMA and DMAMUX2 Clock Enable"]
pub type BdmaenR = crate::BitReader;
#[doc = "Field `BDMAEN` writer - BDMA and DMAMUX2 Clock Enable"]
pub type BdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3EN` reader - ADC3 Peripheral Clocks Enable"]
pub type Adc3enR = crate::BitReader;
#[doc = "Field `ADC3EN` writer - ADC3 Peripheral Clocks Enable"]
pub type Adc3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEMEN` reader - HSEM peripheral clock enable"]
pub type HsemenR = crate::BitReader;
#[doc = "Field `HSEMEN` writer - HSEM peripheral clock enable"]
pub type HsemenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPRAMEN` reader - Backup RAM Clock Enable"]
pub type BkpramenR = crate::BitReader;
#[doc = "Field `BKPRAMEN` writer - Backup RAM Clock Enable"]
pub type BkpramenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioaen(&self) -> GpioaenR {
        GpioaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioben(&self) -> GpiobenR {
        GpiobenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiocen(&self) -> GpiocenR {
        GpiocenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioden(&self) -> GpiodenR {
        GpiodenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioeen(&self) -> GpioeenR {
        GpioeenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiofen(&self) -> GpiofenR {
        GpiofenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiogen(&self) -> GpiogenR {
        GpiogenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiohen(&self) -> GpiohenR {
        GpiohenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioien(&self) -> GpioienR {
        GpioienR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiojen(&self) -> GpiojenR {
        GpiojenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioken(&self) -> GpiokenR {
        GpiokenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA and DMAMUX2 Clock Enable"]
    #[inline(always)]
    pub fn bdmaen(&self) -> BdmaenR {
        BdmaenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc3en(&self) -> Adc3enR {
        Adc3enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HSEM peripheral clock enable"]
    #[inline(always)]
    pub fn hsemen(&self) -> HsemenR {
        HsemenR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable"]
    #[inline(always)]
    pub fn bkpramen(&self) -> BkpramenR {
        BkpramenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioaen(&mut self) -> GpioaenW<C1Ahb4enrSpec> {
        GpioaenW::new(self, 0)
    }
    #[doc = "Bit 1 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioben(&mut self) -> GpiobenW<C1Ahb4enrSpec> {
        GpiobenW::new(self, 1)
    }
    #[doc = "Bit 2 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiocen(&mut self) -> GpiocenW<C1Ahb4enrSpec> {
        GpiocenW::new(self, 2)
    }
    #[doc = "Bit 3 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioden(&mut self) -> GpiodenW<C1Ahb4enrSpec> {
        GpiodenW::new(self, 3)
    }
    #[doc = "Bit 4 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioeen(&mut self) -> GpioeenW<C1Ahb4enrSpec> {
        GpioeenW::new(self, 4)
    }
    #[doc = "Bit 5 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiofen(&mut self) -> GpiofenW<C1Ahb4enrSpec> {
        GpiofenW::new(self, 5)
    }
    #[doc = "Bit 6 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiogen(&mut self) -> GpiogenW<C1Ahb4enrSpec> {
        GpiogenW::new(self, 6)
    }
    #[doc = "Bit 7 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiohen(&mut self) -> GpiohenW<C1Ahb4enrSpec> {
        GpiohenW::new(self, 7)
    }
    #[doc = "Bit 8 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioien(&mut self) -> GpioienW<C1Ahb4enrSpec> {
        GpioienW::new(self, 8)
    }
    #[doc = "Bit 9 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpiojen(&mut self) -> GpiojenW<C1Ahb4enrSpec> {
        GpiojenW::new(self, 9)
    }
    #[doc = "Bit 10 - 0GPIO peripheral clock enable"]
    #[inline(always)]
    pub fn gpioken(&mut self) -> GpiokenW<C1Ahb4enrSpec> {
        GpiokenW::new(self, 10)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable"]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<C1Ahb4enrSpec> {
        CrcenW::new(self, 19)
    }
    #[doc = "Bit 21 - BDMA and DMAMUX2 Clock Enable"]
    #[inline(always)]
    pub fn bdmaen(&mut self) -> BdmaenW<C1Ahb4enrSpec> {
        BdmaenW::new(self, 21)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn adc3en(&mut self) -> Adc3enW<C1Ahb4enrSpec> {
        Adc3enW::new(self, 24)
    }
    #[doc = "Bit 25 - HSEM peripheral clock enable"]
    #[inline(always)]
    pub fn hsemen(&mut self) -> HsemenW<C1Ahb4enrSpec> {
        HsemenW::new(self, 25)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable"]
    #[inline(always)]
    pub fn bkpramen(&mut self) -> BkpramenW<C1Ahb4enrSpec> {
        BkpramenW::new(self, 28)
    }
}
#[doc = "RCC AHB4 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Ahb4enrSpec;
impl crate::RegisterSpec for C1Ahb4enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb4enr::R`](R) reader structure"]
impl crate::Readable for C1Ahb4enrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb4enr::W`](W) writer structure"]
impl crate::Writable for C1Ahb4enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1_AHB4ENR to value 0"]
impl crate::Resettable for C1Ahb4enrSpec {}

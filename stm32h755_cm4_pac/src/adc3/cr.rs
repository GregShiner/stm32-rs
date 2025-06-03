#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `ADEN` reader - ADC enable"]
pub type AdenR = crate::BitReader;
#[doc = "Field `ADEN` writer - ADC enable"]
pub type AdenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADDIS` reader - ADC disable"]
pub type AddisR = crate::BitReader;
#[doc = "Field `ADDIS` writer - ADC disable"]
pub type AddisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTART` reader - ADC group regular conversion start"]
pub type AdstartR = crate::BitReader;
#[doc = "Field `ADSTART` writer - ADC group regular conversion start"]
pub type AdstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTART` reader - ADC group injected conversion start"]
pub type JadstartR = crate::BitReader;
#[doc = "Field `JADSTART` writer - ADC group injected conversion start"]
pub type JadstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSTP` reader - ADC group regular conversion stop"]
pub type AdstpR = crate::BitReader;
#[doc = "Field `ADSTP` writer - ADC group regular conversion stop"]
pub type AdstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JADSTP` reader - ADC group injected conversion stop"]
pub type JadstpR = crate::BitReader;
#[doc = "Field `JADSTP` writer - ADC group injected conversion stop"]
pub type JadstpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOST` reader - Boost mode control"]
pub type BoostR = crate::FieldReader;
#[doc = "Field `BOOST` writer - Boost mode control"]
pub type BoostW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADCALLIN` reader - Linearity calibration"]
pub type AdcallinR = crate::BitReader;
#[doc = "Field `ADCALLIN` writer - Linearity calibration"]
pub type AdcallinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW1` reader - Linearity calibration ready Word 1"]
pub type Lincalrdyw1R = crate::BitReader;
#[doc = "Field `LINCALRDYW1` writer - Linearity calibration ready Word 1"]
pub type Lincalrdyw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW2` reader - Linearity calibration ready Word 2"]
pub type Lincalrdyw2R = crate::BitReader;
#[doc = "Field `LINCALRDYW2` writer - Linearity calibration ready Word 2"]
pub type Lincalrdyw2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW3` reader - Linearity calibration ready Word 3"]
pub type Lincalrdyw3R = crate::BitReader;
#[doc = "Field `LINCALRDYW3` writer - Linearity calibration ready Word 3"]
pub type Lincalrdyw3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW4` reader - Linearity calibration ready Word 4"]
pub type Lincalrdyw4R = crate::BitReader;
#[doc = "Field `LINCALRDYW4` writer - Linearity calibration ready Word 4"]
pub type Lincalrdyw4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW5` reader - Linearity calibration ready Word 5"]
pub type Lincalrdyw5R = crate::BitReader;
#[doc = "Field `LINCALRDYW5` writer - Linearity calibration ready Word 5"]
pub type Lincalrdyw5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINCALRDYW6` reader - Linearity calibration ready Word 6"]
pub type Lincalrdyw6R = crate::BitReader;
#[doc = "Field `LINCALRDYW6` writer - Linearity calibration ready Word 6"]
pub type Lincalrdyw6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADVREGEN` reader - ADC voltage regulator enable"]
pub type AdvregenR = crate::BitReader;
#[doc = "Field `ADVREGEN` writer - ADC voltage regulator enable"]
pub type AdvregenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEEPPWD` reader - ADC deep power down enable"]
pub type DeeppwdR = crate::BitReader;
#[doc = "Field `DEEPPWD` writer - ADC deep power down enable"]
pub type DeeppwdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCALDIF` reader - ADC differential mode for calibration"]
pub type AdcaldifR = crate::BitReader;
#[doc = "Field `ADCALDIF` writer - ADC differential mode for calibration"]
pub type AdcaldifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADCAL` reader - ADC calibration"]
pub type AdcalR = crate::BitReader;
#[doc = "Field `ADCAL` writer - ADC calibration"]
pub type AdcalW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&self) -> AdenR {
        AdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    pub fn addis(&self) -> AddisR {
        AddisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&self) -> AdstartR {
        AdstartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&self) -> JadstartR {
        JadstartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&self) -> AdstpR {
        AdstpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&self) -> JadstpR {
        JadstpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Boost mode control"]
    #[inline(always)]
    pub fn boost(&self) -> BoostR {
        BoostR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - Linearity calibration"]
    #[inline(always)]
    pub fn adcallin(&self) -> AdcallinR {
        AdcallinR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 22 - Linearity calibration ready Word 1"]
    #[inline(always)]
    pub fn lincalrdyw1(&self) -> Lincalrdyw1R {
        Lincalrdyw1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Linearity calibration ready Word 2"]
    #[inline(always)]
    pub fn lincalrdyw2(&self) -> Lincalrdyw2R {
        Lincalrdyw2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Linearity calibration ready Word 3"]
    #[inline(always)]
    pub fn lincalrdyw3(&self) -> Lincalrdyw3R {
        Lincalrdyw3R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Linearity calibration ready Word 4"]
    #[inline(always)]
    pub fn lincalrdyw4(&self) -> Lincalrdyw4R {
        Lincalrdyw4R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Linearity calibration ready Word 5"]
    #[inline(always)]
    pub fn lincalrdyw5(&self) -> Lincalrdyw5R {
        Lincalrdyw5R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Linearity calibration ready Word 6"]
    #[inline(always)]
    pub fn lincalrdyw6(&self) -> Lincalrdyw6R {
        Lincalrdyw6R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    pub fn advregen(&self) -> AdvregenR {
        AdvregenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    pub fn deeppwd(&self) -> DeeppwdR {
        DeeppwdR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    pub fn adcaldif(&self) -> AdcaldifR {
        AdcaldifR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&self) -> AdcalR {
        AdcalR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC enable"]
    #[inline(always)]
    pub fn aden(&mut self) -> AdenW<CrSpec> {
        AdenW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC disable"]
    #[inline(always)]
    pub fn addis(&mut self) -> AddisW<CrSpec> {
        AddisW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular conversion start"]
    #[inline(always)]
    pub fn adstart(&mut self) -> AdstartW<CrSpec> {
        AdstartW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group injected conversion start"]
    #[inline(always)]
    pub fn jadstart(&mut self) -> JadstartW<CrSpec> {
        JadstartW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular conversion stop"]
    #[inline(always)]
    pub fn adstp(&mut self) -> AdstpW<CrSpec> {
        AdstpW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected conversion stop"]
    #[inline(always)]
    pub fn jadstp(&mut self) -> JadstpW<CrSpec> {
        JadstpW::new(self, 5)
    }
    #[doc = "Bits 8:9 - Boost mode control"]
    #[inline(always)]
    pub fn boost(&mut self) -> BoostW<CrSpec> {
        BoostW::new(self, 8)
    }
    #[doc = "Bit 16 - Linearity calibration"]
    #[inline(always)]
    pub fn adcallin(&mut self) -> AdcallinW<CrSpec> {
        AdcallinW::new(self, 16)
    }
    #[doc = "Bit 22 - Linearity calibration ready Word 1"]
    #[inline(always)]
    pub fn lincalrdyw1(&mut self) -> Lincalrdyw1W<CrSpec> {
        Lincalrdyw1W::new(self, 22)
    }
    #[doc = "Bit 23 - Linearity calibration ready Word 2"]
    #[inline(always)]
    pub fn lincalrdyw2(&mut self) -> Lincalrdyw2W<CrSpec> {
        Lincalrdyw2W::new(self, 23)
    }
    #[doc = "Bit 24 - Linearity calibration ready Word 3"]
    #[inline(always)]
    pub fn lincalrdyw3(&mut self) -> Lincalrdyw3W<CrSpec> {
        Lincalrdyw3W::new(self, 24)
    }
    #[doc = "Bit 25 - Linearity calibration ready Word 4"]
    #[inline(always)]
    pub fn lincalrdyw4(&mut self) -> Lincalrdyw4W<CrSpec> {
        Lincalrdyw4W::new(self, 25)
    }
    #[doc = "Bit 26 - Linearity calibration ready Word 5"]
    #[inline(always)]
    pub fn lincalrdyw5(&mut self) -> Lincalrdyw5W<CrSpec> {
        Lincalrdyw5W::new(self, 26)
    }
    #[doc = "Bit 27 - Linearity calibration ready Word 6"]
    #[inline(always)]
    pub fn lincalrdyw6(&mut self) -> Lincalrdyw6W<CrSpec> {
        Lincalrdyw6W::new(self, 27)
    }
    #[doc = "Bit 28 - ADC voltage regulator enable"]
    #[inline(always)]
    pub fn advregen(&mut self) -> AdvregenW<CrSpec> {
        AdvregenW::new(self, 28)
    }
    #[doc = "Bit 29 - ADC deep power down enable"]
    #[inline(always)]
    pub fn deeppwd(&mut self) -> DeeppwdW<CrSpec> {
        DeeppwdW::new(self, 29)
    }
    #[doc = "Bit 30 - ADC differential mode for calibration"]
    #[inline(always)]
    pub fn adcaldif(&mut self) -> AdcaldifW<CrSpec> {
        AdcaldifW::new(self, 30)
    }
    #[doc = "Bit 31 - ADC calibration"]
    #[inline(always)]
    pub fn adcal(&mut self) -> AdcalW<CrSpec> {
        AdcalW::new(self, 31)
    }
}
#[doc = "ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

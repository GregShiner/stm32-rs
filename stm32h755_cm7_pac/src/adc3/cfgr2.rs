#[doc = "Register `CFGR2` reader"]
pub type R = crate::R<Cfgr2Spec>;
#[doc = "Register `CFGR2` writer"]
pub type W = crate::W<Cfgr2Spec>;
#[doc = "Field `ROVSE` reader - ADC oversampler enable on scope ADC group regular"]
pub type RovseR = crate::BitReader;
#[doc = "Field `ROVSE` writer - ADC oversampler enable on scope ADC group regular"]
pub type RovseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JOVSE` reader - ADC oversampler enable on scope ADC group injected"]
pub type JovseR = crate::BitReader;
#[doc = "Field `JOVSE` writer - ADC oversampler enable on scope ADC group injected"]
pub type JovseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVSS` reader - ADC oversampling shift"]
pub type OvssR = crate::FieldReader;
#[doc = "Field `OVSS` writer - ADC oversampling shift"]
pub type OvssW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TROVS` reader - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub type TrovsR = crate::BitReader;
#[doc = "Field `TROVS` writer - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
pub type TrovsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVSM` reader - Regular Oversampling mode"]
pub type RovsmR = crate::BitReader;
#[doc = "Field `ROVSM` writer - Regular Oversampling mode"]
pub type RovsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSHIFT1` reader - Right-shift data after Offset 1 correction"]
pub type Rshift1R = crate::BitReader;
#[doc = "Field `RSHIFT1` writer - Right-shift data after Offset 1 correction"]
pub type Rshift1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSHIFT2` reader - Right-shift data after Offset 2 correction"]
pub type Rshift2R = crate::BitReader;
#[doc = "Field `RSHIFT2` writer - Right-shift data after Offset 2 correction"]
pub type Rshift2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSHIFT3` reader - Right-shift data after Offset 3 correction"]
pub type Rshift3R = crate::BitReader;
#[doc = "Field `RSHIFT3` writer - Right-shift data after Offset 3 correction"]
pub type Rshift3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSHIFT4` reader - Right-shift data after Offset 4 correction"]
pub type Rshift4R = crate::BitReader;
#[doc = "Field `RSHIFT4` writer - Right-shift data after Offset 4 correction"]
pub type Rshift4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSR` reader - Oversampling ratio"]
pub type OsrR = crate::FieldReader<u16>;
#[doc = "Field `OSR` writer - Oversampling ratio"]
pub type OsrW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LSHIFT` reader - Left shift factor"]
pub type LshiftR = crate::FieldReader;
#[doc = "Field `LSHIFT` writer - Left shift factor"]
pub type LshiftW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    pub fn rovse(&self) -> RovseR {
        RovseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jovse(&self) -> JovseR {
        JovseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    pub fn ovss(&self) -> OvssR {
        OvssR::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    pub fn trovs(&self) -> TrovsR {
        TrovsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&self) -> RovsmR {
        RovsmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Right-shift data after Offset 1 correction"]
    #[inline(always)]
    pub fn rshift1(&self) -> Rshift1R {
        Rshift1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Right-shift data after Offset 2 correction"]
    #[inline(always)]
    pub fn rshift2(&self) -> Rshift2R {
        Rshift2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Right-shift data after Offset 3 correction"]
    #[inline(always)]
    pub fn rshift3(&self) -> Rshift3R {
        Rshift3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Right-shift data after Offset 4 correction"]
    #[inline(always)]
    pub fn rshift4(&self) -> Rshift4R {
        Rshift4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Oversampling ratio"]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 28:31 - Left shift factor"]
    #[inline(always)]
    pub fn lshift(&self) -> LshiftR {
        LshiftR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC oversampler enable on scope ADC group regular"]
    #[inline(always)]
    pub fn rovse(&mut self) -> RovseW<Cfgr2Spec> {
        RovseW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC oversampler enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jovse(&mut self) -> JovseW<Cfgr2Spec> {
        JovseW::new(self, 1)
    }
    #[doc = "Bits 5:8 - ADC oversampling shift"]
    #[inline(always)]
    pub fn ovss(&mut self) -> OvssW<Cfgr2Spec> {
        OvssW::new(self, 5)
    }
    #[doc = "Bit 9 - ADC oversampling discontinuous mode (triggered mode) for ADC group regular"]
    #[inline(always)]
    pub fn trovs(&mut self) -> TrovsW<Cfgr2Spec> {
        TrovsW::new(self, 9)
    }
    #[doc = "Bit 10 - Regular Oversampling mode"]
    #[inline(always)]
    pub fn rovsm(&mut self) -> RovsmW<Cfgr2Spec> {
        RovsmW::new(self, 10)
    }
    #[doc = "Bit 11 - Right-shift data after Offset 1 correction"]
    #[inline(always)]
    pub fn rshift1(&mut self) -> Rshift1W<Cfgr2Spec> {
        Rshift1W::new(self, 11)
    }
    #[doc = "Bit 12 - Right-shift data after Offset 2 correction"]
    #[inline(always)]
    pub fn rshift2(&mut self) -> Rshift2W<Cfgr2Spec> {
        Rshift2W::new(self, 12)
    }
    #[doc = "Bit 13 - Right-shift data after Offset 3 correction"]
    #[inline(always)]
    pub fn rshift3(&mut self) -> Rshift3W<Cfgr2Spec> {
        Rshift3W::new(self, 13)
    }
    #[doc = "Bit 14 - Right-shift data after Offset 4 correction"]
    #[inline(always)]
    pub fn rshift4(&mut self) -> Rshift4W<Cfgr2Spec> {
        Rshift4W::new(self, 14)
    }
    #[doc = "Bits 16:25 - Oversampling ratio"]
    #[inline(always)]
    pub fn osr(&mut self) -> OsrW<Cfgr2Spec> {
        OsrW::new(self, 16)
    }
    #[doc = "Bits 28:31 - Left shift factor"]
    #[inline(always)]
    pub fn lshift(&mut self) -> LshiftW<Cfgr2Spec> {
        LshiftW::new(self, 28)
    }
}
#[doc = "ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr2Spec;
impl crate::RegisterSpec for Cfgr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr2::R`](R) reader structure"]
impl crate::Readable for Cfgr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr2::W`](W) writer structure"]
impl crate::Writable for Cfgr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR2 to value 0"]
impl crate::Resettable for Cfgr2Spec {}

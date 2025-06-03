#[doc = "Register `EECR2` reader"]
pub type R = crate::R<Eecr2Spec>;
#[doc = "Register `EECR2` writer"]
pub type W = crate::W<Eecr2Spec>;
#[doc = "Field `EE6SRC` reader - External Event 6 Source"]
pub type Ee6srcR = crate::FieldReader;
#[doc = "Field `EE6SRC` writer - External Event 6 Source"]
pub type Ee6srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE6POL` reader - External Event 6 Polarity"]
pub type Ee6polR = crate::BitReader;
#[doc = "Field `EE6POL` writer - External Event 6 Polarity"]
pub type Ee6polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE6SNS` reader - External Event 6 Sensitivity"]
pub type Ee6snsR = crate::FieldReader;
#[doc = "Field `EE6SNS` writer - External Event 6 Sensitivity"]
pub type Ee6snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE7SRC` reader - External Event 7 Source"]
pub type Ee7srcR = crate::FieldReader;
#[doc = "Field `EE7SRC` writer - External Event 7 Source"]
pub type Ee7srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE7POL` reader - External Event 7 Polarity"]
pub type Ee7polR = crate::BitReader;
#[doc = "Field `EE7POL` writer - External Event 7 Polarity"]
pub type Ee7polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE7SNS` reader - External Event 7 Sensitivity"]
pub type Ee7snsR = crate::FieldReader;
#[doc = "Field `EE7SNS` writer - External Event 7 Sensitivity"]
pub type Ee7snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE8SRC` reader - External Event 8 Source"]
pub type Ee8srcR = crate::FieldReader;
#[doc = "Field `EE8SRC` writer - External Event 8 Source"]
pub type Ee8srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE8POL` reader - External Event 8 Polarity"]
pub type Ee8polR = crate::BitReader;
#[doc = "Field `EE8POL` writer - External Event 8 Polarity"]
pub type Ee8polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE8SNS` reader - External Event 8 Sensitivity"]
pub type Ee8snsR = crate::FieldReader;
#[doc = "Field `EE8SNS` writer - External Event 8 Sensitivity"]
pub type Ee8snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE9SRC` reader - External Event 9 Source"]
pub type Ee9srcR = crate::FieldReader;
#[doc = "Field `EE9SRC` writer - External Event 9 Source"]
pub type Ee9srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE9POL` reader - External Event 9 Polarity"]
pub type Ee9polR = crate::BitReader;
#[doc = "Field `EE9POL` writer - External Event 9 Polarity"]
pub type Ee9polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE9SNS` reader - External Event 9 Sensitivity"]
pub type Ee9snsR = crate::FieldReader;
#[doc = "Field `EE9SNS` writer - External Event 9 Sensitivity"]
pub type Ee9snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE10SRC` reader - External Event 10 Source"]
pub type Ee10srcR = crate::FieldReader;
#[doc = "Field `EE10SRC` writer - External Event 10 Source"]
pub type Ee10srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EE10POL` reader - External Event 10 Polarity"]
pub type Ee10polR = crate::BitReader;
#[doc = "Field `EE10POL` writer - External Event 10 Polarity"]
pub type Ee10polW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE10SNS` reader - External Event 10 Sensitivity"]
pub type Ee10snsR = crate::FieldReader;
#[doc = "Field `EE10SNS` writer - External Event 10 Sensitivity"]
pub type Ee10snsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - External Event 6 Source"]
    #[inline(always)]
    pub fn ee6src(&self) -> Ee6srcR {
        Ee6srcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - External Event 6 Polarity"]
    #[inline(always)]
    pub fn ee6pol(&self) -> Ee6polR {
        Ee6polR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - External Event 6 Sensitivity"]
    #[inline(always)]
    pub fn ee6sns(&self) -> Ee6snsR {
        Ee6snsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Event 7 Source"]
    #[inline(always)]
    pub fn ee7src(&self) -> Ee7srcR {
        Ee7srcR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - External Event 7 Polarity"]
    #[inline(always)]
    pub fn ee7pol(&self) -> Ee7polR {
        Ee7polR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - External Event 7 Sensitivity"]
    #[inline(always)]
    pub fn ee7sns(&self) -> Ee7snsR {
        Ee7snsR::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 12:13 - External Event 8 Source"]
    #[inline(always)]
    pub fn ee8src(&self) -> Ee8srcR {
        Ee8srcR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External Event 8 Polarity"]
    #[inline(always)]
    pub fn ee8pol(&self) -> Ee8polR {
        Ee8polR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:16 - External Event 8 Sensitivity"]
    #[inline(always)]
    pub fn ee8sns(&self) -> Ee8snsR {
        Ee8snsR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 18:19 - External Event 9 Source"]
    #[inline(always)]
    pub fn ee9src(&self) -> Ee9srcR {
        Ee9srcR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - External Event 9 Polarity"]
    #[inline(always)]
    pub fn ee9pol(&self) -> Ee9polR {
        Ee9polR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - External Event 9 Sensitivity"]
    #[inline(always)]
    pub fn ee9sns(&self) -> Ee9snsR {
        Ee9snsR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - External Event 10 Source"]
    #[inline(always)]
    pub fn ee10src(&self) -> Ee10srcR {
        Ee10srcR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - External Event 10 Polarity"]
    #[inline(always)]
    pub fn ee10pol(&self) -> Ee10polR {
        Ee10polR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - External Event 10 Sensitivity"]
    #[inline(always)]
    pub fn ee10sns(&self) -> Ee10snsR {
        Ee10snsR::new(((self.bits >> 27) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - External Event 6 Source"]
    #[inline(always)]
    pub fn ee6src(&mut self) -> Ee6srcW<Eecr2Spec> {
        Ee6srcW::new(self, 0)
    }
    #[doc = "Bit 2 - External Event 6 Polarity"]
    #[inline(always)]
    pub fn ee6pol(&mut self) -> Ee6polW<Eecr2Spec> {
        Ee6polW::new(self, 2)
    }
    #[doc = "Bits 3:4 - External Event 6 Sensitivity"]
    #[inline(always)]
    pub fn ee6sns(&mut self) -> Ee6snsW<Eecr2Spec> {
        Ee6snsW::new(self, 3)
    }
    #[doc = "Bits 6:7 - External Event 7 Source"]
    #[inline(always)]
    pub fn ee7src(&mut self) -> Ee7srcW<Eecr2Spec> {
        Ee7srcW::new(self, 6)
    }
    #[doc = "Bit 8 - External Event 7 Polarity"]
    #[inline(always)]
    pub fn ee7pol(&mut self) -> Ee7polW<Eecr2Spec> {
        Ee7polW::new(self, 8)
    }
    #[doc = "Bits 9:10 - External Event 7 Sensitivity"]
    #[inline(always)]
    pub fn ee7sns(&mut self) -> Ee7snsW<Eecr2Spec> {
        Ee7snsW::new(self, 9)
    }
    #[doc = "Bits 12:13 - External Event 8 Source"]
    #[inline(always)]
    pub fn ee8src(&mut self) -> Ee8srcW<Eecr2Spec> {
        Ee8srcW::new(self, 12)
    }
    #[doc = "Bit 14 - External Event 8 Polarity"]
    #[inline(always)]
    pub fn ee8pol(&mut self) -> Ee8polW<Eecr2Spec> {
        Ee8polW::new(self, 14)
    }
    #[doc = "Bits 15:16 - External Event 8 Sensitivity"]
    #[inline(always)]
    pub fn ee8sns(&mut self) -> Ee8snsW<Eecr2Spec> {
        Ee8snsW::new(self, 15)
    }
    #[doc = "Bits 18:19 - External Event 9 Source"]
    #[inline(always)]
    pub fn ee9src(&mut self) -> Ee9srcW<Eecr2Spec> {
        Ee9srcW::new(self, 18)
    }
    #[doc = "Bit 20 - External Event 9 Polarity"]
    #[inline(always)]
    pub fn ee9pol(&mut self) -> Ee9polW<Eecr2Spec> {
        Ee9polW::new(self, 20)
    }
    #[doc = "Bits 21:22 - External Event 9 Sensitivity"]
    #[inline(always)]
    pub fn ee9sns(&mut self) -> Ee9snsW<Eecr2Spec> {
        Ee9snsW::new(self, 21)
    }
    #[doc = "Bits 24:25 - External Event 10 Source"]
    #[inline(always)]
    pub fn ee10src(&mut self) -> Ee10srcW<Eecr2Spec> {
        Ee10srcW::new(self, 24)
    }
    #[doc = "Bit 26 - External Event 10 Polarity"]
    #[inline(always)]
    pub fn ee10pol(&mut self) -> Ee10polW<Eecr2Spec> {
        Ee10polW::new(self, 26)
    }
    #[doc = "Bits 27:28 - External Event 10 Sensitivity"]
    #[inline(always)]
    pub fn ee10sns(&mut self) -> Ee10snsW<Eecr2Spec> {
        Ee10snsW::new(self, 27)
    }
}
#[doc = "Timer External Event Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`eecr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eecr2Spec;
impl crate::RegisterSpec for Eecr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eecr2::R`](R) reader structure"]
impl crate::Readable for Eecr2Spec {}
#[doc = "`write(|w| ..)` method takes [`eecr2::W`](W) writer structure"]
impl crate::Writable for Eecr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EECR2 to value 0"]
impl crate::Resettable for Eecr2Spec {}

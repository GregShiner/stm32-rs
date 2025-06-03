#[doc = "Register `SAI_PDMDLY` reader"]
pub type R = crate::R<SaiPdmdlySpec>;
#[doc = "Register `SAI_PDMDLY` writer"]
pub type W = crate::W<SaiPdmdlySpec>;
#[doc = "Field `DLYM1L` reader - Delay line adjust for first microphone of pair 1"]
pub type Dlym1lR = crate::FieldReader;
#[doc = "Field `DLYM1L` writer - Delay line adjust for first microphone of pair 1"]
pub type Dlym1lW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM1R` reader - Delay line adjust for second microphone of pair 1"]
pub type Dlym1rR = crate::FieldReader;
#[doc = "Field `DLYM1R` writer - Delay line adjust for second microphone of pair 1"]
pub type Dlym1rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM2L` reader - Delay line for first microphone of pair 2"]
pub type Dlym2lR = crate::FieldReader;
#[doc = "Field `DLYM2L` writer - Delay line for first microphone of pair 2"]
pub type Dlym2lW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM2R` reader - Delay line for second microphone of pair 2"]
pub type Dlym2rR = crate::FieldReader;
#[doc = "Field `DLYM2R` writer - Delay line for second microphone of pair 2"]
pub type Dlym2rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM3L` reader - Delay line for first microphone of pair 3"]
pub type Dlym3lR = crate::FieldReader;
#[doc = "Field `DLYM3L` writer - Delay line for first microphone of pair 3"]
pub type Dlym3lW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM3R` reader - Delay line for second microphone of pair 3"]
pub type Dlym3rR = crate::FieldReader;
#[doc = "Field `DLYM3R` writer - Delay line for second microphone of pair 3"]
pub type Dlym3rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM4L` reader - Delay line for first microphone of pair 4"]
pub type Dlym4lR = crate::FieldReader;
#[doc = "Field `DLYM4L` writer - Delay line for first microphone of pair 4"]
pub type Dlym4lW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DLYM4R` reader - Delay line for second microphone of pair 4"]
pub type Dlym4rR = crate::FieldReader;
#[doc = "Field `DLYM4R` writer - Delay line for second microphone of pair 4"]
pub type Dlym4rW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1l(&self) -> Dlym1lR {
        Dlym1lR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1r(&self) -> Dlym1rR {
        Dlym1rR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2l(&self) -> Dlym2lR {
        Dlym2lR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2r(&self) -> Dlym2rR {
        Dlym2rR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3l(&self) -> Dlym3lR {
        Dlym3lR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3r(&self) -> Dlym3rR {
        Dlym3rR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4l(&self) -> Dlym4lR {
        Dlym4lR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4r(&self) -> Dlym4rR {
        Dlym4rR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Delay line adjust for first microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1l(&mut self) -> Dlym1lW<SaiPdmdlySpec> {
        Dlym1lW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Delay line adjust for second microphone of pair 1"]
    #[inline(always)]
    pub fn dlym1r(&mut self) -> Dlym1rW<SaiPdmdlySpec> {
        Dlym1rW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Delay line for first microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2l(&mut self) -> Dlym2lW<SaiPdmdlySpec> {
        Dlym2lW::new(self, 8)
    }
    #[doc = "Bits 12:14 - Delay line for second microphone of pair 2"]
    #[inline(always)]
    pub fn dlym2r(&mut self) -> Dlym2rW<SaiPdmdlySpec> {
        Dlym2rW::new(self, 12)
    }
    #[doc = "Bits 16:18 - Delay line for first microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3l(&mut self) -> Dlym3lW<SaiPdmdlySpec> {
        Dlym3lW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Delay line for second microphone of pair 3"]
    #[inline(always)]
    pub fn dlym3r(&mut self) -> Dlym3rW<SaiPdmdlySpec> {
        Dlym3rW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Delay line for first microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4l(&mut self) -> Dlym4lW<SaiPdmdlySpec> {
        Dlym4lW::new(self, 24)
    }
    #[doc = "Bits 28:30 - Delay line for second microphone of pair 4"]
    #[inline(always)]
    pub fn dlym4r(&mut self) -> Dlym4rW<SaiPdmdlySpec> {
        Dlym4rW::new(self, 28)
    }
}
#[doc = "PDM delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`sai_pdmdly::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_pdmdly::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaiPdmdlySpec;
impl crate::RegisterSpec for SaiPdmdlySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_pdmdly::R`](R) reader structure"]
impl crate::Readable for SaiPdmdlySpec {}
#[doc = "`write(|w| ..)` method takes [`sai_pdmdly::W`](W) writer structure"]
impl crate::Writable for SaiPdmdlySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAI_PDMDLY to value 0"]
impl crate::Resettable for SaiPdmdlySpec {}

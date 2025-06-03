#[doc = "Register `FLTCR` reader"]
pub type R = crate::R<FltcrSpec>;
#[doc = "Register `FLTCR` writer"]
pub type W = crate::W<FltcrSpec>;
#[doc = "Field `FLT1EN` reader - Fault 1 enable"]
pub type Flt1enR = crate::BitReader;
#[doc = "Field `FLT1EN` writer - Fault 1 enable"]
pub type Flt1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT2EN` reader - Fault 2 enable"]
pub type Flt2enR = crate::BitReader;
#[doc = "Field `FLT2EN` writer - Fault 2 enable"]
pub type Flt2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT3EN` reader - Fault 3 enable"]
pub type Flt3enR = crate::BitReader;
#[doc = "Field `FLT3EN` writer - Fault 3 enable"]
pub type Flt3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT4EN` reader - Fault 4 enable"]
pub type Flt4enR = crate::BitReader;
#[doc = "Field `FLT4EN` writer - Fault 4 enable"]
pub type Flt4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5EN` reader - Fault 5 enable"]
pub type Flt5enR = crate::BitReader;
#[doc = "Field `FLT5EN` writer - Fault 5 enable"]
pub type Flt5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTLCK` reader - Fault sources Lock"]
pub type FltlckR = crate::BitReader;
#[doc = "Field `FLTLCK` writer - Fault sources Lock"]
pub type FltlckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&self) -> Flt1enR {
        Flt1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&self) -> Flt2enR {
        Flt2enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&self) -> Flt3enR {
        Flt3enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&self) -> Flt4enR {
        Flt4enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&self) -> Flt5enR {
        Flt5enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&self) -> FltlckR {
        FltlckR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Fault 1 enable"]
    #[inline(always)]
    pub fn flt1en(&mut self) -> Flt1enW<FltcrSpec> {
        Flt1enW::new(self, 0)
    }
    #[doc = "Bit 1 - Fault 2 enable"]
    #[inline(always)]
    pub fn flt2en(&mut self) -> Flt2enW<FltcrSpec> {
        Flt2enW::new(self, 1)
    }
    #[doc = "Bit 2 - Fault 3 enable"]
    #[inline(always)]
    pub fn flt3en(&mut self) -> Flt3enW<FltcrSpec> {
        Flt3enW::new(self, 2)
    }
    #[doc = "Bit 3 - Fault 4 enable"]
    #[inline(always)]
    pub fn flt4en(&mut self) -> Flt4enW<FltcrSpec> {
        Flt4enW::new(self, 3)
    }
    #[doc = "Bit 4 - Fault 5 enable"]
    #[inline(always)]
    pub fn flt5en(&mut self) -> Flt5enW<FltcrSpec> {
        Flt5enW::new(self, 4)
    }
    #[doc = "Bit 31 - Fault sources Lock"]
    #[inline(always)]
    pub fn fltlck(&mut self) -> FltlckW<FltcrSpec> {
        FltlckW::new(self, 31)
    }
}
#[doc = "Timerx Fault Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FltcrSpec;
impl crate::RegisterSpec for FltcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltcr::R`](R) reader structure"]
impl crate::Readable for FltcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fltcr::W`](W) writer structure"]
impl crate::Writable for FltcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTCR to value 0"]
impl crate::Resettable for FltcrSpec {}

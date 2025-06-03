#[doc = "Register `FLTINR2` reader"]
pub type R = crate::R<Fltinr2Spec>;
#[doc = "Register `FLTINR2` writer"]
pub type W = crate::W<Fltinr2Spec>;
#[doc = "Field `FLT5E` reader - FLT5E"]
pub type Flt5eR = crate::BitReader;
#[doc = "Field `FLT5E` writer - FLT5E"]
pub type Flt5eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5P` reader - FLT5P"]
pub type Flt5pR = crate::BitReader;
#[doc = "Field `FLT5P` writer - FLT5P"]
pub type Flt5pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5SRC` reader - FLT5SRC"]
pub type Flt5srcR = crate::BitReader;
#[doc = "Field `FLT5SRC` writer - FLT5SRC"]
pub type Flt5srcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLT5F` reader - FLT5F"]
pub type Flt5fR = crate::FieldReader;
#[doc = "Field `FLT5F` writer - FLT5F"]
pub type Flt5fW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FLT5LCK` reader - FLT5LCK"]
pub type Flt5lckR = crate::BitReader;
#[doc = "Field `FLT5LCK` writer - FLT5LCK"]
pub type Flt5lckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLTSD` reader - FLTSD"]
pub type FltsdR = crate::FieldReader;
#[doc = "Field `FLTSD` writer - FLTSD"]
pub type FltsdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&self) -> Flt5eR {
        Flt5eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&self) -> Flt5pR {
        Flt5pR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&self) -> Flt5srcR {
        Flt5srcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&self) -> Flt5fR {
        Flt5fR::new(((self.bits >> 3) & 0x0f) as u8)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&self) -> Flt5lckR {
        Flt5lckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&self) -> FltsdR {
        FltsdR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLT5E"]
    #[inline(always)]
    pub fn flt5e(&mut self) -> Flt5eW<Fltinr2Spec> {
        Flt5eW::new(self, 0)
    }
    #[doc = "Bit 1 - FLT5P"]
    #[inline(always)]
    pub fn flt5p(&mut self) -> Flt5pW<Fltinr2Spec> {
        Flt5pW::new(self, 1)
    }
    #[doc = "Bit 2 - FLT5SRC"]
    #[inline(always)]
    pub fn flt5src(&mut self) -> Flt5srcW<Fltinr2Spec> {
        Flt5srcW::new(self, 2)
    }
    #[doc = "Bits 3:6 - FLT5F"]
    #[inline(always)]
    pub fn flt5f(&mut self) -> Flt5fW<Fltinr2Spec> {
        Flt5fW::new(self, 3)
    }
    #[doc = "Bit 7 - FLT5LCK"]
    #[inline(always)]
    pub fn flt5lck(&mut self) -> Flt5lckW<Fltinr2Spec> {
        Flt5lckW::new(self, 7)
    }
    #[doc = "Bits 24:25 - FLTSD"]
    #[inline(always)]
    pub fn fltsd(&mut self) -> FltsdW<Fltinr2Spec> {
        FltsdW::new(self, 24)
    }
}
#[doc = "HRTIM Fault Input Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fltinr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fltinr2Spec;
impl crate::RegisterSpec for Fltinr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fltinr2::R`](R) reader structure"]
impl crate::Readable for Fltinr2Spec {}
#[doc = "`write(|w| ..)` method takes [`fltinr2::W`](W) writer structure"]
impl crate::Writable for Fltinr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLTINR2 to value 0"]
impl crate::Resettable for Fltinr2Spec {}

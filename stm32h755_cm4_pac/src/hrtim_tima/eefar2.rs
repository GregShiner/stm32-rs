#[doc = "Register `EEFAR2` reader"]
pub type R = crate::R<Eefar2Spec>;
#[doc = "Register `EEFAR2` writer"]
pub type W = crate::W<Eefar2Spec>;
#[doc = "Field `EE6LTCH` reader - External Event 6 latch"]
pub type Ee6ltchR = crate::BitReader;
#[doc = "Field `EE6LTCH` writer - External Event 6 latch"]
pub type Ee6ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE6FLTR` reader - External Event 6 filter"]
pub type Ee6fltrR = crate::FieldReader;
#[doc = "Field `EE6FLTR` writer - External Event 6 filter"]
pub type Ee6fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE7LTCH` reader - External Event 7 latch"]
pub type Ee7ltchR = crate::BitReader;
#[doc = "Field `EE7LTCH` writer - External Event 7 latch"]
pub type Ee7ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE7FLTR` reader - External Event 7 filter"]
pub type Ee7fltrR = crate::FieldReader;
#[doc = "Field `EE7FLTR` writer - External Event 7 filter"]
pub type Ee7fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE8LTCH` reader - External Event 8 latch"]
pub type Ee8ltchR = crate::BitReader;
#[doc = "Field `EE8LTCH` writer - External Event 8 latch"]
pub type Ee8ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE8FLTR` reader - External Event 8 filter"]
pub type Ee8fltrR = crate::FieldReader;
#[doc = "Field `EE8FLTR` writer - External Event 8 filter"]
pub type Ee8fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE9LTCH` reader - External Event 9 latch"]
pub type Ee9ltchR = crate::BitReader;
#[doc = "Field `EE9LTCH` writer - External Event 9 latch"]
pub type Ee9ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE9FLTR` reader - External Event 9 filter"]
pub type Ee9fltrR = crate::FieldReader;
#[doc = "Field `EE9FLTR` writer - External Event 9 filter"]
pub type Ee9fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE10LTCH` reader - External Event 10 latch"]
pub type Ee10ltchR = crate::BitReader;
#[doc = "Field `EE10LTCH` writer - External Event 10 latch"]
pub type Ee10ltchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE10FLTR` reader - External Event 10 filter"]
pub type Ee10fltrR = crate::FieldReader;
#[doc = "Field `EE10FLTR` writer - External Event 10 filter"]
pub type Ee10fltrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&self) -> Ee6ltchR {
        Ee6ltchR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&self) -> Ee6fltrR {
        Ee6fltrR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&self) -> Ee7ltchR {
        Ee7ltchR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&self) -> Ee7fltrR {
        Ee7fltrR::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&self) -> Ee8ltchR {
        Ee8ltchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&self) -> Ee8fltrR {
        Ee8fltrR::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&self) -> Ee9ltchR {
        Ee9ltchR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&self) -> Ee9fltrR {
        Ee9fltrR::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&self) -> Ee10ltchR {
        Ee10ltchR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&self) -> Ee10fltrR {
        Ee10fltrR::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&mut self) -> Ee6ltchW<Eefar2Spec> {
        Ee6ltchW::new(self, 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&mut self) -> Ee6fltrW<Eefar2Spec> {
        Ee6fltrW::new(self, 1)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&mut self) -> Ee7ltchW<Eefar2Spec> {
        Ee7ltchW::new(self, 6)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&mut self) -> Ee7fltrW<Eefar2Spec> {
        Ee7fltrW::new(self, 7)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&mut self) -> Ee8ltchW<Eefar2Spec> {
        Ee8ltchW::new(self, 12)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&mut self) -> Ee8fltrW<Eefar2Spec> {
        Ee8fltrW::new(self, 13)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&mut self) -> Ee9ltchW<Eefar2Spec> {
        Ee9ltchW::new(self, 18)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&mut self) -> Ee9fltrW<Eefar2Spec> {
        Ee9fltrW::new(self, 19)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&mut self) -> Ee10ltchW<Eefar2Spec> {
        Ee10ltchW::new(self, 24)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&mut self) -> Ee10fltrW<Eefar2Spec> {
        Ee10fltrW::new(self, 25)
    }
}
#[doc = "Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`eefar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Eefar2Spec;
impl crate::RegisterSpec for Eefar2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefar2::R`](R) reader structure"]
impl crate::Readable for Eefar2Spec {}
#[doc = "`write(|w| ..)` method takes [`eefar2::W`](W) writer structure"]
impl crate::Writable for Eefar2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EEFAR2 to value 0"]
impl crate::Resettable for Eefar2Spec {}

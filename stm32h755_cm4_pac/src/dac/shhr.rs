#[doc = "Register `SHHR` reader"]
pub type R = crate::R<ShhrSpec>;
#[doc = "Register `SHHR` writer"]
pub type W = crate::W<ShhrSpec>;
#[doc = "Field `THOLD1` reader - DAC Channel 1 hold Time (only valid in sample &amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type Thold1R = crate::FieldReader<u16>;
#[doc = "Field `THOLD1` writer - DAC Channel 1 hold Time (only valid in sample &amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type Thold1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `THOLD2` reader - DAC Channel 2 hold time (only valid in sample &amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type Thold2R = crate::FieldReader<u16>;
#[doc = "Field `THOLD2` writer - DAC Channel 2 hold time (only valid in sample &amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
pub type Thold2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in sample &amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn thold1(&self) -> Thold1R {
        Thold1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - DAC Channel 2 hold time (only valid in sample &amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn thold2(&self) -> Thold2R {
        Thold2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - DAC Channel 1 hold Time (only valid in sample &amp; hold mode) Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn thold1(&mut self) -> Thold1W<ShhrSpec> {
        Thold1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - DAC Channel 2 hold time (only valid in sample &amp; hold mode). Hold time= (THOLD\\[9:0\\]) x T LSI"]
    #[inline(always)]
    pub fn thold2(&mut self) -> Thold2W<ShhrSpec> {
        Thold2W::new(self, 16)
    }
}
#[doc = "DAC Sample and Hold hold time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShhrSpec;
impl crate::RegisterSpec for ShhrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shhr::R`](R) reader structure"]
impl crate::Readable for ShhrSpec {}
#[doc = "`write(|w| ..)` method takes [`shhr::W`](W) writer structure"]
impl crate::Writable for ShhrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHHR to value 0x0001_0001"]
impl crate::Resettable for ShhrSpec {
    const RESET_VALUE: u32 = 0x0001_0001;
}

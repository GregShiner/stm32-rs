#[doc = "Register `BCCR` reader"]
pub type R = crate::R<BccrSpec>;
#[doc = "Register `BCCR` writer"]
pub type W = crate::W<BccrSpec>;
#[doc = "Field `BCBLUE` reader - Background Color Blue value"]
pub type BcblueR = crate::FieldReader;
#[doc = "Field `BCBLUE` writer - Background Color Blue value"]
pub type BcblueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BCGREEN` reader - Background Color Green value"]
pub type BcgreenR = crate::FieldReader;
#[doc = "Field `BCGREEN` writer - Background Color Green value"]
pub type BcgreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BCRED` reader - Background Color Red value"]
pub type BcredR = crate::FieldReader;
#[doc = "Field `BCRED` writer - Background Color Red value"]
pub type BcredW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Background Color Blue value"]
    #[inline(always)]
    pub fn bcblue(&self) -> BcblueR {
        BcblueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Background Color Green value"]
    #[inline(always)]
    pub fn bcgreen(&self) -> BcgreenR {
        BcgreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Background Color Red value"]
    #[inline(always)]
    pub fn bcred(&self) -> BcredR {
        BcredR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Background Color Blue value"]
    #[inline(always)]
    pub fn bcblue(&mut self) -> BcblueW<BccrSpec> {
        BcblueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Background Color Green value"]
    #[inline(always)]
    pub fn bcgreen(&mut self) -> BcgreenW<BccrSpec> {
        BcgreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Background Color Red value"]
    #[inline(always)]
    pub fn bcred(&mut self) -> BcredW<BccrSpec> {
        BcredW::new(self, 16)
    }
}
#[doc = "Background Color Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BccrSpec;
impl crate::RegisterSpec for BccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bccr::R`](R) reader structure"]
impl crate::Readable for BccrSpec {}
#[doc = "`write(|w| ..)` method takes [`bccr::W`](W) writer structure"]
impl crate::Writable for BccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCCR to value 0"]
impl crate::Resettable for BccrSpec {}

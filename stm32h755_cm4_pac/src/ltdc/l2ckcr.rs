#[doc = "Register `L2CKCR` reader"]
pub type R = crate::R<L2ckcrSpec>;
#[doc = "Register `L2CKCR` writer"]
pub type W = crate::W<L2ckcrSpec>;
#[doc = "Field `CKBLUE` reader - Color Key Blue value"]
pub type CkblueR = crate::FieldReader;
#[doc = "Field `CKBLUE` writer - Color Key Blue value"]
pub type CkblueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKGREEN` reader - Color Key Green value"]
pub type CkgreenR = crate::FieldReader;
#[doc = "Field `CKGREEN` writer - Color Key Green value"]
pub type CkgreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CKRED` reader - Color Key Red value"]
pub type CkredR = crate::FieldReader;
#[doc = "Field `CKRED` writer - Color Key Red value"]
pub type CkredW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&self) -> CkblueR {
        CkblueR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&self) -> CkgreenR {
        CkgreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&self) -> CkredR {
        CkredR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Color Key Blue value"]
    #[inline(always)]
    pub fn ckblue(&mut self) -> CkblueW<L2ckcrSpec> {
        CkblueW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Color Key Green value"]
    #[inline(always)]
    pub fn ckgreen(&mut self) -> CkgreenW<L2ckcrSpec> {
        CkgreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Color Key Red value"]
    #[inline(always)]
    pub fn ckred(&mut self) -> CkredW<L2ckcrSpec> {
        CkredW::new(self, 16)
    }
}
#[doc = "Layerx Color Keying Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2ckcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2ckcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2ckcrSpec;
impl crate::RegisterSpec for L2ckcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2ckcr::R`](R) reader structure"]
impl crate::Readable for L2ckcrSpec {}
#[doc = "`write(|w| ..)` method takes [`l2ckcr::W`](W) writer structure"]
impl crate::Writable for L2ckcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2CKCR to value 0"]
impl crate::Resettable for L2ckcrSpec {}

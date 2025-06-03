#[doc = "Register `CALFACT2` reader"]
pub type R = crate::R<Calfact2Spec>;
#[doc = "Register `CALFACT2` writer"]
pub type W = crate::W<Calfact2Spec>;
#[doc = "Field `LINCALFACT` reader - Linearity Calibration Factor"]
pub type LincalfactR = crate::FieldReader<u32>;
#[doc = "Field `LINCALFACT` writer - Linearity Calibration Factor"]
pub type LincalfactW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    pub fn lincalfact(&self) -> LincalfactR {
        LincalfactR::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    pub fn lincalfact(&mut self) -> LincalfactW<Calfact2Spec> {
        LincalfactW::new(self, 0)
    }
}
#[doc = "ADC Calibration Factor register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`calfact2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Calfact2Spec;
impl crate::RegisterSpec for Calfact2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfact2::R`](R) reader structure"]
impl crate::Readable for Calfact2Spec {}
#[doc = "`write(|w| ..)` method takes [`calfact2::W`](W) writer structure"]
impl crate::Writable for Calfact2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALFACT2 to value 0"]
impl crate::Resettable for Calfact2Spec {}

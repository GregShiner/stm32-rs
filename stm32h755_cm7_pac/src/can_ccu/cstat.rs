#[doc = "Register `CSTAT` reader"]
pub type R = crate::R<CstatSpec>;
#[doc = "Register `CSTAT` writer"]
pub type W = crate::W<CstatSpec>;
#[doc = "Field `OCPC` reader - Oscillator Clock Period Counter"]
pub type OcpcR = crate::FieldReader<u32>;
#[doc = "Field `OCPC` writer - Oscillator Clock Period Counter"]
pub type OcpcW<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
#[doc = "Field `TQC` reader - Time Quanta Counter"]
pub type TqcR = crate::FieldReader<u16>;
#[doc = "Field `TQC` writer - Time Quanta Counter"]
pub type TqcW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CALS` reader - Calibration State"]
pub type CalsR = crate::FieldReader;
#[doc = "Field `CALS` writer - Calibration State"]
pub type CalsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&self) -> OcpcR {
        OcpcR::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&self) -> TqcR {
        TqcR::new(((self.bits >> 18) & 0x07ff) as u16)
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&self) -> CalsR {
        CalsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:17 - Oscillator Clock Period Counter"]
    #[inline(always)]
    pub fn ocpc(&mut self) -> OcpcW<CstatSpec> {
        OcpcW::new(self, 0)
    }
    #[doc = "Bits 18:28 - Time Quanta Counter"]
    #[inline(always)]
    pub fn tqc(&mut self) -> TqcW<CstatSpec> {
        TqcW::new(self, 18)
    }
    #[doc = "Bits 30:31 - Calibration State"]
    #[inline(always)]
    pub fn cals(&mut self) -> CalsW<CstatSpec> {
        CalsW::new(self, 30)
    }
}
#[doc = "Calibration Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CstatSpec;
impl crate::RegisterSpec for CstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cstat::R`](R) reader structure"]
impl crate::Readable for CstatSpec {}
#[doc = "`write(|w| ..)` method takes [`cstat::W`](W) writer structure"]
impl crate::Writable for CstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSTAT to value 0"]
impl crate::Resettable for CstatSpec {}

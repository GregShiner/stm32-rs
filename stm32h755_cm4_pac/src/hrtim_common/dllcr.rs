#[doc = "Register `DLLCR` reader"]
pub type R = crate::R<DllcrSpec>;
#[doc = "Register `DLLCR` writer"]
pub type W = crate::W<DllcrSpec>;
#[doc = "Field `CAL` reader - DLL Calibration Start"]
pub type CalR = crate::BitReader;
#[doc = "Field `CAL` writer - DLL Calibration Start"]
pub type CalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALEN` reader - DLL Calibration Enable"]
pub type CalenR = crate::BitReader;
#[doc = "Field `CALEN` writer - DLL Calibration Enable"]
pub type CalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALRTE` reader - DLL Calibration rate"]
pub type CalrteR = crate::FieldReader;
#[doc = "Field `CALRTE` writer - DLL Calibration rate"]
pub type CalrteW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&self) -> CalR {
        CalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&self) -> CalenR {
        CalenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&self) -> CalrteR {
        CalrteR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DLL Calibration Start"]
    #[inline(always)]
    pub fn cal(&mut self) -> CalW<DllcrSpec> {
        CalW::new(self, 0)
    }
    #[doc = "Bit 1 - DLL Calibration Enable"]
    #[inline(always)]
    pub fn calen(&mut self) -> CalenW<DllcrSpec> {
        CalenW::new(self, 1)
    }
    #[doc = "Bits 2:3 - DLL Calibration rate"]
    #[inline(always)]
    pub fn calrte(&mut self) -> CalrteW<DllcrSpec> {
        CalrteW::new(self, 2)
    }
}
#[doc = "DLL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dllcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DllcrSpec;
impl crate::RegisterSpec for DllcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dllcr::R`](R) reader structure"]
impl crate::Readable for DllcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dllcr::W`](W) writer structure"]
impl crate::Writable for DllcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLLCR to value 0"]
impl crate::Resettable for DllcrSpec {}

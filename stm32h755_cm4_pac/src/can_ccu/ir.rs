#[doc = "Register `IR` reader"]
pub type R = crate::R<IrSpec>;
#[doc = "Register `IR` writer"]
pub type W = crate::W<IrSpec>;
#[doc = "Field `CWE` reader - Calibration Watchdog Event"]
pub type CweR = crate::BitReader;
#[doc = "Field `CWE` writer - Calibration Watchdog Event"]
pub type CweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC` reader - Calibration State Changed"]
pub type CscR = crate::BitReader;
#[doc = "Field `CSC` writer - Calibration State Changed"]
pub type CscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Calibration Watchdog Event"]
    #[inline(always)]
    pub fn cwe(&self) -> CweR {
        CweR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Calibration State Changed"]
    #[inline(always)]
    pub fn csc(&self) -> CscR {
        CscR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Watchdog Event"]
    #[inline(always)]
    pub fn cwe(&mut self) -> CweW<IrSpec> {
        CweW::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration State Changed"]
    #[inline(always)]
    pub fn csc(&mut self) -> CscW<IrSpec> {
        CscW::new(self, 1)
    }
}
#[doc = "Clock Calibration Unit Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrSpec;
impl crate::RegisterSpec for IrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ir::R`](R) reader structure"]
impl crate::Readable for IrSpec {}
#[doc = "`write(|w| ..)` method takes [`ir::W`](W) writer structure"]
impl crate::Writable for IrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IrSpec {}

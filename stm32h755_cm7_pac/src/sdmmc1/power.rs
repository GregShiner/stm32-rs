#[doc = "Register `POWER` reader"]
pub type R = crate::R<PowerSpec>;
#[doc = "Register `POWER` writer"]
pub type W = crate::W<PowerSpec>;
#[doc = "Field `PWRCTRL` reader - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
pub type PwrctrlR = crate::FieldReader;
#[doc = "Field `PWRCTRL` writer - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
pub type PwrctrlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VSWITCH` reader - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
pub type VswitchR = crate::BitReader;
#[doc = "Field `VSWITCH` writer - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
pub type VswitchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWITCHEN` reader - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
pub type VswitchenR = crate::BitReader;
#[doc = "Field `VSWITCHEN` writer - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
pub type VswitchenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIRPOL` reader - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
pub type DirpolR = crate::BitReader;
#[doc = "Field `DIRPOL` writer - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
pub type DirpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
    #[inline(always)]
    pub fn pwrctrl(&self) -> PwrctrlR {
        PwrctrlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
    #[inline(always)]
    pub fn vswitch(&self) -> VswitchR {
        VswitchR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
    #[inline(always)]
    pub fn vswitchen(&self) -> VswitchenR {
        VswitchenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
    #[inline(always)]
    pub fn dirpol(&self) -> DirpolR {
        DirpolR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SDMMC state control bits. These bits can only be written when the SDMMC is not in the power-on state (PWRCTRL?11). These bits are used to define the functional state of the SDMMC signals: Any further write will be ignored, PWRCTRL value will keep 11."]
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PwrctrlW<PowerSpec> {
        PwrctrlW::new(self, 0)
    }
    #[doc = "Bit 2 - Voltage switch sequence start. This bit is used to start the timing critical section of the voltage switch sequence:"]
    #[inline(always)]
    pub fn vswitch(&mut self) -> VswitchW<PowerSpec> {
        VswitchW::new(self, 2)
    }
    #[doc = "Bit 3 - Voltage switch procedure enable. This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). This bit is used to stop the SDMMC_CK after the voltage switch command response:"]
    #[inline(always)]
    pub fn vswitchen(&mut self) -> VswitchenW<PowerSpec> {
        VswitchenW::new(self, 3)
    }
    #[doc = "Bit 4 - Data and command direction signals polarity selection. This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00)."]
    #[inline(always)]
    pub fn dirpol(&mut self) -> DirpolW<PowerSpec> {
        DirpolW::new(self, 4)
    }
}
#[doc = "SDMMC power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PowerSpec;
impl crate::RegisterSpec for PowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power::R`](R) reader structure"]
impl crate::Readable for PowerSpec {}
#[doc = "`write(|w| ..)` method takes [`power::W`](W) writer structure"]
impl crate::Writable for PowerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POWER to value 0"]
impl crate::Resettable for PowerSpec {}

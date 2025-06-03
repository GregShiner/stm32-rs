#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `BREN` reader - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
pub type BrenR = crate::BitReader;
#[doc = "Field `BREN` writer - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
pub type BrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MONEN` reader - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
pub type MonenR = crate::BitReader;
#[doc = "Field `MONEN` writer - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
pub type MonenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRRDY` reader - Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
pub type BrrdyR = crate::BitReader;
#[doc = "Field `VBATL` reader - VBAT level monitoring versus low threshold"]
pub type VbatlR = crate::BitReader;
#[doc = "Field `VBATH` reader - VBAT level monitoring versus high threshold"]
pub type VbathR = crate::BitReader;
#[doc = "Field `TEMPL` reader - Temperature level monitoring versus low threshold"]
pub type TemplR = crate::BitReader;
#[doc = "Field `TEMPH` reader - Temperature level monitoring versus high threshold"]
pub type TemphR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
    #[inline(always)]
    pub fn bren(&self) -> BrenR {
        BrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
    #[inline(always)]
    pub fn monen(&self) -> MonenR {
        MonenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - Backup regulator ready This bit is set by hardware to indicate that the Backup regulator is ready."]
    #[inline(always)]
    pub fn brrdy(&self) -> BrrdyR {
        BrrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - VBAT level monitoring versus low threshold"]
    #[inline(always)]
    pub fn vbatl(&self) -> VbatlR {
        VbatlR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - VBAT level monitoring versus high threshold"]
    #[inline(always)]
    pub fn vbath(&self) -> VbathR {
        VbathR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Temperature level monitoring versus low threshold"]
    #[inline(always)]
    pub fn templ(&self) -> TemplR {
        TemplR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature level monitoring versus high threshold"]
    #[inline(always)]
    pub fn temph(&self) -> TemphR {
        TemphR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Backup regulator enable When set, the Backup regulator (used to maintain the backup RAM content in Standby and VBAT modes) is enabled. If BREN is reset, the backup regulator is switched off. The backup RAM can still be used in Run and Stop modes. However, its content will be lost in Standby and VBAT modes. If BREN is set, the application must wait till the Backup Regulator Ready flag (BRRDY) is set to indicate that the data written into the SRAM will be maintained in Standby and VBAT modes."]
    #[inline(always)]
    pub fn bren(&mut self) -> BrenW<Cr2Spec> {
        BrenW::new(self, 0)
    }
    #[doc = "Bit 4 - VBAT and temperature monitoring enable When set, the VBAT supply and temperature monitoring is enabled."]
    #[inline(always)]
    pub fn monen(&mut self) -> MonenW<Cr2Spec> {
        MonenW::new(self, 4)
    }
}
#[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}

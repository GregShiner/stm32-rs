#[doc = "Register `OPAMP2_CSR` reader"]
pub type R = crate::R<Opamp2CsrSpec>;
#[doc = "Register `OPAMP2_CSR` writer"]
pub type W = crate::W<Opamp2CsrSpec>;
#[doc = "Field `OPAEN` reader - Operational amplifier Enable"]
pub type OpaenR = crate::BitReader;
#[doc = "Field `OPAEN` writer - Operational amplifier Enable"]
pub type OpaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_VP` reader - Force internal reference on VP (reserved for test)"]
pub type ForceVpR = crate::BitReader;
#[doc = "Field `FORCE_VP` writer - Force internal reference on VP (reserved for test)"]
pub type ForceVpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VM_SEL` reader - Inverting input selection"]
pub type VmSelR = crate::FieldReader;
#[doc = "Field `VM_SEL` writer - Inverting input selection"]
pub type VmSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OPAHSM` reader - Operational amplifier high-speed mode"]
pub type OpahsmR = crate::BitReader;
#[doc = "Field `OPAHSM` writer - Operational amplifier high-speed mode"]
pub type OpahsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALON` reader - Calibration mode enabled"]
pub type CalonR = crate::BitReader;
#[doc = "Field `CALON` writer - Calibration mode enabled"]
pub type CalonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALSEL` reader - Calibration selection"]
pub type CalselR = crate::FieldReader;
#[doc = "Field `CALSEL` writer - Calibration selection"]
pub type CalselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PGA_GAIN` reader - Operational amplifier Programmable amplifier gain value"]
pub type PgaGainR = crate::FieldReader;
#[doc = "Field `PGA_GAIN` writer - Operational amplifier Programmable amplifier gain value"]
pub type PgaGainW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `USERTRIM` reader - User trimming enable"]
pub type UsertrimR = crate::BitReader;
#[doc = "Field `USERTRIM` writer - User trimming enable"]
pub type UsertrimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSTREF` reader - OPAMP calibration reference voltage output control (reserved for test)"]
pub type TstrefR = crate::BitReader;
#[doc = "Field `TSTREF` writer - OPAMP calibration reference voltage output control (reserved for test)"]
pub type TstrefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CALOUT` reader - Operational amplifier calibration output"]
pub type CaloutR = crate::BitReader;
#[doc = "Field `CALOUT` writer - Operational amplifier calibration output"]
pub type CaloutW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&self) -> OpaenR {
        OpaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    pub fn force_vp(&self) -> ForceVpR {
        ForceVpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&self) -> VmSelR {
        VmSelR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode"]
    #[inline(always)]
    pub fn opahsm(&self) -> OpahsmR {
        OpahsmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&self) -> CalonR {
        CalonR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&self) -> CalselR {
        CalselR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&self) -> PgaGainR {
        PgaGainR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn usertrim(&self) -> UsertrimR {
        UsertrimR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    pub fn tstref(&self) -> TstrefR {
        TstrefR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&self) -> CaloutR {
        CaloutR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Operational amplifier Enable"]
    #[inline(always)]
    pub fn opaen(&mut self) -> OpaenW<Opamp2CsrSpec> {
        OpaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Force internal reference on VP (reserved for test)"]
    #[inline(always)]
    pub fn force_vp(&mut self) -> ForceVpW<Opamp2CsrSpec> {
        ForceVpW::new(self, 1)
    }
    #[doc = "Bits 5:6 - Inverting input selection"]
    #[inline(always)]
    pub fn vm_sel(&mut self) -> VmSelW<Opamp2CsrSpec> {
        VmSelW::new(self, 5)
    }
    #[doc = "Bit 8 - Operational amplifier high-speed mode"]
    #[inline(always)]
    pub fn opahsm(&mut self) -> OpahsmW<Opamp2CsrSpec> {
        OpahsmW::new(self, 8)
    }
    #[doc = "Bit 11 - Calibration mode enabled"]
    #[inline(always)]
    pub fn calon(&mut self) -> CalonW<Opamp2CsrSpec> {
        CalonW::new(self, 11)
    }
    #[doc = "Bits 12:13 - Calibration selection"]
    #[inline(always)]
    pub fn calsel(&mut self) -> CalselW<Opamp2CsrSpec> {
        CalselW::new(self, 12)
    }
    #[doc = "Bits 14:17 - Operational amplifier Programmable amplifier gain value"]
    #[inline(always)]
    pub fn pga_gain(&mut self) -> PgaGainW<Opamp2CsrSpec> {
        PgaGainW::new(self, 14)
    }
    #[doc = "Bit 18 - User trimming enable"]
    #[inline(always)]
    pub fn usertrim(&mut self) -> UsertrimW<Opamp2CsrSpec> {
        UsertrimW::new(self, 18)
    }
    #[doc = "Bit 29 - OPAMP calibration reference voltage output control (reserved for test)"]
    #[inline(always)]
    pub fn tstref(&mut self) -> TstrefW<Opamp2CsrSpec> {
        TstrefW::new(self, 29)
    }
    #[doc = "Bit 30 - Operational amplifier calibration output"]
    #[inline(always)]
    pub fn calout(&mut self) -> CaloutW<Opamp2CsrSpec> {
        CaloutW::new(self, 30)
    }
}
#[doc = "OPAMP2 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opamp2CsrSpec;
impl crate::RegisterSpec for Opamp2CsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opamp2_csr::R`](R) reader structure"]
impl crate::Readable for Opamp2CsrSpec {}
#[doc = "`write(|w| ..)` method takes [`opamp2_csr::W`](W) writer structure"]
impl crate::Writable for Opamp2CsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPAMP2_CSR to value 0"]
impl crate::Resettable for Opamp2CsrSpec {}

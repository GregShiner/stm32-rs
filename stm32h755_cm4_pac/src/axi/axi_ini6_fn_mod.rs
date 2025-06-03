#[doc = "Register `AXI_INI6_FN_MOD` reader"]
pub type R = crate::R<AxiIni6FnModSpec>;
#[doc = "Register `AXI_INI6_FN_MOD` writer"]
pub type W = crate::W<AxiIni6FnModSpec>;
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override ASIB read issuing capability"]
pub type ReadIssOverrideR = crate::BitReader;
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override ASIB read issuing capability"]
pub type ReadIssOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override ASIB write issuing capability"]
pub type WriteIssOverrideR = crate::BitReader;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override ASIB write issuing capability"]
pub type WriteIssOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> ReadIssOverrideR {
        ReadIssOverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WriteIssOverrideR {
        WriteIssOverrideR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override ASIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> ReadIssOverrideW<AxiIni6FnModSpec> {
        ReadIssOverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - Override ASIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WriteIssOverrideW<AxiIni6FnModSpec> {
        WriteIssOverrideW::new(self, 1)
    }
}
#[doc = "AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini6_fn_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini6_fn_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiIni6FnModSpec;
impl crate::RegisterSpec for AxiIni6FnModSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_ini6_fn_mod::R`](R) reader structure"]
impl crate::Readable for AxiIni6FnModSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_ini6_fn_mod::W`](W) writer structure"]
impl crate::Writable for AxiIni6FnModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_INI6_FN_MOD to value 0x04"]
impl crate::Resettable for AxiIni6FnModSpec {
    const RESET_VALUE: u32 = 0x04;
}

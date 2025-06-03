#[doc = "Register `AXI_TARG2_FN_MOD` reader"]
pub type R = crate::R<AxiTarg2FnModSpec>;
#[doc = "Register `AXI_TARG2_FN_MOD` writer"]
pub type W = crate::W<AxiTarg2FnModSpec>;
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override AMIB read issuing capability"]
pub type ReadIssOverrideR = crate::BitReader;
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override AMIB read issuing capability"]
pub type ReadIssOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override AMIB write issuing capability"]
pub type WriteIssOverrideR = crate::BitReader;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override AMIB write issuing capability"]
pub type WriteIssOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Override AMIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> ReadIssOverrideR {
        ReadIssOverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override AMIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WriteIssOverrideR {
        WriteIssOverrideR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override AMIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> ReadIssOverrideW<AxiTarg2FnModSpec> {
        ReadIssOverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - Override AMIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WriteIssOverrideW<AxiTarg2FnModSpec> {
        WriteIssOverrideW::new(self, 1)
    }
}
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ2_fn_mod::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ2_fn_mod::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiTarg2FnModSpec;
impl crate::RegisterSpec for AxiTarg2FnModSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_targ2_fn_mod::R`](R) reader structure"]
impl crate::Readable for AxiTarg2FnModSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_targ2_fn_mod::W`](W) writer structure"]
impl crate::Writable for AxiTarg2FnModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_TARG2_FN_MOD to value 0x04"]
impl crate::Resettable for AxiTarg2FnModSpec {
    const RESET_VALUE: u32 = 0x04;
}

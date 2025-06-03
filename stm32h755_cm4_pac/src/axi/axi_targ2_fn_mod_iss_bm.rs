#[doc = "Register `AXI_TARG2_FN_MOD_ISS_BM` reader"]
pub type R = crate::R<AxiTarg2FnModIssBmSpec>;
#[doc = "Register `AXI_TARG2_FN_MOD_ISS_BM` writer"]
pub type W = crate::W<AxiTarg2FnModIssBmSpec>;
#[doc = "Field `READ_ISS_OVERRIDE` reader - READ_ISS_OVERRIDE"]
pub type ReadIssOverrideR = crate::BitReader;
#[doc = "Field `READ_ISS_OVERRIDE` writer - READ_ISS_OVERRIDE"]
pub type ReadIssOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Switch matrix write issuing override for target"]
pub type WriteIssOverrideR = crate::BitReader;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Switch matrix write issuing override for target"]
pub type WriteIssOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - READ_ISS_OVERRIDE"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> ReadIssOverrideR {
        ReadIssOverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Switch matrix write issuing override for target"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WriteIssOverrideR {
        WriteIssOverrideR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - READ_ISS_OVERRIDE"]
    #[inline(always)]
    pub fn read_iss_override(&mut self) -> ReadIssOverrideW<AxiTarg2FnModIssBmSpec> {
        ReadIssOverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - Switch matrix write issuing override for target"]
    #[inline(always)]
    pub fn write_iss_override(&mut self) -> WriteIssOverrideW<AxiTarg2FnModIssBmSpec> {
        WriteIssOverrideW::new(self, 1)
    }
}
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ2_fn_mod_iss_bm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ2_fn_mod_iss_bm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiTarg2FnModIssBmSpec;
impl crate::RegisterSpec for AxiTarg2FnModIssBmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_targ2_fn_mod_iss_bm::R`](R) reader structure"]
impl crate::Readable for AxiTarg2FnModIssBmSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_targ2_fn_mod_iss_bm::W`](W) writer structure"]
impl crate::Writable for AxiTarg2FnModIssBmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_TARG2_FN_MOD_ISS_BM to value 0x04"]
impl crate::Resettable for AxiTarg2FnModIssBmSpec {
    const RESET_VALUE: u32 = 0x04;
}

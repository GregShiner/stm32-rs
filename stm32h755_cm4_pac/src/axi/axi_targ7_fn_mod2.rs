#[doc = "Register `AXI_TARG7_FN_MOD2` reader"]
pub type R = crate::R<AxiTarg7FnMod2Spec>;
#[doc = "Register `AXI_TARG7_FN_MOD2` writer"]
pub type W = crate::W<AxiTarg7FnMod2Spec>;
#[doc = "Field `BYPASS_MERGE` reader - Disable packing of beats to match the output data width"]
pub type BypassMergeR = crate::BitReader;
#[doc = "Field `BYPASS_MERGE` writer - Disable packing of beats to match the output data width"]
pub type BypassMergeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disable packing of beats to match the output data width"]
    #[inline(always)]
    pub fn bypass_merge(&self) -> BypassMergeR {
        BypassMergeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable packing of beats to match the output data width"]
    #[inline(always)]
    pub fn bypass_merge(&mut self) -> BypassMergeW<AxiTarg7FnMod2Spec> {
        BypassMergeW::new(self, 0)
    }
}
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ7_fn_mod2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ7_fn_mod2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiTarg7FnMod2Spec;
impl crate::RegisterSpec for AxiTarg7FnMod2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_targ7_fn_mod2::R`](R) reader structure"]
impl crate::Readable for AxiTarg7FnMod2Spec {}
#[doc = "`write(|w| ..)` method takes [`axi_targ7_fn_mod2::W`](W) writer structure"]
impl crate::Writable for AxiTarg7FnMod2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_TARG7_FN_MOD2 to value 0x04"]
impl crate::Resettable for AxiTarg7FnMod2Spec {
    const RESET_VALUE: u32 = 0x04;
}

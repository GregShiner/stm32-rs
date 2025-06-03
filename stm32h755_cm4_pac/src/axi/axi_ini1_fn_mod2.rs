#[doc = "Register `AXI_INI1_FN_MOD2` reader"]
pub type R = crate::R<AxiIni1FnMod2Spec>;
#[doc = "Register `AXI_INI1_FN_MOD2` writer"]
pub type W = crate::W<AxiIni1FnMod2Spec>;
#[doc = "Field `BYPASS_MERGE` reader - Disables alteration of transactions by the up-sizer unless required by the protocol"]
pub type BypassMergeR = crate::BitReader;
#[doc = "Field `BYPASS_MERGE` writer - Disables alteration of transactions by the up-sizer unless required by the protocol"]
pub type BypassMergeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disables alteration of transactions by the up-sizer unless required by the protocol"]
    #[inline(always)]
    pub fn bypass_merge(&self) -> BypassMergeR {
        BypassMergeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disables alteration of transactions by the up-sizer unless required by the protocol"]
    #[inline(always)]
    pub fn bypass_merge(&mut self) -> BypassMergeW<AxiIni1FnMod2Spec> {
        BypassMergeW::new(self, 0)
    }
}
#[doc = "AXI interconnect - INI x functionality modification 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini1_fn_mod2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini1_fn_mod2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiIni1FnMod2Spec;
impl crate::RegisterSpec for AxiIni1FnMod2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_ini1_fn_mod2::R`](R) reader structure"]
impl crate::Readable for AxiIni1FnMod2Spec {}
#[doc = "`write(|w| ..)` method takes [`axi_ini1_fn_mod2::W`](W) writer structure"]
impl crate::Writable for AxiIni1FnMod2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_INI1_FN_MOD2 to value 0x04"]
impl crate::Resettable for AxiIni1FnMod2Spec {
    const RESET_VALUE: u32 = 0x04;
}

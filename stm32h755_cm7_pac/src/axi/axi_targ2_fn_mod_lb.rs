#[doc = "Register `AXI_TARG2_FN_MOD_LB` reader"]
pub type R = crate::R<AxiTarg2FnModLbSpec>;
#[doc = "Register `AXI_TARG2_FN_MOD_LB` writer"]
pub type W = crate::W<AxiTarg2FnModLbSpec>;
#[doc = "Field `FN_MOD_LB` reader - Controls burst breaking of long bursts"]
pub type FnModLbR = crate::BitReader;
#[doc = "Field `FN_MOD_LB` writer - Controls burst breaking of long bursts"]
pub type FnModLbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&self) -> FnModLbR {
        FnModLbR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls burst breaking of long bursts"]
    #[inline(always)]
    pub fn fn_mod_lb(&mut self) -> FnModLbW<AxiTarg2FnModLbSpec> {
        FnModLbW::new(self, 0)
    }
}
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ2_fn_mod_lb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ2_fn_mod_lb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiTarg2FnModLbSpec;
impl crate::RegisterSpec for AxiTarg2FnModLbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_targ2_fn_mod_lb::R`](R) reader structure"]
impl crate::Readable for AxiTarg2FnModLbSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_targ2_fn_mod_lb::W`](W) writer structure"]
impl crate::Writable for AxiTarg2FnModLbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_TARG2_FN_MOD_LB to value 0x04"]
impl crate::Resettable for AxiTarg2FnModLbSpec {
    const RESET_VALUE: u32 = 0x04;
}

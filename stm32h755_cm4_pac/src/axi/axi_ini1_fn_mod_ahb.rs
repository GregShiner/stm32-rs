#[doc = "Register `AXI_INI1_FN_MOD_AHB` reader"]
pub type R = crate::R<AxiIni1FnModAhbSpec>;
#[doc = "Register `AXI_INI1_FN_MOD_AHB` writer"]
pub type W = crate::W<AxiIni1FnModAhbSpec>;
#[doc = "Field `RD_INC_OVERRIDE` reader - Converts all AHB-Lite write transactions to a series of single beat AXI"]
pub type RdIncOverrideR = crate::BitReader;
#[doc = "Field `RD_INC_OVERRIDE` writer - Converts all AHB-Lite write transactions to a series of single beat AXI"]
pub type RdIncOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WR_INC_OVERRIDE` reader - Converts all AHB-Lite read transactions to a series of single beat AXI"]
pub type WrIncOverrideR = crate::BitReader;
#[doc = "Field `WR_INC_OVERRIDE` writer - Converts all AHB-Lite read transactions to a series of single beat AXI"]
pub type WrIncOverrideW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn rd_inc_override(&self) -> RdIncOverrideR {
        RdIncOverrideR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn wr_inc_override(&self) -> WrIncOverrideR {
        WrIncOverrideR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Converts all AHB-Lite write transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn rd_inc_override(&mut self) -> RdIncOverrideW<AxiIni1FnModAhbSpec> {
        RdIncOverrideW::new(self, 0)
    }
    #[doc = "Bit 1 - Converts all AHB-Lite read transactions to a series of single beat AXI"]
    #[inline(always)]
    pub fn wr_inc_override(&mut self) -> WrIncOverrideW<AxiIni1FnModAhbSpec> {
        WrIncOverrideW::new(self, 1)
    }
}
#[doc = "AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini1_fn_mod_ahb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini1_fn_mod_ahb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiIni1FnModAhbSpec;
impl crate::RegisterSpec for AxiIni1FnModAhbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_ini1_fn_mod_ahb::R`](R) reader structure"]
impl crate::Readable for AxiIni1FnModAhbSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_ini1_fn_mod_ahb::W`](W) writer structure"]
impl crate::Writable for AxiIni1FnModAhbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_INI1_FN_MOD_AHB to value 0x04"]
impl crate::Resettable for AxiIni1FnModAhbSpec {
    const RESET_VALUE: u32 = 0x04;
}

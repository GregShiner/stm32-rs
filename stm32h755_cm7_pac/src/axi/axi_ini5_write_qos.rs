#[doc = "Register `AXI_INI5_WRITE_QOS` reader"]
pub type R = crate::R<AxiIni5WriteQosSpec>;
#[doc = "Register `AXI_INI5_WRITE_QOS` writer"]
pub type W = crate::W<AxiIni5WriteQosSpec>;
#[doc = "Field `AW_QOS` reader - Write channel QoS setting"]
pub type AwQosR = crate::FieldReader;
#[doc = "Field `AW_QOS` writer - Write channel QoS setting"]
pub type AwQosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Write channel QoS setting"]
    #[inline(always)]
    pub fn aw_qos(&self) -> AwQosR {
        AwQosR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Write channel QoS setting"]
    #[inline(always)]
    pub fn aw_qos(&mut self) -> AwQosW<AxiIni5WriteQosSpec> {
        AwQosW::new(self, 0)
    }
}
#[doc = "AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini5_write_qos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini5_write_qos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiIni5WriteQosSpec;
impl crate::RegisterSpec for AxiIni5WriteQosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_ini5_write_qos::R`](R) reader structure"]
impl crate::Readable for AxiIni5WriteQosSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_ini5_write_qos::W`](W) writer structure"]
impl crate::Writable for AxiIni5WriteQosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_INI5_WRITE_QOS to value 0x04"]
impl crate::Resettable for AxiIni5WriteQosSpec {
    const RESET_VALUE: u32 = 0x04;
}

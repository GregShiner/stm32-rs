#[doc = "Register `AXI_INI5_READ_QOS` reader"]
pub type R = crate::R<AxiIni5ReadQosSpec>;
#[doc = "Register `AXI_INI5_READ_QOS` writer"]
pub type W = crate::W<AxiIni5ReadQosSpec>;
#[doc = "Field `AR_QOS` reader - Read channel QoS setting"]
pub type ArQosR = crate::FieldReader;
#[doc = "Field `AR_QOS` writer - Read channel QoS setting"]
pub type ArQosW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Read channel QoS setting"]
    #[inline(always)]
    pub fn ar_qos(&self) -> ArQosR {
        ArQosR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Read channel QoS setting"]
    #[inline(always)]
    pub fn ar_qos(&mut self) -> ArQosW<AxiIni5ReadQosSpec> {
        ArQosW::new(self, 0)
    }
}
#[doc = "AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini5_read_qos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini5_read_qos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiIni5ReadQosSpec;
impl crate::RegisterSpec for AxiIni5ReadQosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_ini5_read_qos::R`](R) reader structure"]
impl crate::Readable for AxiIni5ReadQosSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_ini5_read_qos::W`](W) writer structure"]
impl crate::Writable for AxiIni5ReadQosSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_INI5_READ_QOS to value 0x04"]
impl crate::Resettable for AxiIni5ReadQosSpec {
    const RESET_VALUE: u32 = 0x04;
}

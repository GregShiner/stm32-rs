#[doc = "Register `SWTRGR` writer"]
pub type W = crate::W<SwtrgrSpec>;
#[doc = "Field `SWTRIG1` writer - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
pub type Swtrig1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWTRIG2` writer - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register."]
pub type Swtrig2W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - DAC channel1 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR1 register value has been loaded into the DAC_DOR1 register."]
    #[inline(always)]
    pub fn swtrig1(&mut self) -> Swtrig1W<SwtrgrSpec> {
        Swtrig1W::new(self, 0)
    }
    #[doc = "Bit 1 - DAC channel2 software trigger This bit is set by software to trigger the DAC in software trigger mode. Note: This bit is cleared by hardware (one APB1 clock cycle later) once the DAC_DHR2 register value has been loaded into the DAC_DOR2 register."]
    #[inline(always)]
    pub fn swtrig2(&mut self) -> Swtrig2W<SwtrgrSpec> {
        Swtrig2W::new(self, 1)
    }
}
#[doc = "DAC software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SwtrgrSpec;
impl crate::RegisterSpec for SwtrgrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`swtrgr::W`](W) writer structure"]
impl crate::Writable for SwtrgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWTRGR to value 0"]
impl crate::Resettable for SwtrgrSpec {}

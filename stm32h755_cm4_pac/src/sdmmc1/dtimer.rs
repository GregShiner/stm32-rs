#[doc = "Register `DTIMER` reader"]
pub type R = crate::R<DtimerSpec>;
#[doc = "Register `DTIMER` writer"]
pub type W = crate::W<DtimerSpec>;
#[doc = "Field `DATATIME` reader - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
pub type DatatimeR = crate::FieldReader<u32>;
#[doc = "Field `DATATIME` writer - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
pub type DatatimeW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
    #[inline(always)]
    pub fn datatime(&self) -> DatatimeR {
        DatatimeR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data and R1b busy timeout period This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and DPSMACT = 0). Data and R1b busy timeout period expressed in card bus clock periods."]
    #[inline(always)]
    pub fn datatime(&mut self) -> DatatimeW<DtimerSpec> {
        DatatimeW::new(self, 0)
    }
}
#[doc = "The SDMMC_DTIMER register contains the data timeout period, in card bus clock periods. A counter loads the value from the SDMMC_DTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the DPSM is in either of these states, the timeout status flag is set.\n\nYou can [`read`](crate::Reg::read) this register and get [`dtimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtimerSpec;
impl crate::RegisterSpec for DtimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtimer::R`](R) reader structure"]
impl crate::Readable for DtimerSpec {}
#[doc = "`write(|w| ..)` method takes [`dtimer::W`](W) writer structure"]
impl crate::Writable for DtimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTIMER to value 0"]
impl crate::Resettable for DtimerSpec {}

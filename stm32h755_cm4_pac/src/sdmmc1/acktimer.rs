#[doc = "Register `ACKTIMER` reader"]
pub type R = crate::R<AcktimerSpec>;
#[doc = "Register `ACKTIMER` writer"]
pub type W = crate::W<AcktimerSpec>;
#[doc = "Field `ACKTIME` reader - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
pub type AcktimeR = crate::FieldReader<u32>;
#[doc = "Field `ACKTIME` writer - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
pub type AcktimeW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
    #[inline(always)]
    pub fn acktime(&self) -> AcktimeR {
        AcktimeR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Boot acknowledgment timeout period This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0). Boot acknowledgment timeout period expressed in card bus clock periods."]
    #[inline(always)]
    pub fn acktime(&mut self) -> AcktimeW<AcktimerSpec> {
        AcktimeW::new(self, 0)
    }
}
#[doc = "The SDMMC_ACKTIMER register contains the acknowledgment timeout period, in SDMMC_CK bus clock periods. A counter loads the value from the SDMMC_ACKTIMER register, and starts decrementing when the data path state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is in this states, the acknowledgment timeout status flag is set.\n\nYou can [`read`](crate::Reg::read) this register and get [`acktimer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acktimer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcktimerSpec;
impl crate::RegisterSpec for AcktimerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acktimer::R`](R) reader structure"]
impl crate::Readable for AcktimerSpec {}
#[doc = "`write(|w| ..)` method takes [`acktimer::W`](W) writer structure"]
impl crate::Writable for AcktimerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACKTIMER to value 0"]
impl crate::Resettable for AcktimerSpec {}

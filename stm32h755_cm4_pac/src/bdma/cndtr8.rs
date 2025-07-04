#[doc = "Register `CNDTR8` reader"]
pub type R = crate::R<Cndtr8Spec>;
#[doc = "Register `CNDTR8` writer"]
pub type W = crate::W<Cndtr8Spec>;
#[doc = "Field `NDT` reader - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
pub type NdtR = crate::FieldReader<u16>;
#[doc = "Field `NDT` writer - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
pub type NdtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
    #[inline(always)]
    pub fn ndt(&self) -> NdtR {
        NdtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data to transfer Number of data to be transferred (0 up to 65535). This register can only be written when the channel is disabled. Once the channel is enabled, this register is read-only, indicating the remaining bytes to be transmitted. This register decrements after each DMA transfer. Once the transfer is completed, this register can either stay at zero or be reloaded automatically by the value previously programmed if the channel is configured in auto-reload mode. If this register is zero, no transaction can be served whether the channel is enabled or not."]
    #[inline(always)]
    pub fn ndt(&mut self) -> NdtW<Cndtr8Spec> {
        NdtW::new(self, 0)
    }
}
#[doc = "DMA channel x number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`cndtr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndtr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cndtr8Spec;
impl crate::RegisterSpec for Cndtr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cndtr8::R`](R) reader structure"]
impl crate::Readable for Cndtr8Spec {}
#[doc = "`write(|w| ..)` method takes [`cndtr8::W`](W) writer structure"]
impl crate::Writable for Cndtr8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNDTR8 to value 0"]
impl crate::Resettable for Cndtr8Spec {}

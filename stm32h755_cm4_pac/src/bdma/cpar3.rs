#[doc = "Register `CPAR3` reader"]
pub type R = crate::R<Cpar3Spec>;
#[doc = "Register `CPAR3` writer"]
pub type W = crate::W<Cpar3Spec>;
#[doc = "Field `PA` reader - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
pub type PaR = crate::FieldReader<u32>;
#[doc = "Field `PA` writer - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Peripheral address Base address of the peripheral data register from/to which the data will be read/written. When PSIZE is 01 (16-bit), the PA\\[0\\] bit is ignored. Access is automatically aligned to a half-word address. When PSIZE is 10 (32-bit), PA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<Cpar3Spec> {
        PaW::new(self, 0)
    }
}
#[doc = "This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpar3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpar3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpar3Spec;
impl crate::RegisterSpec for Cpar3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpar3::R`](R) reader structure"]
impl crate::Readable for Cpar3Spec {}
#[doc = "`write(|w| ..)` method takes [`cpar3::W`](W) writer structure"]
impl crate::Writable for Cpar3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPAR3 to value 0"]
impl crate::Resettable for Cpar3Spec {}

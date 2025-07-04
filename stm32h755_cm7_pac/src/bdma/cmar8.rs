#[doc = "Register `CMAR8` reader"]
pub type R = crate::R<Cmar8Spec>;
#[doc = "Register `CMAR8` writer"]
pub type W = crate::W<Cmar8Spec>;
#[doc = "Field `MA` reader - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\] bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
pub type MaR = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\] bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
pub type MaW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\] bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn ma(&self) -> MaR {
        MaR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address Base address of the memory area from/to which the data will be read/written. When MSIZE is 01 (16-bit), the MA\\[0\\] bit is ignored. Access is automatically aligned to a half-word address. When MSIZE is 10 (32-bit), MA\\[1:0\\] are ignored. Access is automatically aligned to a word address."]
    #[inline(always)]
    pub fn ma(&mut self) -> MaW<Cmar8Spec> {
        MaW::new(self, 0)
    }
}
#[doc = "This register must not be written when the channel is enabled.\n\nYou can [`read`](crate::Reg::read) this register and get [`cmar8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmar8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmar8Spec;
impl crate::RegisterSpec for Cmar8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar8::R`](R) reader structure"]
impl crate::Readable for Cmar8Spec {}
#[doc = "`write(|w| ..)` method takes [`cmar8::W`](W) writer structure"]
impl crate::Writable for Cmar8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMAR8 to value 0"]
impl crate::Resettable for Cmar8Spec {}

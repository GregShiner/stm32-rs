#[doc = "Register `RXCRC` reader"]
pub type R = crate::R<RxcrcSpec>;
#[doc = "Register `RXCRC` writer"]
pub type W = crate::W<RxcrcSpec>;
#[doc = "Field `RXCRC` reader - CRC register for receiver"]
pub type RxcrcR = crate::FieldReader<u32>;
#[doc = "Field `RXCRC` writer - CRC register for receiver"]
pub type RxcrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&self) -> RxcrcR {
        RxcrcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC register for receiver"]
    #[inline(always)]
    pub fn rxcrc(&mut self) -> RxcrcW<RxcrcSpec> {
        RxcrcW::new(self, 0)
    }
}
#[doc = "Receiver CRC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxcrcSpec;
impl crate::RegisterSpec for RxcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxcrc::R`](R) reader structure"]
impl crate::Readable for RxcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`rxcrc::W`](W) writer structure"]
impl crate::Writable for RxcrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXCRC to value 0"]
impl crate::Resettable for RxcrcSpec {}

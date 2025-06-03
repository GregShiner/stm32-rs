#[doc = "Register `TXCRC` reader"]
pub type R = crate::R<TxcrcSpec>;
#[doc = "Register `TXCRC` writer"]
pub type W = crate::W<TxcrcSpec>;
#[doc = "Field `TXCRC` reader - CRC register for transmitter"]
pub type TxcrcR = crate::FieldReader<u32>;
#[doc = "Field `TXCRC` writer - CRC register for transmitter"]
pub type TxcrcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC register for transmitter"]
    #[inline(always)]
    pub fn txcrc(&self) -> TxcrcR {
        TxcrcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC register for transmitter"]
    #[inline(always)]
    pub fn txcrc(&mut self) -> TxcrcW<TxcrcSpec> {
        TxcrcW::new(self, 0)
    }
}
#[doc = "Transmitter CRC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txcrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txcrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxcrcSpec;
impl crate::RegisterSpec for TxcrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txcrc::R`](R) reader structure"]
impl crate::Readable for TxcrcSpec {}
#[doc = "`write(|w| ..)` method takes [`txcrc::W`](W) writer structure"]
impl crate::Writable for TxcrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXCRC to value 0"]
impl crate::Resettable for TxcrcSpec {}

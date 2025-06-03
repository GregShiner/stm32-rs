#[doc = "Register `CRCEADD1R` reader"]
pub type R = crate::R<Crceadd1rSpec>;
#[doc = "Register `CRCEADD1R` writer"]
pub type W = crate::W<Crceadd1rSpec>;
#[doc = "Field `CRC_END_ADDR` reader - CRC end address on bank 1"]
pub type CrcEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `CRC_END_ADDR` writer - CRC end address on bank 1"]
pub type CrcEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC end address on bank 1"]
    #[inline(always)]
    pub fn crc_end_addr(&self) -> CrcEndAddrR {
        CrcEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC end address on bank 1"]
    #[inline(always)]
    pub fn crc_end_addr(&mut self) -> CrcEndAddrW<Crceadd1rSpec> {
        CrcEndAddrW::new(self, 0)
    }
}
#[doc = "FLASH CRC end address register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crceadd1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceadd1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crceadd1rSpec;
impl crate::RegisterSpec for Crceadd1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crceadd1r::R`](R) reader structure"]
impl crate::Readable for Crceadd1rSpec {}
#[doc = "`write(|w| ..)` method takes [`crceadd1r::W`](W) writer structure"]
impl crate::Writable for Crceadd1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCEADD1R to value 0"]
impl crate::Resettable for Crceadd1rSpec {}

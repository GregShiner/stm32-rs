#[doc = "Register `CRCEADD2R` reader"]
pub type R = crate::R<Crceadd2rSpec>;
#[doc = "Register `CRCEADD2R` writer"]
pub type W = crate::W<Crceadd2rSpec>;
#[doc = "Field `CRC_END_ADDR` reader - CRC end address on bank 2"]
pub type CrcEndAddrR = crate::FieldReader<u32>;
#[doc = "Field `CRC_END_ADDR` writer - CRC end address on bank 2"]
pub type CrcEndAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC end address on bank 2"]
    #[inline(always)]
    pub fn crc_end_addr(&self) -> CrcEndAddrR {
        CrcEndAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC end address on bank 2"]
    #[inline(always)]
    pub fn crc_end_addr(&mut self) -> CrcEndAddrW<Crceadd2rSpec> {
        CrcEndAddrW::new(self, 0)
    }
}
#[doc = "FLASH CRC end address register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`crceadd2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceadd2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crceadd2rSpec;
impl crate::RegisterSpec for Crceadd2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crceadd2r::R`](R) reader structure"]
impl crate::Readable for Crceadd2rSpec {}
#[doc = "`write(|w| ..)` method takes [`crceadd2r::W`](W) writer structure"]
impl crate::Writable for Crceadd2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCEADD2R to value 0"]
impl crate::Resettable for Crceadd2rSpec {}

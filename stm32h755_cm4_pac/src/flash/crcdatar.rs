#[doc = "Register `CRCDATAR` reader"]
pub type R = crate::R<CrcdatarSpec>;
#[doc = "Register `CRCDATAR` writer"]
pub type W = crate::W<CrcdatarSpec>;
#[doc = "Field `CRC_DATA` reader - CRC result"]
pub type CrcDataR = crate::FieldReader<u32>;
#[doc = "Field `CRC_DATA` writer - CRC result"]
pub type CrcDataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    pub fn crc_data(&self) -> CrcDataR {
        CrcDataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC result"]
    #[inline(always)]
    pub fn crc_data(&mut self) -> CrcDataW<CrcdatarSpec> {
        CrcDataW::new(self, 0)
    }
}
#[doc = "FLASH CRC data register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdatar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdatar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcdatarSpec;
impl crate::RegisterSpec for CrcdatarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcdatar::R`](R) reader structure"]
impl crate::Readable for CrcdatarSpec {}
#[doc = "`write(|w| ..)` method takes [`crcdatar::W`](W) writer structure"]
impl crate::Writable for CrcdatarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCDATAR to value 0"]
impl crate::Resettable for CrcdatarSpec {}

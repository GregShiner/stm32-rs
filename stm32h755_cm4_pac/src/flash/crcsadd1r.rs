#[doc = "Register `CRCSADD1R` reader"]
pub type R = crate::R<Crcsadd1rSpec>;
#[doc = "Register `CRCSADD1R` writer"]
pub type W = crate::W<Crcsadd1rSpec>;
#[doc = "Field `CRC_START_ADDR` reader - CRC start address on bank 1"]
pub type CrcStartAddrR = crate::FieldReader<u32>;
#[doc = "Field `CRC_START_ADDR` writer - CRC start address on bank 1"]
pub type CrcStartAddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - CRC start address on bank 1"]
    #[inline(always)]
    pub fn crc_start_addr(&self) -> CrcStartAddrR {
        CrcStartAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - CRC start address on bank 1"]
    #[inline(always)]
    pub fn crc_start_addr(&mut self) -> CrcStartAddrW<Crcsadd1rSpec> {
        CrcStartAddrW::new(self, 0)
    }
}
#[doc = "FLASH CRC start address register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsadd1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsadd1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crcsadd1rSpec;
impl crate::RegisterSpec for Crcsadd1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crcsadd1r::R`](R) reader structure"]
impl crate::Readable for Crcsadd1rSpec {}
#[doc = "`write(|w| ..)` method takes [`crcsadd1r::W`](W) writer structure"]
impl crate::Writable for Crcsadd1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCSADD1R to value 0"]
impl crate::Resettable for Crcsadd1rSpec {}

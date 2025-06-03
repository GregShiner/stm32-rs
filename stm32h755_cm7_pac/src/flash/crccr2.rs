#[doc = "Register `CRCCR2` reader"]
pub type R = crate::R<Crccr2Spec>;
#[doc = "Register `CRCCR2` writer"]
pub type W = crate::W<Crccr2Spec>;
#[doc = "Field `CRC_SECT` reader - Bank 2 CRC sector number"]
pub type CrcSectR = crate::FieldReader;
#[doc = "Field `CRC_SECT` writer - Bank 2 CRC sector number"]
pub type CrcSectW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ALL_BANK` reader - Bank 2 CRC select bit"]
pub type AllBankR = crate::BitReader;
#[doc = "Field `ALL_BANK` writer - Bank 2 CRC select bit"]
pub type AllBankW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_BY_SECT` reader - Bank 2 CRC sector mode select bit"]
pub type CrcBySectR = crate::BitReader;
#[doc = "Field `CRC_BY_SECT` writer - Bank 2 CRC sector mode select bit"]
pub type CrcBySectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD_SECT` reader - Bank 2 CRC sector select bit"]
pub type AddSectR = crate::BitReader;
#[doc = "Field `ADD_SECT` writer - Bank 2 CRC sector select bit"]
pub type AddSectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAN_SECT` reader - Bank 2 CRC sector list clear bit"]
pub type CleanSectR = crate::BitReader;
#[doc = "Field `CLEAN_SECT` writer - Bank 2 CRC sector list clear bit"]
pub type CleanSectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START_CRC` reader - Bank 2 CRC start bit"]
pub type StartCrcR = crate::BitReader;
#[doc = "Field `START_CRC` writer - Bank 2 CRC start bit"]
pub type StartCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAN_CRC` reader - Bank 2 CRC clear bit"]
pub type CleanCrcR = crate::BitReader;
#[doc = "Field `CLEAN_CRC` writer - Bank 2 CRC clear bit"]
pub type CleanCrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_BURST` reader - Bank 2 CRC burst size"]
pub type CrcBurstR = crate::FieldReader;
#[doc = "Field `CRC_BURST` writer - Bank 2 CRC burst size"]
pub type CrcBurstW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Bank 2 CRC sector number"]
    #[inline(always)]
    pub fn crc_sect(&self) -> CrcSectR {
        CrcSectR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Bank 2 CRC select bit"]
    #[inline(always)]
    pub fn all_bank(&self) -> AllBankR {
        AllBankR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bank 2 CRC sector mode select bit"]
    #[inline(always)]
    pub fn crc_by_sect(&self) -> CrcBySectR {
        CrcBySectR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bank 2 CRC sector select bit"]
    #[inline(always)]
    pub fn add_sect(&self) -> AddSectR {
        AddSectR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bank 2 CRC sector list clear bit"]
    #[inline(always)]
    pub fn clean_sect(&self) -> CleanSectR {
        CleanSectR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 2 CRC start bit"]
    #[inline(always)]
    pub fn start_crc(&self) -> StartCrcR {
        StartCrcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 2 CRC clear bit"]
    #[inline(always)]
    pub fn clean_crc(&self) -> CleanCrcR {
        CleanCrcR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Bank 2 CRC burst size"]
    #[inline(always)]
    pub fn crc_burst(&self) -> CrcBurstR {
        CrcBurstR::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Bank 2 CRC sector number"]
    #[inline(always)]
    pub fn crc_sect(&mut self) -> CrcSectW<Crccr2Spec> {
        CrcSectW::new(self, 0)
    }
    #[doc = "Bit 7 - Bank 2 CRC select bit"]
    #[inline(always)]
    pub fn all_bank(&mut self) -> AllBankW<Crccr2Spec> {
        AllBankW::new(self, 7)
    }
    #[doc = "Bit 8 - Bank 2 CRC sector mode select bit"]
    #[inline(always)]
    pub fn crc_by_sect(&mut self) -> CrcBySectW<Crccr2Spec> {
        CrcBySectW::new(self, 8)
    }
    #[doc = "Bit 9 - Bank 2 CRC sector select bit"]
    #[inline(always)]
    pub fn add_sect(&mut self) -> AddSectW<Crccr2Spec> {
        AddSectW::new(self, 9)
    }
    #[doc = "Bit 10 - Bank 2 CRC sector list clear bit"]
    #[inline(always)]
    pub fn clean_sect(&mut self) -> CleanSectW<Crccr2Spec> {
        CleanSectW::new(self, 10)
    }
    #[doc = "Bit 16 - Bank 2 CRC start bit"]
    #[inline(always)]
    pub fn start_crc(&mut self) -> StartCrcW<Crccr2Spec> {
        StartCrcW::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 2 CRC clear bit"]
    #[inline(always)]
    pub fn clean_crc(&mut self) -> CleanCrcW<Crccr2Spec> {
        CleanCrcW::new(self, 17)
    }
    #[doc = "Bits 20:21 - Bank 2 CRC burst size"]
    #[inline(always)]
    pub fn crc_burst(&mut self) -> CrcBurstW<Crccr2Spec> {
        CrcBurstW::new(self, 20)
    }
}
#[doc = "FLASH CRC control register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Crccr2Spec;
impl crate::RegisterSpec for Crccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crccr2::R`](R) reader structure"]
impl crate::Readable for Crccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`crccr2::W`](W) writer structure"]
impl crate::Writable for Crccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRCCR2 to value 0"]
impl crate::Resettable for Crccr2Spec {}

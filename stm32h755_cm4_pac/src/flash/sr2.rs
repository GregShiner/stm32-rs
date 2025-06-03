#[doc = "Register `SR2` reader"]
pub type R = crate::R<Sr2Spec>;
#[doc = "Register `SR2` writer"]
pub type W = crate::W<Sr2Spec>;
#[doc = "Field `BSY2` reader - Bank 2 ongoing program flag"]
pub type Bsy2R = crate::BitReader;
#[doc = "Field `BSY2` writer - Bank 2 ongoing program flag"]
pub type Bsy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBNE2` reader - Bank 2 write buffer not empty flag"]
pub type Wbne2R = crate::BitReader;
#[doc = "Field `WBNE2` writer - Bank 2 write buffer not empty flag"]
pub type Wbne2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QW2` reader - Bank 2 wait queue flag"]
pub type Qw2R = crate::BitReader;
#[doc = "Field `QW2` writer - Bank 2 wait queue flag"]
pub type Qw2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_BUSY2` reader - Bank 2 CRC busy flag"]
pub type CrcBusy2R = crate::BitReader;
#[doc = "Field `CRC_BUSY2` writer - Bank 2 CRC busy flag"]
pub type CrcBusy2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP2` reader - Bank 2 end-of-program flag"]
pub type Eop2R = crate::BitReader;
#[doc = "Field `EOP2` writer - Bank 2 end-of-program flag"]
pub type Eop2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR2` reader - Bank 2 write protection error flag"]
pub type Wrperr2R = crate::BitReader;
#[doc = "Field `WRPERR2` writer - Bank 2 write protection error flag"]
pub type Wrperr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR2` reader - Bank 2 programming sequence error flag"]
pub type Pgserr2R = crate::BitReader;
#[doc = "Field `PGSERR2` writer - Bank 2 programming sequence error flag"]
pub type Pgserr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRBERR2` reader - Bank 2 strobe error flag"]
pub type Strberr2R = crate::BitReader;
#[doc = "Field `STRBERR2` writer - Bank 2 strobe error flag"]
pub type Strberr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCERR2` reader - Bank 2 inconsistency error flag"]
pub type Incerr2R = crate::BitReader;
#[doc = "Field `INCERR2` writer - Bank 2 inconsistency error flag"]
pub type Incerr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR2` reader - Bank 2 write/erase error flag"]
pub type Operr2R = crate::BitReader;
#[doc = "Field `OPERR2` writer - Bank 2 write/erase error flag"]
pub type Operr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDPERR2` reader - Bank 2 read protection error flag"]
pub type Rdperr2R = crate::BitReader;
#[doc = "Field `RDPERR2` writer - Bank 2 read protection error flag"]
pub type Rdperr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSERR2` reader - Bank 2 secure error flag"]
pub type Rdserr2R = crate::BitReader;
#[doc = "Field `RDSERR2` writer - Bank 2 secure error flag"]
pub type Rdserr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNECCERR2` reader - Bank 2 single correction error flag"]
pub type Sneccerr2R = crate::BitReader;
#[doc = "Field `SNECCERR2` writer - Bank 2 single correction error flag"]
pub type Sneccerr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBECCERR2` reader - Bank 2 ECC double detection error flag"]
pub type Dbeccerr2R = crate::BitReader;
#[doc = "Field `DBECCERR2` writer - Bank 2 ECC double detection error flag"]
pub type Dbeccerr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEND2` reader - Bank 2 CRC-complete flag"]
pub type Crcend2R = crate::BitReader;
#[doc = "Field `CRCEND2` writer - Bank 2 CRC-complete flag"]
pub type Crcend2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bank 2 ongoing program flag"]
    #[inline(always)]
    pub fn bsy2(&self) -> Bsy2R {
        Bsy2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 2 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne2(&self) -> Wbne2R {
        Wbne2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 2 wait queue flag"]
    #[inline(always)]
    pub fn qw2(&self) -> Qw2R {
        Qw2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 2 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy2(&self) -> CrcBusy2R {
        CrcBusy2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 2 end-of-program flag"]
    #[inline(always)]
    pub fn eop2(&self) -> Eop2R {
        Eop2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 2 write protection error flag"]
    #[inline(always)]
    pub fn wrperr2(&self) -> Wrperr2R {
        Wrperr2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 2 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr2(&self) -> Pgserr2R {
        Pgserr2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 2 strobe error flag"]
    #[inline(always)]
    pub fn strberr2(&self) -> Strberr2R {
        Strberr2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 2 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr2(&self) -> Incerr2R {
        Incerr2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 2 write/erase error flag"]
    #[inline(always)]
    pub fn operr2(&self) -> Operr2R {
        Operr2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 2 read protection error flag"]
    #[inline(always)]
    pub fn rdperr2(&self) -> Rdperr2R {
        Rdperr2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 2 secure error flag"]
    #[inline(always)]
    pub fn rdserr2(&self) -> Rdserr2R {
        Rdserr2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 2 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr2(&self) -> Sneccerr2R {
        Sneccerr2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 2 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr2(&self) -> Dbeccerr2R {
        Dbeccerr2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 2 CRC-complete flag"]
    #[inline(always)]
    pub fn crcend2(&self) -> Crcend2R {
        Crcend2R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 2 ongoing program flag"]
    #[inline(always)]
    pub fn bsy2(&mut self) -> Bsy2W<Sr2Spec> {
        Bsy2W::new(self, 0)
    }
    #[doc = "Bit 1 - Bank 2 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne2(&mut self) -> Wbne2W<Sr2Spec> {
        Wbne2W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 2 wait queue flag"]
    #[inline(always)]
    pub fn qw2(&mut self) -> Qw2W<Sr2Spec> {
        Qw2W::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 2 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy2(&mut self) -> CrcBusy2W<Sr2Spec> {
        CrcBusy2W::new(self, 3)
    }
    #[doc = "Bit 16 - Bank 2 end-of-program flag"]
    #[inline(always)]
    pub fn eop2(&mut self) -> Eop2W<Sr2Spec> {
        Eop2W::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 2 write protection error flag"]
    #[inline(always)]
    pub fn wrperr2(&mut self) -> Wrperr2W<Sr2Spec> {
        Wrperr2W::new(self, 17)
    }
    #[doc = "Bit 18 - Bank 2 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr2(&mut self) -> Pgserr2W<Sr2Spec> {
        Pgserr2W::new(self, 18)
    }
    #[doc = "Bit 19 - Bank 2 strobe error flag"]
    #[inline(always)]
    pub fn strberr2(&mut self) -> Strberr2W<Sr2Spec> {
        Strberr2W::new(self, 19)
    }
    #[doc = "Bit 21 - Bank 2 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr2(&mut self) -> Incerr2W<Sr2Spec> {
        Incerr2W::new(self, 21)
    }
    #[doc = "Bit 22 - Bank 2 write/erase error flag"]
    #[inline(always)]
    pub fn operr2(&mut self) -> Operr2W<Sr2Spec> {
        Operr2W::new(self, 22)
    }
    #[doc = "Bit 23 - Bank 2 read protection error flag"]
    #[inline(always)]
    pub fn rdperr2(&mut self) -> Rdperr2W<Sr2Spec> {
        Rdperr2W::new(self, 23)
    }
    #[doc = "Bit 24 - Bank 2 secure error flag"]
    #[inline(always)]
    pub fn rdserr2(&mut self) -> Rdserr2W<Sr2Spec> {
        Rdserr2W::new(self, 24)
    }
    #[doc = "Bit 25 - Bank 2 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr2(&mut self) -> Sneccerr2W<Sr2Spec> {
        Sneccerr2W::new(self, 25)
    }
    #[doc = "Bit 26 - Bank 2 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr2(&mut self) -> Dbeccerr2W<Sr2Spec> {
        Dbeccerr2W::new(self, 26)
    }
    #[doc = "Bit 27 - Bank 2 CRC-complete flag"]
    #[inline(always)]
    pub fn crcend2(&mut self) -> Crcend2W<Sr2Spec> {
        Crcend2W::new(self, 27)
    }
}
#[doc = "FLASH status register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr2Spec;
impl crate::RegisterSpec for Sr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr2::R`](R) reader structure"]
impl crate::Readable for Sr2Spec {}
#[doc = "`write(|w| ..)` method takes [`sr2::W`](W) writer structure"]
impl crate::Writable for Sr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR2 to value 0"]
impl crate::Resettable for Sr2Spec {}

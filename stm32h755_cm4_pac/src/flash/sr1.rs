#[doc = "Register `SR1` reader"]
pub type R = crate::R<Sr1Spec>;
#[doc = "Register `SR1` writer"]
pub type W = crate::W<Sr1Spec>;
#[doc = "Field `BSY1` reader - Bank 1 ongoing program flag"]
pub type Bsy1R = crate::BitReader;
#[doc = "Field `BSY1` writer - Bank 1 ongoing program flag"]
pub type Bsy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WBNE1` reader - Bank 1 write buffer not empty flag"]
pub type Wbne1R = crate::BitReader;
#[doc = "Field `WBNE1` writer - Bank 1 write buffer not empty flag"]
pub type Wbne1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QW1` reader - Bank 1 wait queue flag"]
pub type Qw1R = crate::BitReader;
#[doc = "Field `QW1` writer - Bank 1 wait queue flag"]
pub type Qw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRC_BUSY1` reader - Bank 1 CRC busy flag"]
pub type CrcBusy1R = crate::BitReader;
#[doc = "Field `CRC_BUSY1` writer - Bank 1 CRC busy flag"]
pub type CrcBusy1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOP1` reader - Bank 1 end-of-program flag"]
pub type Eop1R = crate::BitReader;
#[doc = "Field `EOP1` writer - Bank 1 end-of-program flag"]
pub type Eop1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERR1` reader - Bank 1 write protection error flag"]
pub type Wrperr1R = crate::BitReader;
#[doc = "Field `WRPERR1` writer - Bank 1 write protection error flag"]
pub type Wrperr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERR1` reader - Bank 1 programming sequence error flag"]
pub type Pgserr1R = crate::BitReader;
#[doc = "Field `PGSERR1` writer - Bank 1 programming sequence error flag"]
pub type Pgserr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRBERR1` reader - Bank 1 strobe error flag"]
pub type Strberr1R = crate::BitReader;
#[doc = "Field `STRBERR1` writer - Bank 1 strobe error flag"]
pub type Strberr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCERR1` reader - Bank 1 inconsistency error flag"]
pub type Incerr1R = crate::BitReader;
#[doc = "Field `INCERR1` writer - Bank 1 inconsistency error flag"]
pub type Incerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERR1` reader - Bank 1 write/erase error flag"]
pub type Operr1R = crate::BitReader;
#[doc = "Field `OPERR1` writer - Bank 1 write/erase error flag"]
pub type Operr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDPERR1` reader - Bank 1 read protection error flag"]
pub type Rdperr1R = crate::BitReader;
#[doc = "Field `RDPERR1` writer - Bank 1 read protection error flag"]
pub type Rdperr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSERR1` reader - Bank 1 secure error flag"]
pub type Rdserr1R = crate::BitReader;
#[doc = "Field `RDSERR1` writer - Bank 1 secure error flag"]
pub type Rdserr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNECCERR11` reader - Bank 1 single correction error flag"]
pub type Sneccerr11R = crate::BitReader;
#[doc = "Field `SNECCERR11` writer - Bank 1 single correction error flag"]
pub type Sneccerr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBECCERR1` reader - Bank 1 ECC double detection error flag"]
pub type Dbeccerr1R = crate::BitReader;
#[doc = "Field `DBECCERR1` writer - Bank 1 ECC double detection error flag"]
pub type Dbeccerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEND1` reader - Bank 1 CRC-complete flag"]
pub type Crcend1R = crate::BitReader;
#[doc = "Field `CRCEND1` writer - Bank 1 CRC-complete flag"]
pub type Crcend1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bank 1 ongoing program flag"]
    #[inline(always)]
    pub fn bsy1(&self) -> Bsy1R {
        Bsy1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne1(&self) -> Wbne1R {
        Wbne1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    pub fn qw1(&self) -> Qw1R {
        Qw1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy1(&self) -> CrcBusy1R {
        CrcBusy1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    pub fn eop1(&self) -> Eop1R {
        Eop1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    pub fn wrperr1(&self) -> Wrperr1R {
        Wrperr1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr1(&self) -> Pgserr1R {
        Pgserr1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    pub fn strberr1(&self) -> Strberr1R {
        Strberr1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr1(&self) -> Incerr1R {
        Incerr1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error flag"]
    #[inline(always)]
    pub fn operr1(&self) -> Operr1R {
        Operr1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    pub fn rdperr1(&self) -> Rdperr1R {
        Rdperr1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    pub fn rdserr1(&self) -> Rdserr1R {
        Rdserr1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr11(&self) -> Sneccerr11R {
        Sneccerr11R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr1(&self) -> Dbeccerr1R {
        Dbeccerr1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRC-complete flag"]
    #[inline(always)]
    pub fn crcend1(&self) -> Crcend1R {
        Crcend1R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 ongoing program flag"]
    #[inline(always)]
    pub fn bsy1(&mut self) -> Bsy1W<Sr1Spec> {
        Bsy1W::new(self, 0)
    }
    #[doc = "Bit 1 - Bank 1 write buffer not empty flag"]
    #[inline(always)]
    pub fn wbne1(&mut self) -> Wbne1W<Sr1Spec> {
        Wbne1W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 1 wait queue flag"]
    #[inline(always)]
    pub fn qw1(&mut self) -> Qw1W<Sr1Spec> {
        Qw1W::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 1 CRC busy flag"]
    #[inline(always)]
    pub fn crc_busy1(&mut self) -> CrcBusy1W<Sr1Spec> {
        CrcBusy1W::new(self, 3)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program flag"]
    #[inline(always)]
    pub fn eop1(&mut self) -> Eop1W<Sr1Spec> {
        Eop1W::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 1 write protection error flag"]
    #[inline(always)]
    pub fn wrperr1(&mut self) -> Wrperr1W<Sr1Spec> {
        Wrperr1W::new(self, 17)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error flag"]
    #[inline(always)]
    pub fn pgserr1(&mut self) -> Pgserr1W<Sr1Spec> {
        Pgserr1W::new(self, 18)
    }
    #[doc = "Bit 19 - Bank 1 strobe error flag"]
    #[inline(always)]
    pub fn strberr1(&mut self) -> Strberr1W<Sr1Spec> {
        Strberr1W::new(self, 19)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error flag"]
    #[inline(always)]
    pub fn incerr1(&mut self) -> Incerr1W<Sr1Spec> {
        Incerr1W::new(self, 21)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error flag"]
    #[inline(always)]
    pub fn operr1(&mut self) -> Operr1W<Sr1Spec> {
        Operr1W::new(self, 22)
    }
    #[doc = "Bit 23 - Bank 1 read protection error flag"]
    #[inline(always)]
    pub fn rdperr1(&mut self) -> Rdperr1W<Sr1Spec> {
        Rdperr1W::new(self, 23)
    }
    #[doc = "Bit 24 - Bank 1 secure error flag"]
    #[inline(always)]
    pub fn rdserr1(&mut self) -> Rdserr1W<Sr1Spec> {
        Rdserr1W::new(self, 24)
    }
    #[doc = "Bit 25 - Bank 1 single correction error flag"]
    #[inline(always)]
    pub fn sneccerr11(&mut self) -> Sneccerr11W<Sr1Spec> {
        Sneccerr11W::new(self, 25)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error flag"]
    #[inline(always)]
    pub fn dbeccerr1(&mut self) -> Dbeccerr1W<Sr1Spec> {
        Dbeccerr1W::new(self, 26)
    }
    #[doc = "Bit 27 - Bank 1 CRC-complete flag"]
    #[inline(always)]
    pub fn crcend1(&mut self) -> Crcend1W<Sr1Spec> {
        Crcend1W::new(self, 27)
    }
}
#[doc = "FLASH status register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sr1Spec;
impl crate::RegisterSpec for Sr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr1::R`](R) reader structure"]
impl crate::Readable for Sr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sr1::W`](W) writer structure"]
impl crate::Writable for Sr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SR1 to value 0"]
impl crate::Resettable for Sr1Spec {}

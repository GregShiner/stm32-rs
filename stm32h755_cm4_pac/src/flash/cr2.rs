#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `LOCK2` reader - Bank 2 configuration lock bit"]
pub type Lock2R = crate::BitReader;
#[doc = "Field `LOCK2` writer - Bank 2 configuration lock bit"]
pub type Lock2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG2` reader - Bank 2 program enable bit"]
pub type Pg2R = crate::BitReader;
#[doc = "Field `PG2` writer - Bank 2 program enable bit"]
pub type Pg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SER2` reader - Bank 2 sector erase request"]
pub type Ser2R = crate::BitReader;
#[doc = "Field `SER2` writer - Bank 2 sector erase request"]
pub type Ser2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BER2` reader - Bank 2 erase request"]
pub type Ber2R = crate::BitReader;
#[doc = "Field `BER2` writer - Bank 2 erase request"]
pub type Ber2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE2` reader - Bank 2 program size"]
pub type Psize2R = crate::FieldReader;
#[doc = "Field `PSIZE2` writer - Bank 2 program size"]
pub type Psize2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FW2` reader - Bank 2 write forcing control bit"]
pub type Fw2R = crate::BitReader;
#[doc = "Field `FW2` writer - Bank 2 write forcing control bit"]
pub type Fw2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START2` reader - Bank 2 bank or sector erase start control bit"]
pub type Start2R = crate::BitReader;
#[doc = "Field `START2` writer - Bank 2 bank or sector erase start control bit"]
pub type Start2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNB2` reader - Bank 2 sector erase selection number"]
pub type Snb2R = crate::FieldReader;
#[doc = "Field `SNB2` writer - Bank 2 sector erase selection number"]
pub type Snb2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CRC_EN` reader - Bank 2 CRC control bit"]
pub type CrcEnR = crate::BitReader;
#[doc = "Field `CRC_EN` writer - Bank 2 CRC control bit"]
pub type CrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE2` reader - Bank 2 end-of-program interrupt control bit"]
pub type Eopie2R = crate::BitReader;
#[doc = "Field `EOPIE2` writer - Bank 2 end-of-program interrupt control bit"]
pub type Eopie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERRIE2` reader - Bank 2 write protection error interrupt enable bit"]
pub type Wrperrie2R = crate::BitReader;
#[doc = "Field `WRPERRIE2` writer - Bank 2 write protection error interrupt enable bit"]
pub type Wrperrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERRIE2` reader - Bank 2 programming sequence error interrupt enable bit"]
pub type Pgserrie2R = crate::BitReader;
#[doc = "Field `PGSERRIE2` writer - Bank 2 programming sequence error interrupt enable bit"]
pub type Pgserrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRBERRIE2` reader - Bank 2 strobe error interrupt enable bit"]
pub type Strberrie2R = crate::BitReader;
#[doc = "Field `STRBERRIE2` writer - Bank 2 strobe error interrupt enable bit"]
pub type Strberrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCERRIE2` reader - Bank 2 inconsistency error interrupt enable bit"]
pub type Incerrie2R = crate::BitReader;
#[doc = "Field `INCERRIE2` writer - Bank 2 inconsistency error interrupt enable bit"]
pub type Incerrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERRIE2` reader - Bank 2 write/erase error interrupt enable bit"]
pub type Operrie2R = crate::BitReader;
#[doc = "Field `OPERRIE2` writer - Bank 2 write/erase error interrupt enable bit"]
pub type Operrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDPERRIE2` reader - Bank 2 read protection error interrupt enable bit"]
pub type Rdperrie2R = crate::BitReader;
#[doc = "Field `RDPERRIE2` writer - Bank 2 read protection error interrupt enable bit"]
pub type Rdperrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSERRIE2` reader - Bank 2 secure error interrupt enable bit"]
pub type Rdserrie2R = crate::BitReader;
#[doc = "Field `RDSERRIE2` writer - Bank 2 secure error interrupt enable bit"]
pub type Rdserrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNECCERRIE2` reader - Bank 2 ECC single correction error interrupt enable bit"]
pub type Sneccerrie2R = crate::BitReader;
#[doc = "Field `SNECCERRIE2` writer - Bank 2 ECC single correction error interrupt enable bit"]
pub type Sneccerrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBECCERRIE2` reader - Bank 2 ECC double detection error interrupt enable bit"]
pub type Dbeccerrie2R = crate::BitReader;
#[doc = "Field `DBECCERRIE2` writer - Bank 2 ECC double detection error interrupt enable bit"]
pub type Dbeccerrie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCENDIE2` reader - Bank 2 end of CRC calculation interrupt enable bit"]
pub type Crcendie2R = crate::BitReader;
#[doc = "Field `CRCENDIE2` writer - Bank 2 end of CRC calculation interrupt enable bit"]
pub type Crcendie2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bank 2 configuration lock bit"]
    #[inline(always)]
    pub fn lock2(&self) -> Lock2R {
        Lock2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 2 program enable bit"]
    #[inline(always)]
    pub fn pg2(&self) -> Pg2R {
        Pg2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 2 sector erase request"]
    #[inline(always)]
    pub fn ser2(&self) -> Ser2R {
        Ser2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 2 erase request"]
    #[inline(always)]
    pub fn ber2(&self) -> Ber2R {
        Ber2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Bank 2 program size"]
    #[inline(always)]
    pub fn psize2(&self) -> Psize2R {
        Psize2R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bank 2 write forcing control bit"]
    #[inline(always)]
    pub fn fw2(&self) -> Fw2R {
        Fw2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank 2 bank or sector erase start control bit"]
    #[inline(always)]
    pub fn start2(&self) -> Start2R {
        Start2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bank 2 sector erase selection number"]
    #[inline(always)]
    pub fn snb2(&self) -> Snb2R {
        Snb2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Bank 2 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&self) -> CrcEnR {
        CrcEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 2 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie2(&self) -> Eopie2R {
        Eopie2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 2 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie2(&self) -> Wrperrie2R {
        Wrperrie2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 2 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie2(&self) -> Pgserrie2R {
        Pgserrie2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 2 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie2(&self) -> Strberrie2R {
        Strberrie2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 2 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie2(&self) -> Incerrie2R {
        Incerrie2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 2 write/erase error interrupt enable bit"]
    #[inline(always)]
    pub fn operrie2(&self) -> Operrie2R {
        Operrie2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 2 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie2(&self) -> Rdperrie2R {
        Rdperrie2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 2 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie2(&self) -> Rdserrie2R {
        Rdserrie2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 2 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie2(&self) -> Sneccerrie2R {
        Sneccerrie2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 2 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie2(&self) -> Dbeccerrie2R {
        Dbeccerrie2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 2 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie2(&self) -> Crcendie2R {
        Crcendie2R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 2 configuration lock bit"]
    #[inline(always)]
    pub fn lock2(&mut self) -> Lock2W<Cr2Spec> {
        Lock2W::new(self, 0)
    }
    #[doc = "Bit 1 - Bank 2 program enable bit"]
    #[inline(always)]
    pub fn pg2(&mut self) -> Pg2W<Cr2Spec> {
        Pg2W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 2 sector erase request"]
    #[inline(always)]
    pub fn ser2(&mut self) -> Ser2W<Cr2Spec> {
        Ser2W::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 2 erase request"]
    #[inline(always)]
    pub fn ber2(&mut self) -> Ber2W<Cr2Spec> {
        Ber2W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Bank 2 program size"]
    #[inline(always)]
    pub fn psize2(&mut self) -> Psize2W<Cr2Spec> {
        Psize2W::new(self, 4)
    }
    #[doc = "Bit 6 - Bank 2 write forcing control bit"]
    #[inline(always)]
    pub fn fw2(&mut self) -> Fw2W<Cr2Spec> {
        Fw2W::new(self, 6)
    }
    #[doc = "Bit 7 - Bank 2 bank or sector erase start control bit"]
    #[inline(always)]
    pub fn start2(&mut self) -> Start2W<Cr2Spec> {
        Start2W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Bank 2 sector erase selection number"]
    #[inline(always)]
    pub fn snb2(&mut self) -> Snb2W<Cr2Spec> {
        Snb2W::new(self, 8)
    }
    #[doc = "Bit 15 - Bank 2 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&mut self) -> CrcEnW<Cr2Spec> {
        CrcEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Bank 2 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie2(&mut self) -> Eopie2W<Cr2Spec> {
        Eopie2W::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 2 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie2(&mut self) -> Wrperrie2W<Cr2Spec> {
        Wrperrie2W::new(self, 17)
    }
    #[doc = "Bit 18 - Bank 2 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie2(&mut self) -> Pgserrie2W<Cr2Spec> {
        Pgserrie2W::new(self, 18)
    }
    #[doc = "Bit 19 - Bank 2 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie2(&mut self) -> Strberrie2W<Cr2Spec> {
        Strberrie2W::new(self, 19)
    }
    #[doc = "Bit 21 - Bank 2 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie2(&mut self) -> Incerrie2W<Cr2Spec> {
        Incerrie2W::new(self, 21)
    }
    #[doc = "Bit 22 - Bank 2 write/erase error interrupt enable bit"]
    #[inline(always)]
    pub fn operrie2(&mut self) -> Operrie2W<Cr2Spec> {
        Operrie2W::new(self, 22)
    }
    #[doc = "Bit 23 - Bank 2 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie2(&mut self) -> Rdperrie2W<Cr2Spec> {
        Rdperrie2W::new(self, 23)
    }
    #[doc = "Bit 24 - Bank 2 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie2(&mut self) -> Rdserrie2W<Cr2Spec> {
        Rdserrie2W::new(self, 24)
    }
    #[doc = "Bit 25 - Bank 2 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie2(&mut self) -> Sneccerrie2W<Cr2Spec> {
        Sneccerrie2W::new(self, 25)
    }
    #[doc = "Bit 26 - Bank 2 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie2(&mut self) -> Dbeccerrie2W<Cr2Spec> {
        Dbeccerrie2W::new(self, 26)
    }
    #[doc = "Bit 27 - Bank 2 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie2(&mut self) -> Crcendie2W<Cr2Spec> {
        Crcendie2W::new(self, 27)
    }
}
#[doc = "FLASH control register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}

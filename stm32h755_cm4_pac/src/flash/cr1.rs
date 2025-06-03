#[doc = "Register `CR1` reader"]
pub type R = crate::R<Cr1Spec>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<Cr1Spec>;
#[doc = "Field `LOCK1` reader - Bank 1 configuration lock bit"]
pub type Lock1R = crate::BitReader;
#[doc = "Field `LOCK1` writer - Bank 1 configuration lock bit"]
pub type Lock1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PG1` reader - Bank 1 program enable bit"]
pub type Pg1R = crate::BitReader;
#[doc = "Field `PG1` writer - Bank 1 program enable bit"]
pub type Pg1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SER1` reader - Bank 1 sector erase request"]
pub type Ser1R = crate::BitReader;
#[doc = "Field `SER1` writer - Bank 1 sector erase request"]
pub type Ser1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BER1` reader - Bank 1 erase request"]
pub type Ber1R = crate::BitReader;
#[doc = "Field `BER1` writer - Bank 1 erase request"]
pub type Ber1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE1` reader - Bank 1 program size"]
pub type Psize1R = crate::FieldReader;
#[doc = "Field `PSIZE1` writer - Bank 1 program size"]
pub type Psize1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FW1` reader - Bank 1 write forcing control bit"]
pub type Fw1R = crate::BitReader;
#[doc = "Field `FW1` writer - Bank 1 write forcing control bit"]
pub type Fw1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `START1` reader - Bank 1 bank or sector erase start control bit"]
pub type Start1R = crate::BitReader;
#[doc = "Field `START1` writer - Bank 1 bank or sector erase start control bit"]
pub type Start1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNB1` reader - Bank 1 sector erase selection number"]
pub type Snb1R = crate::FieldReader;
#[doc = "Field `SNB1` writer - Bank 1 sector erase selection number"]
pub type Snb1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CRC_EN` reader - Bank 1 CRC control bit"]
pub type CrcEnR = crate::BitReader;
#[doc = "Field `CRC_EN` writer - Bank 1 CRC control bit"]
pub type CrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOPIE1` reader - Bank 1 end-of-program interrupt control bit"]
pub type Eopie1R = crate::BitReader;
#[doc = "Field `EOPIE1` writer - Bank 1 end-of-program interrupt control bit"]
pub type Eopie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRPERRIE1` reader - Bank 1 write protection error interrupt enable bit"]
pub type Wrperrie1R = crate::BitReader;
#[doc = "Field `WRPERRIE1` writer - Bank 1 write protection error interrupt enable bit"]
pub type Wrperrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGSERRIE1` reader - Bank 1 programming sequence error interrupt enable bit"]
pub type Pgserrie1R = crate::BitReader;
#[doc = "Field `PGSERRIE1` writer - Bank 1 programming sequence error interrupt enable bit"]
pub type Pgserrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STRBERRIE1` reader - Bank 1 strobe error interrupt enable bit"]
pub type Strberrie1R = crate::BitReader;
#[doc = "Field `STRBERRIE1` writer - Bank 1 strobe error interrupt enable bit"]
pub type Strberrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCERRIE1` reader - Bank 1 inconsistency error interrupt enable bit"]
pub type Incerrie1R = crate::BitReader;
#[doc = "Field `INCERRIE1` writer - Bank 1 inconsistency error interrupt enable bit"]
pub type Incerrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPERRIE1` reader - Bank 1 write/erase error interrupt enable bit"]
pub type Operrie1R = crate::BitReader;
#[doc = "Field `OPERRIE1` writer - Bank 1 write/erase error interrupt enable bit"]
pub type Operrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDPERRIE1` reader - Bank 1 read protection error interrupt enable bit"]
pub type Rdperrie1R = crate::BitReader;
#[doc = "Field `RDPERRIE1` writer - Bank 1 read protection error interrupt enable bit"]
pub type Rdperrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSERRIE1` reader - Bank 1 secure error interrupt enable bit"]
pub type Rdserrie1R = crate::BitReader;
#[doc = "Field `RDSERRIE1` writer - Bank 1 secure error interrupt enable bit"]
pub type Rdserrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNECCERRIE1` reader - Bank 1 ECC single correction error interrupt enable bit"]
pub type Sneccerrie1R = crate::BitReader;
#[doc = "Field `SNECCERRIE1` writer - Bank 1 ECC single correction error interrupt enable bit"]
pub type Sneccerrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBECCERRIE1` reader - Bank 1 ECC double detection error interrupt enable bit"]
pub type Dbeccerrie1R = crate::BitReader;
#[doc = "Field `DBECCERRIE1` writer - Bank 1 ECC double detection error interrupt enable bit"]
pub type Dbeccerrie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCENDIE1` reader - Bank 1 end of CRC calculation interrupt enable bit"]
pub type Crcendie1R = crate::BitReader;
#[doc = "Field `CRCENDIE1` writer - Bank 1 end of CRC calculation interrupt enable bit"]
pub type Crcendie1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    pub fn lock1(&self) -> Lock1R {
        Lock1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bank 1 program enable bit"]
    #[inline(always)]
    pub fn pg1(&self) -> Pg1R {
        Pg1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    pub fn ser1(&self) -> Ser1R {
        Ser1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    pub fn ber1(&self) -> Ber1R {
        Ber1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    pub fn psize1(&self) -> Psize1R {
        Psize1R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    pub fn fw1(&self) -> Fw1R {
        Fw1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bank 1 bank or sector erase start control bit"]
    #[inline(always)]
    pub fn start1(&self) -> Start1R {
        Start1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    pub fn snb1(&self) -> Snb1R {
        Snb1R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&self) -> CrcEnR {
        CrcEnR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie1(&self) -> Eopie1R {
        Eopie1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie1(&self) -> Wrperrie1R {
        Wrperrie1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie1(&self) -> Pgserrie1R {
        Pgserrie1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie1(&self) -> Strberrie1R {
        Strberrie1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie1(&self) -> Incerrie1R {
        Incerrie1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error interrupt enable bit"]
    #[inline(always)]
    pub fn operrie1(&self) -> Operrie1R {
        Operrie1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie1(&self) -> Rdperrie1R {
        Rdperrie1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie1(&self) -> Rdserrie1R {
        Rdserrie1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie1(&self) -> Sneccerrie1R {
        Sneccerrie1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie1(&self) -> Dbeccerrie1R {
        Dbeccerrie1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie1(&self) -> Crcendie1R {
        Crcendie1R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bank 1 configuration lock bit"]
    #[inline(always)]
    pub fn lock1(&mut self) -> Lock1W<Cr1Spec> {
        Lock1W::new(self, 0)
    }
    #[doc = "Bit 1 - Bank 1 program enable bit"]
    #[inline(always)]
    pub fn pg1(&mut self) -> Pg1W<Cr1Spec> {
        Pg1W::new(self, 1)
    }
    #[doc = "Bit 2 - Bank 1 sector erase request"]
    #[inline(always)]
    pub fn ser1(&mut self) -> Ser1W<Cr1Spec> {
        Ser1W::new(self, 2)
    }
    #[doc = "Bit 3 - Bank 1 erase request"]
    #[inline(always)]
    pub fn ber1(&mut self) -> Ber1W<Cr1Spec> {
        Ber1W::new(self, 3)
    }
    #[doc = "Bits 4:5 - Bank 1 program size"]
    #[inline(always)]
    pub fn psize1(&mut self) -> Psize1W<Cr1Spec> {
        Psize1W::new(self, 4)
    }
    #[doc = "Bit 6 - Bank 1 write forcing control bit"]
    #[inline(always)]
    pub fn fw1(&mut self) -> Fw1W<Cr1Spec> {
        Fw1W::new(self, 6)
    }
    #[doc = "Bit 7 - Bank 1 bank or sector erase start control bit"]
    #[inline(always)]
    pub fn start1(&mut self) -> Start1W<Cr1Spec> {
        Start1W::new(self, 7)
    }
    #[doc = "Bits 8:10 - Bank 1 sector erase selection number"]
    #[inline(always)]
    pub fn snb1(&mut self) -> Snb1W<Cr1Spec> {
        Snb1W::new(self, 8)
    }
    #[doc = "Bit 15 - Bank 1 CRC control bit"]
    #[inline(always)]
    pub fn crc_en(&mut self) -> CrcEnW<Cr1Spec> {
        CrcEnW::new(self, 15)
    }
    #[doc = "Bit 16 - Bank 1 end-of-program interrupt control bit"]
    #[inline(always)]
    pub fn eopie1(&mut self) -> Eopie1W<Cr1Spec> {
        Eopie1W::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 1 write protection error interrupt enable bit"]
    #[inline(always)]
    pub fn wrperrie1(&mut self) -> Wrperrie1W<Cr1Spec> {
        Wrperrie1W::new(self, 17)
    }
    #[doc = "Bit 18 - Bank 1 programming sequence error interrupt enable bit"]
    #[inline(always)]
    pub fn pgserrie1(&mut self) -> Pgserrie1W<Cr1Spec> {
        Pgserrie1W::new(self, 18)
    }
    #[doc = "Bit 19 - Bank 1 strobe error interrupt enable bit"]
    #[inline(always)]
    pub fn strberrie1(&mut self) -> Strberrie1W<Cr1Spec> {
        Strberrie1W::new(self, 19)
    }
    #[doc = "Bit 21 - Bank 1 inconsistency error interrupt enable bit"]
    #[inline(always)]
    pub fn incerrie1(&mut self) -> Incerrie1W<Cr1Spec> {
        Incerrie1W::new(self, 21)
    }
    #[doc = "Bit 22 - Bank 1 write/erase error interrupt enable bit"]
    #[inline(always)]
    pub fn operrie1(&mut self) -> Operrie1W<Cr1Spec> {
        Operrie1W::new(self, 22)
    }
    #[doc = "Bit 23 - Bank 1 read protection error interrupt enable bit"]
    #[inline(always)]
    pub fn rdperrie1(&mut self) -> Rdperrie1W<Cr1Spec> {
        Rdperrie1W::new(self, 23)
    }
    #[doc = "Bit 24 - Bank 1 secure error interrupt enable bit"]
    #[inline(always)]
    pub fn rdserrie1(&mut self) -> Rdserrie1W<Cr1Spec> {
        Rdserrie1W::new(self, 24)
    }
    #[doc = "Bit 25 - Bank 1 ECC single correction error interrupt enable bit"]
    #[inline(always)]
    pub fn sneccerrie1(&mut self) -> Sneccerrie1W<Cr1Spec> {
        Sneccerrie1W::new(self, 25)
    }
    #[doc = "Bit 26 - Bank 1 ECC double detection error interrupt enable bit"]
    #[inline(always)]
    pub fn dbeccerrie1(&mut self) -> Dbeccerrie1W<Cr1Spec> {
        Dbeccerrie1W::new(self, 26)
    }
    #[doc = "Bit 27 - Bank 1 end of CRC calculation interrupt enable bit"]
    #[inline(always)]
    pub fn crcendie1(&mut self) -> Crcendie1W<Cr1Spec> {
        Crcendie1W::new(self, 27)
    }
}
#[doc = "FLASH control register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr1Spec;
impl crate::RegisterSpec for Cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for Cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for Cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for Cr1Spec {}

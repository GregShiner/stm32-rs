#[doc = "Register `CCR2` reader"]
pub type R = crate::R<Ccr2Spec>;
#[doc = "Register `CCR2` writer"]
pub type W = crate::W<Ccr2Spec>;
#[doc = "Field `CLR_EOP2` reader - Bank 1 EOP1 flag clear bit"]
pub type ClrEop2R = crate::BitReader;
#[doc = "Field `CLR_EOP2` writer - Bank 1 EOP1 flag clear bit"]
pub type ClrEop2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_WRPERR2` reader - Bank 2 WRPERR1 flag clear bit"]
pub type ClrWrperr2R = crate::BitReader;
#[doc = "Field `CLR_WRPERR2` writer - Bank 2 WRPERR1 flag clear bit"]
pub type ClrWrperr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_PGSERR2` reader - Bank 2 PGSERR1 flag clear bi"]
pub type ClrPgserr2R = crate::BitReader;
#[doc = "Field `CLR_PGSERR2` writer - Bank 2 PGSERR1 flag clear bi"]
pub type ClrPgserr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_STRBERR2` reader - Bank 2 STRBERR1 flag clear bit"]
pub type ClrStrberr2R = crate::BitReader;
#[doc = "Field `CLR_STRBERR2` writer - Bank 2 STRBERR1 flag clear bit"]
pub type ClrStrberr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_INCERR2` reader - Bank 2 INCERR1 flag clear bit"]
pub type ClrIncerr2R = crate::BitReader;
#[doc = "Field `CLR_INCERR2` writer - Bank 2 INCERR1 flag clear bit"]
pub type ClrIncerr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_OPERR2` reader - Bank 2 OPERR1 flag clear bit"]
pub type ClrOperr2R = crate::BitReader;
#[doc = "Field `CLR_OPERR2` writer - Bank 2 OPERR1 flag clear bit"]
pub type ClrOperr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RDPERR2` reader - Bank 2 RDPERR1 flag clear bit"]
pub type ClrRdperr2R = crate::BitReader;
#[doc = "Field `CLR_RDPERR2` writer - Bank 2 RDPERR1 flag clear bit"]
pub type ClrRdperr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RDSERR1` reader - Bank 1 RDSERR1 flag clear bit"]
pub type ClrRdserr1R = crate::BitReader;
#[doc = "Field `CLR_RDSERR1` writer - Bank 1 RDSERR1 flag clear bit"]
pub type ClrRdserr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_SNECCERR2` reader - Bank 2 SNECCERR1 flag clear bit"]
pub type ClrSneccerr2R = crate::BitReader;
#[doc = "Field `CLR_SNECCERR2` writer - Bank 2 SNECCERR1 flag clear bit"]
pub type ClrSneccerr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_DBECCERR1` reader - Bank 1 DBECCERR1 flag clear bit"]
pub type ClrDbeccerr1R = crate::BitReader;
#[doc = "Field `CLR_DBECCERR1` writer - Bank 1 DBECCERR1 flag clear bit"]
pub type ClrDbeccerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_CRCEND2` reader - Bank 2 CRCEND1 flag clear bit"]
pub type ClrCrcend2R = crate::BitReader;
#[doc = "Field `CLR_CRCEND2` writer - Bank 2 CRCEND1 flag clear bit"]
pub type ClrCrcend2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop2(&self) -> ClrEop2R {
        ClrEop2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 2 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr2(&self) -> ClrWrperr2R {
        ClrWrperr2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 2 PGSERR1 flag clear bi"]
    #[inline(always)]
    pub fn clr_pgserr2(&self) -> ClrPgserr2R {
        ClrPgserr2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 2 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr2(&self) -> ClrStrberr2R {
        ClrStrberr2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 2 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr2(&self) -> ClrIncerr2R {
        ClrIncerr2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 2 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr2(&self) -> ClrOperr2R {
        ClrOperr2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 2 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr2(&self) -> ClrRdperr2R {
        ClrRdperr2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr1(&self) -> ClrRdserr1R {
        ClrRdserr1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 2 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr2(&self) -> ClrSneccerr2R {
        ClrSneccerr2R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr1(&self) -> ClrDbeccerr1R {
        ClrDbeccerr1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 2 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend2(&self) -> ClrCrcend2R {
        ClrCrcend2R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop2(&mut self) -> ClrEop2W<Ccr2Spec> {
        ClrEop2W::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 2 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr2(&mut self) -> ClrWrperr2W<Ccr2Spec> {
        ClrWrperr2W::new(self, 17)
    }
    #[doc = "Bit 18 - Bank 2 PGSERR1 flag clear bi"]
    #[inline(always)]
    pub fn clr_pgserr2(&mut self) -> ClrPgserr2W<Ccr2Spec> {
        ClrPgserr2W::new(self, 18)
    }
    #[doc = "Bit 19 - Bank 2 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr2(&mut self) -> ClrStrberr2W<Ccr2Spec> {
        ClrStrberr2W::new(self, 19)
    }
    #[doc = "Bit 21 - Bank 2 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr2(&mut self) -> ClrIncerr2W<Ccr2Spec> {
        ClrIncerr2W::new(self, 21)
    }
    #[doc = "Bit 22 - Bank 2 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr2(&mut self) -> ClrOperr2W<Ccr2Spec> {
        ClrOperr2W::new(self, 22)
    }
    #[doc = "Bit 23 - Bank 2 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr2(&mut self) -> ClrRdperr2W<Ccr2Spec> {
        ClrRdperr2W::new(self, 23)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr1(&mut self) -> ClrRdserr1W<Ccr2Spec> {
        ClrRdserr1W::new(self, 24)
    }
    #[doc = "Bit 25 - Bank 2 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr2(&mut self) -> ClrSneccerr2W<Ccr2Spec> {
        ClrSneccerr2W::new(self, 25)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr1(&mut self) -> ClrDbeccerr1W<Ccr2Spec> {
        ClrDbeccerr1W::new(self, 26)
    }
    #[doc = "Bit 27 - Bank 2 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend2(&mut self) -> ClrCrcend2W<Ccr2Spec> {
        ClrCrcend2W::new(self, 27)
    }
}
#[doc = "FLASH clear control register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr2Spec;
impl crate::RegisterSpec for Ccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr2::R`](R) reader structure"]
impl crate::Readable for Ccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr2::W`](W) writer structure"]
impl crate::Writable for Ccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR2 to value 0"]
impl crate::Resettable for Ccr2Spec {}

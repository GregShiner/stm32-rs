#[doc = "Register `CCR1` reader"]
pub type R = crate::R<Ccr1Spec>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<Ccr1Spec>;
#[doc = "Field `CLR_EOP1` reader - Bank 1 EOP1 flag clear bit"]
pub type ClrEop1R = crate::BitReader;
#[doc = "Field `CLR_EOP1` writer - Bank 1 EOP1 flag clear bit"]
pub type ClrEop1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_WRPERR1` reader - Bank 1 WRPERR1 flag clear bit"]
pub type ClrWrperr1R = crate::BitReader;
#[doc = "Field `CLR_WRPERR1` writer - Bank 1 WRPERR1 flag clear bit"]
pub type ClrWrperr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_PGSERR1` reader - Bank 1 PGSERR1 flag clear bi"]
pub type ClrPgserr1R = crate::BitReader;
#[doc = "Field `CLR_PGSERR1` writer - Bank 1 PGSERR1 flag clear bi"]
pub type ClrPgserr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_STRBERR1` reader - Bank 1 STRBERR1 flag clear bit"]
pub type ClrStrberr1R = crate::BitReader;
#[doc = "Field `CLR_STRBERR1` writer - Bank 1 STRBERR1 flag clear bit"]
pub type ClrStrberr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_INCERR1` reader - Bank 1 INCERR1 flag clear bit"]
pub type ClrIncerr1R = crate::BitReader;
#[doc = "Field `CLR_INCERR1` writer - Bank 1 INCERR1 flag clear bit"]
pub type ClrIncerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_OPERR1` reader - Bank 1 OPERR1 flag clear bit"]
pub type ClrOperr1R = crate::BitReader;
#[doc = "Field `CLR_OPERR1` writer - Bank 1 OPERR1 flag clear bit"]
pub type ClrOperr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RDPERR1` reader - Bank 1 RDPERR1 flag clear bit"]
pub type ClrRdperr1R = crate::BitReader;
#[doc = "Field `CLR_RDPERR1` writer - Bank 1 RDPERR1 flag clear bit"]
pub type ClrRdperr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_RDSERR1` reader - Bank 1 RDSERR1 flag clear bit"]
pub type ClrRdserr1R = crate::BitReader;
#[doc = "Field `CLR_RDSERR1` writer - Bank 1 RDSERR1 flag clear bit"]
pub type ClrRdserr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_SNECCERR1` reader - Bank 1 SNECCERR1 flag clear bit"]
pub type ClrSneccerr1R = crate::BitReader;
#[doc = "Field `CLR_SNECCERR1` writer - Bank 1 SNECCERR1 flag clear bit"]
pub type ClrSneccerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_DBECCERR1` reader - Bank 1 DBECCERR1 flag clear bit"]
pub type ClrDbeccerr1R = crate::BitReader;
#[doc = "Field `CLR_DBECCERR1` writer - Bank 1 DBECCERR1 flag clear bit"]
pub type ClrDbeccerr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLR_CRCEND1` reader - Bank 1 CRCEND1 flag clear bit"]
pub type ClrCrcend1R = crate::BitReader;
#[doc = "Field `CLR_CRCEND1` writer - Bank 1 CRCEND1 flag clear bit"]
pub type ClrCrcend1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop1(&self) -> ClrEop1R {
        ClrEop1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bank 1 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr1(&self) -> ClrWrperr1R {
        ClrWrperr1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bank 1 PGSERR1 flag clear bi"]
    #[inline(always)]
    pub fn clr_pgserr1(&self) -> ClrPgserr1R {
        ClrPgserr1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bank 1 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr1(&self) -> ClrStrberr1R {
        ClrStrberr1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - Bank 1 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr1(&self) -> ClrIncerr1R {
        ClrIncerr1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bank 1 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr1(&self) -> ClrOperr1R {
        ClrOperr1R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bank 1 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr1(&self) -> ClrRdperr1R {
        ClrRdperr1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr1(&self) -> ClrRdserr1R {
        ClrRdserr1R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bank 1 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr1(&self) -> ClrSneccerr1R {
        ClrSneccerr1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr1(&self) -> ClrDbeccerr1R {
        ClrDbeccerr1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bank 1 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend1(&self) -> ClrCrcend1R {
        ClrCrcend1R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Bank 1 EOP1 flag clear bit"]
    #[inline(always)]
    pub fn clr_eop1(&mut self) -> ClrEop1W<Ccr1Spec> {
        ClrEop1W::new(self, 16)
    }
    #[doc = "Bit 17 - Bank 1 WRPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_wrperr1(&mut self) -> ClrWrperr1W<Ccr1Spec> {
        ClrWrperr1W::new(self, 17)
    }
    #[doc = "Bit 18 - Bank 1 PGSERR1 flag clear bi"]
    #[inline(always)]
    pub fn clr_pgserr1(&mut self) -> ClrPgserr1W<Ccr1Spec> {
        ClrPgserr1W::new(self, 18)
    }
    #[doc = "Bit 19 - Bank 1 STRBERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_strberr1(&mut self) -> ClrStrberr1W<Ccr1Spec> {
        ClrStrberr1W::new(self, 19)
    }
    #[doc = "Bit 21 - Bank 1 INCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_incerr1(&mut self) -> ClrIncerr1W<Ccr1Spec> {
        ClrIncerr1W::new(self, 21)
    }
    #[doc = "Bit 22 - Bank 1 OPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_operr1(&mut self) -> ClrOperr1W<Ccr1Spec> {
        ClrOperr1W::new(self, 22)
    }
    #[doc = "Bit 23 - Bank 1 RDPERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdperr1(&mut self) -> ClrRdperr1W<Ccr1Spec> {
        ClrRdperr1W::new(self, 23)
    }
    #[doc = "Bit 24 - Bank 1 RDSERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_rdserr1(&mut self) -> ClrRdserr1W<Ccr1Spec> {
        ClrRdserr1W::new(self, 24)
    }
    #[doc = "Bit 25 - Bank 1 SNECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_sneccerr1(&mut self) -> ClrSneccerr1W<Ccr1Spec> {
        ClrSneccerr1W::new(self, 25)
    }
    #[doc = "Bit 26 - Bank 1 DBECCERR1 flag clear bit"]
    #[inline(always)]
    pub fn clr_dbeccerr1(&mut self) -> ClrDbeccerr1W<Ccr1Spec> {
        ClrDbeccerr1W::new(self, 26)
    }
    #[doc = "Bit 27 - Bank 1 CRCEND1 flag clear bit"]
    #[inline(always)]
    pub fn clr_crcend1(&mut self) -> ClrCrcend1W<Ccr1Spec> {
        ClrCrcend1W::new(self, 27)
    }
}
#[doc = "FLASH clear control register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr1Spec;
impl crate::RegisterSpec for Ccr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for Ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for Ccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR1 to value 0"]
impl crate::Resettable for Ccr1Spec {}

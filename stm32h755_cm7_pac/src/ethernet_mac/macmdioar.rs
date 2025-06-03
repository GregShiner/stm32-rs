#[doc = "Register `MACMDIOAR` reader"]
pub type R = crate::R<MacmdioarSpec>;
#[doc = "Register `MACMDIOAR` writer"]
pub type W = crate::W<MacmdioarSpec>;
#[doc = "Field `MB` reader - MB"]
pub type MbR = crate::BitReader;
#[doc = "Field `MB` writer - MB"]
pub type MbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `C45E` reader - C45E"]
pub type C45eR = crate::BitReader;
#[doc = "Field `C45E` writer - C45E"]
pub type C45eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GOC` reader - GOC"]
pub type GocR = crate::FieldReader;
#[doc = "Field `GOC` writer - GOC"]
pub type GocW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKAP` reader - SKAP"]
pub type SkapR = crate::BitReader;
#[doc = "Field `SKAP` writer - SKAP"]
pub type SkapW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CR` reader - CR"]
pub type CrR = crate::FieldReader;
#[doc = "Field `CR` writer - CR"]
pub type CrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NTC` reader - NTC"]
pub type NtcR = crate::FieldReader;
#[doc = "Field `NTC` writer - NTC"]
pub type NtcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RDA` reader - RDA"]
pub type RdaR = crate::FieldReader;
#[doc = "Field `RDA` writer - RDA"]
pub type RdaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PA` reader - PA"]
pub type PaR = crate::FieldReader;
#[doc = "Field `PA` writer - PA"]
pub type PaW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BTB` reader - BTB"]
pub type BtbR = crate::BitReader;
#[doc = "Field `BTB` writer - BTB"]
pub type BtbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSE` reader - PSE"]
pub type PseR = crate::BitReader;
#[doc = "Field `PSE` writer - PSE"]
pub type PseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MB"]
    #[inline(always)]
    pub fn mb(&self) -> MbR {
        MbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    pub fn c45e(&self) -> C45eR {
        C45eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    pub fn goc(&self) -> GocR {
        GocR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    pub fn skap(&self) -> SkapR {
        SkapR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    pub fn ntc(&self) -> NtcR {
        NtcR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    pub fn rda(&self) -> RdaR {
        RdaR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    pub fn pa(&self) -> PaR {
        PaR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    pub fn btb(&self) -> BtbR {
        BtbR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PSE"]
    #[inline(always)]
    pub fn pse(&self) -> PseR {
        PseR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MB"]
    #[inline(always)]
    pub fn mb(&mut self) -> MbW<MacmdioarSpec> {
        MbW::new(self, 0)
    }
    #[doc = "Bit 1 - C45E"]
    #[inline(always)]
    pub fn c45e(&mut self) -> C45eW<MacmdioarSpec> {
        C45eW::new(self, 1)
    }
    #[doc = "Bits 2:3 - GOC"]
    #[inline(always)]
    pub fn goc(&mut self) -> GocW<MacmdioarSpec> {
        GocW::new(self, 2)
    }
    #[doc = "Bit 4 - SKAP"]
    #[inline(always)]
    pub fn skap(&mut self) -> SkapW<MacmdioarSpec> {
        SkapW::new(self, 4)
    }
    #[doc = "Bits 8:11 - CR"]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<MacmdioarSpec> {
        CrW::new(self, 8)
    }
    #[doc = "Bits 12:14 - NTC"]
    #[inline(always)]
    pub fn ntc(&mut self) -> NtcW<MacmdioarSpec> {
        NtcW::new(self, 12)
    }
    #[doc = "Bits 16:20 - RDA"]
    #[inline(always)]
    pub fn rda(&mut self) -> RdaW<MacmdioarSpec> {
        RdaW::new(self, 16)
    }
    #[doc = "Bits 21:25 - PA"]
    #[inline(always)]
    pub fn pa(&mut self) -> PaW<MacmdioarSpec> {
        PaW::new(self, 21)
    }
    #[doc = "Bit 26 - BTB"]
    #[inline(always)]
    pub fn btb(&mut self) -> BtbW<MacmdioarSpec> {
        BtbW::new(self, 26)
    }
    #[doc = "Bit 27 - PSE"]
    #[inline(always)]
    pub fn pse(&mut self) -> PseW<MacmdioarSpec> {
        PseW::new(self, 27)
    }
}
#[doc = "MDIO address register\n\nYou can [`read`](crate::Reg::read) this register and get [`macmdioar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macmdioar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacmdioarSpec;
impl crate::RegisterSpec for MacmdioarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macmdioar::R`](R) reader structure"]
impl crate::Readable for MacmdioarSpec {}
#[doc = "`write(|w| ..)` method takes [`macmdioar::W`](W) writer structure"]
impl crate::Writable for MacmdioarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACMDIOAR to value 0"]
impl crate::Resettable for MacmdioarSpec {}

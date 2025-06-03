#[doc = "Register `MACPFR` reader"]
pub type R = crate::R<MacpfrSpec>;
#[doc = "Register `MACPFR` writer"]
pub type W = crate::W<MacpfrSpec>;
#[doc = "Field `PR` reader - PR"]
pub type PrR = crate::BitReader;
#[doc = "Field `PR` writer - PR"]
pub type PrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUC` reader - HUC"]
pub type HucR = crate::BitReader;
#[doc = "Field `HUC` writer - HUC"]
pub type HucW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HMC` reader - HMC"]
pub type HmcR = crate::BitReader;
#[doc = "Field `HMC` writer - HMC"]
pub type HmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAIF` reader - DAIF"]
pub type DaifR = crate::BitReader;
#[doc = "Field `DAIF` writer - DAIF"]
pub type DaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PM` reader - PM"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - PM"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBF` reader - DBF"]
pub type DbfR = crate::BitReader;
#[doc = "Field `DBF` writer - DBF"]
pub type DbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCF` reader - PCF"]
pub type PcfR = crate::FieldReader;
#[doc = "Field `PCF` writer - PCF"]
pub type PcfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAIF` reader - SAIF"]
pub type SaifR = crate::BitReader;
#[doc = "Field `SAIF` writer - SAIF"]
pub type SaifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAF` reader - SAF"]
pub type SafR = crate::BitReader;
#[doc = "Field `SAF` writer - SAF"]
pub type SafW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPF` reader - HPF"]
pub type HpfR = crate::BitReader;
#[doc = "Field `HPF` writer - HPF"]
pub type HpfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTFE` reader - VTFE"]
pub type VtfeR = crate::BitReader;
#[doc = "Field `VTFE` writer - VTFE"]
pub type VtfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPFE` reader - IPFE"]
pub type IpfeR = crate::BitReader;
#[doc = "Field `IPFE` writer - IPFE"]
pub type IpfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DNTU` reader - DNTU"]
pub type DntuR = crate::BitReader;
#[doc = "Field `DNTU` writer - DNTU"]
pub type DntuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RA` reader - RA"]
pub type RaR = crate::BitReader;
#[doc = "Field `RA` writer - RA"]
pub type RaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PR"]
    #[inline(always)]
    pub fn pr(&self) -> PrR {
        PrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - HUC"]
    #[inline(always)]
    pub fn huc(&self) -> HucR {
        HucR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HMC"]
    #[inline(always)]
    pub fn hmc(&self) -> HmcR {
        HmcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&self) -> DaifR {
        DaifR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DBF"]
    #[inline(always)]
    pub fn dbf(&self) -> DbfR {
        DbfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PCF"]
    #[inline(always)]
    pub fn pcf(&self) -> PcfR {
        PcfR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SAIF"]
    #[inline(always)]
    pub fn saif(&self) -> SaifR {
        SaifR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SAF"]
    #[inline(always)]
    pub fn saf(&self) -> SafR {
        SafR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HPF"]
    #[inline(always)]
    pub fn hpf(&self) -> HpfR {
        HpfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    pub fn vtfe(&self) -> VtfeR {
        VtfeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - IPFE"]
    #[inline(always)]
    pub fn ipfe(&self) -> IpfeR {
        IpfeR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DNTU"]
    #[inline(always)]
    pub fn dntu(&self) -> DntuR {
        DntuR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&self) -> RaR {
        RaR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PR"]
    #[inline(always)]
    pub fn pr(&mut self) -> PrW<MacpfrSpec> {
        PrW::new(self, 0)
    }
    #[doc = "Bit 1 - HUC"]
    #[inline(always)]
    pub fn huc(&mut self) -> HucW<MacpfrSpec> {
        HucW::new(self, 1)
    }
    #[doc = "Bit 2 - HMC"]
    #[inline(always)]
    pub fn hmc(&mut self) -> HmcW<MacpfrSpec> {
        HmcW::new(self, 2)
    }
    #[doc = "Bit 3 - DAIF"]
    #[inline(always)]
    pub fn daif(&mut self) -> DaifW<MacpfrSpec> {
        DaifW::new(self, 3)
    }
    #[doc = "Bit 4 - PM"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<MacpfrSpec> {
        PmW::new(self, 4)
    }
    #[doc = "Bit 5 - DBF"]
    #[inline(always)]
    pub fn dbf(&mut self) -> DbfW<MacpfrSpec> {
        DbfW::new(self, 5)
    }
    #[doc = "Bits 6:7 - PCF"]
    #[inline(always)]
    pub fn pcf(&mut self) -> PcfW<MacpfrSpec> {
        PcfW::new(self, 6)
    }
    #[doc = "Bit 8 - SAIF"]
    #[inline(always)]
    pub fn saif(&mut self) -> SaifW<MacpfrSpec> {
        SaifW::new(self, 8)
    }
    #[doc = "Bit 9 - SAF"]
    #[inline(always)]
    pub fn saf(&mut self) -> SafW<MacpfrSpec> {
        SafW::new(self, 9)
    }
    #[doc = "Bit 10 - HPF"]
    #[inline(always)]
    pub fn hpf(&mut self) -> HpfW<MacpfrSpec> {
        HpfW::new(self, 10)
    }
    #[doc = "Bit 16 - VTFE"]
    #[inline(always)]
    pub fn vtfe(&mut self) -> VtfeW<MacpfrSpec> {
        VtfeW::new(self, 16)
    }
    #[doc = "Bit 20 - IPFE"]
    #[inline(always)]
    pub fn ipfe(&mut self) -> IpfeW<MacpfrSpec> {
        IpfeW::new(self, 20)
    }
    #[doc = "Bit 21 - DNTU"]
    #[inline(always)]
    pub fn dntu(&mut self) -> DntuW<MacpfrSpec> {
        DntuW::new(self, 21)
    }
    #[doc = "Bit 31 - RA"]
    #[inline(always)]
    pub fn ra(&mut self) -> RaW<MacpfrSpec> {
        RaW::new(self, 31)
    }
}
#[doc = "Packet filtering control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macpfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macpfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacpfrSpec;
impl crate::RegisterSpec for MacpfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macpfr::R`](R) reader structure"]
impl crate::Readable for MacpfrSpec {}
#[doc = "`write(|w| ..)` method takes [`macpfr::W`](W) writer structure"]
impl crate::Writable for MacpfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACPFR to value 0"]
impl crate::Resettable for MacpfrSpec {}

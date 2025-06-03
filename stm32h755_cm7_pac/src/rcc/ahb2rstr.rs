#[doc = "Register `AHB2RSTR` reader"]
pub type R = crate::R<Ahb2rstrSpec>;
#[doc = "Register `AHB2RSTR` writer"]
pub type W = crate::W<Ahb2rstrSpec>;
#[doc = "Field `CAMITFRST` reader - CAMITF block reset"]
pub type CamitfrstR = crate::BitReader;
#[doc = "Field `CAMITFRST` writer - CAMITF block reset"]
pub type CamitfrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTRST` reader - Cryptography block reset"]
pub type CryptrstR = crate::BitReader;
#[doc = "Field `CRYPTRST` writer - Cryptography block reset"]
pub type CryptrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHRST` reader - Hash block reset"]
pub type HashrstR = crate::BitReader;
#[doc = "Field `HASHRST` writer - Hash block reset"]
pub type HashrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGRST` reader - Random Number Generator block reset"]
pub type RngrstR = crate::BitReader;
#[doc = "Field `RNGRST` writer - Random Number Generator block reset"]
pub type RngrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2RST` reader - SDMMC2 and SDMMC2 Delay block reset"]
pub type Sdmmc2rstR = crate::BitReader;
#[doc = "Field `SDMMC2RST` writer - SDMMC2 and SDMMC2 Delay block reset"]
pub type Sdmmc2rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    pub fn camitfrst(&self) -> CamitfrstR {
        CamitfrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    pub fn cryptrst(&self) -> CryptrstR {
        CryptrstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    pub fn hashrst(&self) -> HashrstR {
        HashrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    pub fn rngrst(&self) -> RngrstR {
        RngrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    pub fn sdmmc2rst(&self) -> Sdmmc2rstR {
        Sdmmc2rstR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF block reset"]
    #[inline(always)]
    pub fn camitfrst(&mut self) -> CamitfrstW<Ahb2rstrSpec> {
        CamitfrstW::new(self, 0)
    }
    #[doc = "Bit 4 - Cryptography block reset"]
    #[inline(always)]
    pub fn cryptrst(&mut self) -> CryptrstW<Ahb2rstrSpec> {
        CryptrstW::new(self, 4)
    }
    #[doc = "Bit 5 - Hash block reset"]
    #[inline(always)]
    pub fn hashrst(&mut self) -> HashrstW<Ahb2rstrSpec> {
        HashrstW::new(self, 5)
    }
    #[doc = "Bit 6 - Random Number Generator block reset"]
    #[inline(always)]
    pub fn rngrst(&mut self) -> RngrstW<Ahb2rstrSpec> {
        RngrstW::new(self, 6)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay block reset"]
    #[inline(always)]
    pub fn sdmmc2rst(&mut self) -> Sdmmc2rstW<Ahb2rstrSpec> {
        Sdmmc2rstW::new(self, 9)
    }
}
#[doc = "RCC AHB2 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2rstrSpec;
impl crate::RegisterSpec for Ahb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2rstr::R`](R) reader structure"]
impl crate::Readable for Ahb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2rstr::W`](W) writer structure"]
impl crate::Writable for Ahb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2RSTR to value 0"]
impl crate::Resettable for Ahb2rstrSpec {}

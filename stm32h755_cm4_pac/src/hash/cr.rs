#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `INIT` writer - Initialize message digest calculation"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAE` reader - DMA enable"]
pub type DmaeR = crate::BitReader;
#[doc = "Field `DMAE` writer - DMA enable"]
pub type DmaeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATYPE` reader - Data type selection"]
pub type DatatypeR = crate::FieldReader;
#[doc = "Field `DATATYPE` writer - Data type selection"]
pub type DatatypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MODE` reader - Mode selection"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - Mode selection"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGO0` reader - Algorithm selection"]
pub type Algo0R = crate::BitReader;
#[doc = "Field `ALGO0` writer - Algorithm selection"]
pub type Algo0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NBW` reader - Number of words already pushed"]
pub type NbwR = crate::FieldReader;
#[doc = "Field `DINNE` reader - DIN not empty"]
pub type DinneR = crate::BitReader;
#[doc = "Field `MDMAT` reader - Multiple DMA Transfers"]
pub type MdmatR = crate::BitReader;
#[doc = "Field `MDMAT` writer - Multiple DMA Transfers"]
pub type MdmatW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LKEY` reader - Long key selection"]
pub type LkeyR = crate::BitReader;
#[doc = "Field `LKEY` writer - Long key selection"]
pub type LkeyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALGO1` reader - ALGO"]
pub type Algo1R = crate::BitReader;
#[doc = "Field `ALGO1` writer - ALGO"]
pub type Algo1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&self) -> DmaeR {
        DmaeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&self) -> DatatypeR {
        DatatypeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Mode selection"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Algorithm selection"]
    #[inline(always)]
    pub fn algo0(&self) -> Algo0R {
        Algo0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of words already pushed"]
    #[inline(always)]
    pub fn nbw(&self) -> NbwR {
        NbwR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - DIN not empty"]
    #[inline(always)]
    pub fn dinne(&self) -> DinneR {
        DinneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Multiple DMA Transfers"]
    #[inline(always)]
    pub fn mdmat(&self) -> MdmatR {
        MdmatR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Long key selection"]
    #[inline(always)]
    pub fn lkey(&self) -> LkeyR {
        LkeyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - ALGO"]
    #[inline(always)]
    pub fn algo1(&self) -> Algo1R {
        Algo1R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Initialize message digest calculation"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<CrSpec> {
        InitW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA enable"]
    #[inline(always)]
    pub fn dmae(&mut self) -> DmaeW<CrSpec> {
        DmaeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Data type selection"]
    #[inline(always)]
    pub fn datatype(&mut self) -> DatatypeW<CrSpec> {
        DatatypeW::new(self, 4)
    }
    #[doc = "Bit 6 - Mode selection"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<CrSpec> {
        ModeW::new(self, 6)
    }
    #[doc = "Bit 7 - Algorithm selection"]
    #[inline(always)]
    pub fn algo0(&mut self) -> Algo0W<CrSpec> {
        Algo0W::new(self, 7)
    }
    #[doc = "Bit 13 - Multiple DMA Transfers"]
    #[inline(always)]
    pub fn mdmat(&mut self) -> MdmatW<CrSpec> {
        MdmatW::new(self, 13)
    }
    #[doc = "Bit 16 - Long key selection"]
    #[inline(always)]
    pub fn lkey(&mut self) -> LkeyW<CrSpec> {
        LkeyW::new(self, 16)
    }
    #[doc = "Bit 18 - ALGO"]
    #[inline(always)]
    pub fn algo1(&mut self) -> Algo1W<CrSpec> {
        Algo1W::new(self, 18)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}

#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `MSWU` reader - Master Timer Software update"]
pub type MswuR = crate::BitReader;
#[doc = "Field `MSWU` writer - Master Timer Software update"]
pub type MswuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TASWU` reader - Timer A Software update"]
pub type TaswuR = crate::BitReader;
#[doc = "Field `TASWU` writer - Timer A Software update"]
pub type TaswuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBSWU` reader - Timer B Software Update"]
pub type TbswuR = crate::BitReader;
#[doc = "Field `TBSWU` writer - Timer B Software Update"]
pub type TbswuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCSWU` reader - Timer C Software Update"]
pub type TcswuR = crate::BitReader;
#[doc = "Field `TCSWU` writer - Timer C Software Update"]
pub type TcswuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDSWU` reader - Timer D Software Update"]
pub type TdswuR = crate::BitReader;
#[doc = "Field `TDSWU` writer - Timer D Software Update"]
pub type TdswuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESWU` reader - Timer E Software Update"]
pub type TeswuR = crate::BitReader;
#[doc = "Field `TESWU` writer - Timer E Software Update"]
pub type TeswuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRST` reader - Master Counter software reset"]
pub type MrstR = crate::BitReader;
#[doc = "Field `MRST` writer - Master Counter software reset"]
pub type MrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TARST` reader - Timer A counter software reset"]
pub type TarstR = crate::BitReader;
#[doc = "Field `TARST` writer - Timer A counter software reset"]
pub type TarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBRST` reader - Timer B counter software reset"]
pub type TbrstR = crate::BitReader;
#[doc = "Field `TBRST` writer - Timer B counter software reset"]
pub type TbrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCRST` reader - Timer C counter software reset"]
pub type TcrstR = crate::BitReader;
#[doc = "Field `TCRST` writer - Timer C counter software reset"]
pub type TcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDRST` reader - Timer D counter software reset"]
pub type TdrstR = crate::BitReader;
#[doc = "Field `TDRST` writer - Timer D counter software reset"]
pub type TdrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERST` reader - Timer E counter software reset"]
pub type TerstR = crate::BitReader;
#[doc = "Field `TERST` writer - Timer E counter software reset"]
pub type TerstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&self) -> MswuR {
        MswuR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&self) -> TaswuR {
        TaswuR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&self) -> TbswuR {
        TbswuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&self) -> TcswuR {
        TcswuR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&self) -> TdswuR {
        TdswuR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&self) -> TeswuR {
        TeswuR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&self) -> MrstR {
        MrstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&self) -> TarstR {
        TarstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&self) -> TbrstR {
        TbrstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&self) -> TcrstR {
        TcrstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&self) -> TdrstR {
        TdrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&self) -> TerstR {
        TerstR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Timer Software update"]
    #[inline(always)]
    pub fn mswu(&mut self) -> MswuW<Cr2Spec> {
        MswuW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Software update"]
    #[inline(always)]
    pub fn taswu(&mut self) -> TaswuW<Cr2Spec> {
        TaswuW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B Software Update"]
    #[inline(always)]
    pub fn tbswu(&mut self) -> TbswuW<Cr2Spec> {
        TbswuW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer C Software Update"]
    #[inline(always)]
    pub fn tcswu(&mut self) -> TcswuW<Cr2Spec> {
        TcswuW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer D Software Update"]
    #[inline(always)]
    pub fn tdswu(&mut self) -> TdswuW<Cr2Spec> {
        TdswuW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer E Software Update"]
    #[inline(always)]
    pub fn teswu(&mut self) -> TeswuW<Cr2Spec> {
        TeswuW::new(self, 5)
    }
    #[doc = "Bit 8 - Master Counter software reset"]
    #[inline(always)]
    pub fn mrst(&mut self) -> MrstW<Cr2Spec> {
        MrstW::new(self, 8)
    }
    #[doc = "Bit 9 - Timer A counter software reset"]
    #[inline(always)]
    pub fn tarst(&mut self) -> TarstW<Cr2Spec> {
        TarstW::new(self, 9)
    }
    #[doc = "Bit 10 - Timer B counter software reset"]
    #[inline(always)]
    pub fn tbrst(&mut self) -> TbrstW<Cr2Spec> {
        TbrstW::new(self, 10)
    }
    #[doc = "Bit 11 - Timer C counter software reset"]
    #[inline(always)]
    pub fn tcrst(&mut self) -> TcrstW<Cr2Spec> {
        TcrstW::new(self, 11)
    }
    #[doc = "Bit 12 - Timer D counter software reset"]
    #[inline(always)]
    pub fn tdrst(&mut self) -> TdrstW<Cr2Spec> {
        TdrstW::new(self, 12)
    }
    #[doc = "Bit 13 - Timer E counter software reset"]
    #[inline(always)]
    pub fn terst(&mut self) -> TerstW<Cr2Spec> {
        TerstW::new(self, 13)
    }
}
#[doc = "Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

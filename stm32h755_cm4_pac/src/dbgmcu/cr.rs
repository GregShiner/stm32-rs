#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `DBGSLPD1` reader - Allow D1 domain debug in Sleep mode"]
pub type Dbgslpd1R = crate::BitReader;
#[doc = "Field `DBGSLPD1` writer - Allow D1 domain debug in Sleep mode"]
pub type Dbgslpd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTPD1` reader - Allow D1 domain debug in Stop mode"]
pub type Dbgstpd1R = crate::BitReader;
#[doc = "Field `DBGSTPD1` writer - Allow D1 domain debug in Stop mode"]
pub type Dbgstpd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTBD1` reader - Allow D1 domain debug in Standby mode"]
pub type Dbgstbd1R = crate::BitReader;
#[doc = "Field `DBGSTBD1` writer - Allow D1 domain debug in Standby mode"]
pub type Dbgstbd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSLPD2` reader - Allow D2 domain debug in Sleep mode"]
pub type Dbgslpd2R = crate::BitReader;
#[doc = "Field `DBGSLPD2` writer - Allow D2 domain debug in Sleep mode"]
pub type Dbgslpd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTPD2` reader - Allow D2 domain debug in Stop mode"]
pub type Dbgstpd2R = crate::BitReader;
#[doc = "Field `DBGSTPD2` writer - Allow D2 domain debug in Stop mode"]
pub type Dbgstpd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTBD2` reader - Allow D2 domain debug in Standby mode"]
pub type Dbgstbd2R = crate::BitReader;
#[doc = "Field `DBGSTBD2` writer - Allow D2 domain debug in Standby mode"]
pub type Dbgstbd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTPD3` reader - Allow debug in D3 Stop mode"]
pub type Dbgstpd3R = crate::BitReader;
#[doc = "Field `DBGSTPD3` writer - Allow debug in D3 Stop mode"]
pub type Dbgstpd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBGSTBD3` reader - Allow debug in D3 Standby mode"]
pub type Dbgstbd3R = crate::BitReader;
#[doc = "Field `DBGSTBD3` writer - Allow debug in D3 Standby mode"]
pub type Dbgstbd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACECLKEN` reader - Trace port clock enable"]
pub type TraceclkenR = crate::BitReader;
#[doc = "Field `TRACECLKEN` writer - Trace port clock enable"]
pub type TraceclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1DBGCKEN` reader - D1 debug clock enable"]
pub type D1dbgckenR = crate::BitReader;
#[doc = "Field `D1DBGCKEN` writer - D1 debug clock enable"]
pub type D1dbgckenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D3DBGCKEN` reader - D3 debug clock enable"]
pub type D3dbgckenR = crate::BitReader;
#[doc = "Field `D3DBGCKEN` writer - D3 debug clock enable"]
pub type D3dbgckenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRGOEN` reader - External trigger output enable"]
pub type TrgoenR = crate::BitReader;
#[doc = "Field `TRGOEN` writer - External trigger output enable"]
pub type TrgoenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd1(&self) -> Dbgslpd1R {
        Dbgslpd1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd1(&self) -> Dbgstpd1R {
        Dbgstpd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd1(&self) -> Dbgstbd1R {
        Dbgstbd1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Allow D2 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd2(&self) -> Dbgslpd2R {
        Dbgslpd2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Allow D2 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd2(&self) -> Dbgstpd2R {
        Dbgstpd2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Allow D2 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd2(&self) -> Dbgstbd2R {
        Dbgstbd2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Allow debug in D3 Stop mode"]
    #[inline(always)]
    pub fn dbgstpd3(&self) -> Dbgstpd3R {
        Dbgstpd3R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Allow debug in D3 Standby mode"]
    #[inline(always)]
    pub fn dbgstbd3(&self) -> Dbgstbd3R {
        Dbgstbd3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&self) -> TraceclkenR {
        TraceclkenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    pub fn d1dbgcken(&self) -> D1dbgckenR {
        D1dbgckenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    pub fn d3dbgcken(&self) -> D3dbgckenR {
        D3dbgckenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&self) -> TrgoenR {
        TrgoenR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Allow D1 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd1(&mut self) -> Dbgslpd1W<CrSpec> {
        Dbgslpd1W::new(self, 0)
    }
    #[doc = "Bit 1 - Allow D1 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd1(&mut self) -> Dbgstpd1W<CrSpec> {
        Dbgstpd1W::new(self, 1)
    }
    #[doc = "Bit 2 - Allow D1 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd1(&mut self) -> Dbgstbd1W<CrSpec> {
        Dbgstbd1W::new(self, 2)
    }
    #[doc = "Bit 3 - Allow D2 domain debug in Sleep mode"]
    #[inline(always)]
    pub fn dbgslpd2(&mut self) -> Dbgslpd2W<CrSpec> {
        Dbgslpd2W::new(self, 3)
    }
    #[doc = "Bit 4 - Allow D2 domain debug in Stop mode"]
    #[inline(always)]
    pub fn dbgstpd2(&mut self) -> Dbgstpd2W<CrSpec> {
        Dbgstpd2W::new(self, 4)
    }
    #[doc = "Bit 5 - Allow D2 domain debug in Standby mode"]
    #[inline(always)]
    pub fn dbgstbd2(&mut self) -> Dbgstbd2W<CrSpec> {
        Dbgstbd2W::new(self, 5)
    }
    #[doc = "Bit 7 - Allow debug in D3 Stop mode"]
    #[inline(always)]
    pub fn dbgstpd3(&mut self) -> Dbgstpd3W<CrSpec> {
        Dbgstpd3W::new(self, 7)
    }
    #[doc = "Bit 8 - Allow debug in D3 Standby mode"]
    #[inline(always)]
    pub fn dbgstbd3(&mut self) -> Dbgstbd3W<CrSpec> {
        Dbgstbd3W::new(self, 8)
    }
    #[doc = "Bit 20 - Trace port clock enable"]
    #[inline(always)]
    pub fn traceclken(&mut self) -> TraceclkenW<CrSpec> {
        TraceclkenW::new(self, 20)
    }
    #[doc = "Bit 21 - D1 debug clock enable"]
    #[inline(always)]
    pub fn d1dbgcken(&mut self) -> D1dbgckenW<CrSpec> {
        D1dbgckenW::new(self, 21)
    }
    #[doc = "Bit 22 - D3 debug clock enable"]
    #[inline(always)]
    pub fn d3dbgcken(&mut self) -> D3dbgckenW<CrSpec> {
        D3dbgckenW::new(self, 22)
    }
    #[doc = "Bit 28 - External trigger output enable"]
    #[inline(always)]
    pub fn trgoen(&mut self) -> TrgoenW<CrSpec> {
        TrgoenW::new(self, 28)
    }
}
#[doc = "DBGMCU Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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

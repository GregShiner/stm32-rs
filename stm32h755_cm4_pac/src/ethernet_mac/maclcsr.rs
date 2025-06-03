#[doc = "Register `MACLCSR` reader"]
pub type R = crate::R<MaclcsrSpec>;
#[doc = "Register `MACLCSR` writer"]
pub type W = crate::W<MaclcsrSpec>;
#[doc = "Field `TLPIEN` reader - TLPIEN"]
pub type TlpienR = crate::BitReader;
#[doc = "Field `TLPIEX` reader - TLPIEX"]
pub type TlpiexR = crate::BitReader;
#[doc = "Field `RLPIEN` reader - RLPIEN"]
pub type RlpienR = crate::BitReader;
#[doc = "Field `RLPIEX` reader - RLPIEX"]
pub type RlpiexR = crate::BitReader;
#[doc = "Field `TLPIST` reader - TLPIST"]
pub type TlpistR = crate::BitReader;
#[doc = "Field `RLPIST` reader - RLPIST"]
pub type RlpistR = crate::BitReader;
#[doc = "Field `LPIEN` reader - LPIEN"]
pub type LpienR = crate::BitReader;
#[doc = "Field `LPIEN` writer - LPIEN"]
pub type LpienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PLS"]
pub type PlsR = crate::BitReader;
#[doc = "Field `PLS` writer - PLS"]
pub type PlsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLSEN` reader - PLSEN"]
pub type PlsenR = crate::BitReader;
#[doc = "Field `PLSEN` writer - PLSEN"]
pub type PlsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITXA` reader - LPITXA"]
pub type LpitxaR = crate::BitReader;
#[doc = "Field `LPITXA` writer - LPITXA"]
pub type LpitxaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITE` reader - LPITE"]
pub type LpiteR = crate::BitReader;
#[doc = "Field `LPITE` writer - LPITE"]
pub type LpiteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITCSE` reader - LPITCSE"]
pub type LpitcseR = crate::BitReader;
#[doc = "Field `LPITCSE` writer - LPITCSE"]
pub type LpitcseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TLPIEN"]
    #[inline(always)]
    pub fn tlpien(&self) -> TlpienR {
        TlpienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TLPIEX"]
    #[inline(always)]
    pub fn tlpiex(&self) -> TlpiexR {
        TlpiexR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RLPIEN"]
    #[inline(always)]
    pub fn rlpien(&self) -> RlpienR {
        RlpienR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RLPIEX"]
    #[inline(always)]
    pub fn rlpiex(&self) -> RlpiexR {
        RlpiexR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - TLPIST"]
    #[inline(always)]
    pub fn tlpist(&self) -> TlpistR {
        TlpistR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RLPIST"]
    #[inline(always)]
    pub fn rlpist(&self) -> RlpistR {
        RlpistR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LPIEN"]
    #[inline(always)]
    pub fn lpien(&self) -> LpienR {
        LpienR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLS"]
    #[inline(always)]
    pub fn pls(&self) -> PlsR {
        PlsR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLSEN"]
    #[inline(always)]
    pub fn plsen(&self) -> PlsenR {
        PlsenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPITXA"]
    #[inline(always)]
    pub fn lpitxa(&self) -> LpitxaR {
        LpitxaR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - LPITE"]
    #[inline(always)]
    pub fn lpite(&self) -> LpiteR {
        LpiteR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPITCSE"]
    #[inline(always)]
    pub fn lpitcse(&self) -> LpitcseR {
        LpitcseR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPIEN"]
    #[inline(always)]
    pub fn lpien(&mut self) -> LpienW<MaclcsrSpec> {
        LpienW::new(self, 16)
    }
    #[doc = "Bit 17 - PLS"]
    #[inline(always)]
    pub fn pls(&mut self) -> PlsW<MaclcsrSpec> {
        PlsW::new(self, 17)
    }
    #[doc = "Bit 18 - PLSEN"]
    #[inline(always)]
    pub fn plsen(&mut self) -> PlsenW<MaclcsrSpec> {
        PlsenW::new(self, 18)
    }
    #[doc = "Bit 19 - LPITXA"]
    #[inline(always)]
    pub fn lpitxa(&mut self) -> LpitxaW<MaclcsrSpec> {
        LpitxaW::new(self, 19)
    }
    #[doc = "Bit 20 - LPITE"]
    #[inline(always)]
    pub fn lpite(&mut self) -> LpiteW<MaclcsrSpec> {
        LpiteW::new(self, 20)
    }
    #[doc = "Bit 21 - LPITCSE"]
    #[inline(always)]
    pub fn lpitcse(&mut self) -> LpitcseW<MaclcsrSpec> {
        LpitcseW::new(self, 21)
    }
}
#[doc = "LPI control status register\n\nYou can [`read`](crate::Reg::read) this register and get [`maclcsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maclcsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaclcsrSpec;
impl crate::RegisterSpec for MaclcsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maclcsr::R`](R) reader structure"]
impl crate::Readable for MaclcsrSpec {}
#[doc = "`write(|w| ..)` method takes [`maclcsr::W`](W) writer structure"]
impl crate::Writable for MaclcsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACLCSR to value 0"]
impl crate::Resettable for MaclcsrSpec {}

#[doc = "Register `MACVTR` reader"]
pub type R = crate::R<MacvtrSpec>;
#[doc = "Register `MACVTR` writer"]
pub type W = crate::W<MacvtrSpec>;
#[doc = "Field `VL` reader - VL"]
pub type VlR = crate::FieldReader<u16>;
#[doc = "Field `VL` writer - VL"]
pub type VlW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ETV` reader - ETV"]
pub type EtvR = crate::BitReader;
#[doc = "Field `ETV` writer - ETV"]
pub type EtvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTIM` reader - VTIM"]
pub type VtimR = crate::BitReader;
#[doc = "Field `VTIM` writer - VTIM"]
pub type VtimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESVL` reader - ESVL"]
pub type EsvlR = crate::BitReader;
#[doc = "Field `ESVL` writer - ESVL"]
pub type EsvlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERSVLM` reader - ERSVLM"]
pub type ErsvlmR = crate::BitReader;
#[doc = "Field `ERSVLM` writer - ERSVLM"]
pub type ErsvlmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOVLTC` reader - DOVLTC"]
pub type DovltcR = crate::BitReader;
#[doc = "Field `DOVLTC` writer - DOVLTC"]
pub type DovltcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVLS` reader - EVLS"]
pub type EvlsR = crate::FieldReader;
#[doc = "Field `EVLS` writer - EVLS"]
pub type EvlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EVLRXS` reader - EVLRXS"]
pub type EvlrxsR = crate::BitReader;
#[doc = "Field `EVLRXS` writer - EVLRXS"]
pub type EvlrxsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VTHM` reader - VTHM"]
pub type VthmR = crate::BitReader;
#[doc = "Field `VTHM` writer - VTHM"]
pub type VthmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDVLP` reader - EDVLP"]
pub type EdvlpR = crate::BitReader;
#[doc = "Field `EDVLP` writer - EDVLP"]
pub type EdvlpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIVLT` reader - ERIVLT"]
pub type ErivltR = crate::BitReader;
#[doc = "Field `ERIVLT` writer - ERIVLT"]
pub type ErivltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIVLS` reader - EIVLS"]
pub type EivlsR = crate::FieldReader;
#[doc = "Field `EIVLS` writer - EIVLS"]
pub type EivlsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EIVLRXS` reader - EIVLRXS"]
pub type EivlrxsR = crate::BitReader;
#[doc = "Field `EIVLRXS` writer - EIVLRXS"]
pub type EivlrxsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - VL"]
    #[inline(always)]
    pub fn vl(&self) -> VlR {
        VlR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - ETV"]
    #[inline(always)]
    pub fn etv(&self) -> EtvR {
        EtvR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - VTIM"]
    #[inline(always)]
    pub fn vtim(&self) -> VtimR {
        VtimR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ESVL"]
    #[inline(always)]
    pub fn esvl(&self) -> EsvlR {
        EsvlR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ERSVLM"]
    #[inline(always)]
    pub fn ersvlm(&self) -> ErsvlmR {
        ErsvlmR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DOVLTC"]
    #[inline(always)]
    pub fn dovltc(&self) -> DovltcR {
        DovltcR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - EVLS"]
    #[inline(always)]
    pub fn evls(&self) -> EvlsR {
        EvlsR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 24 - EVLRXS"]
    #[inline(always)]
    pub fn evlrxs(&self) -> EvlrxsR {
        EvlrxsR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VTHM"]
    #[inline(always)]
    pub fn vthm(&self) -> VthmR {
        VthmR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - EDVLP"]
    #[inline(always)]
    pub fn edvlp(&self) -> EdvlpR {
        EdvlpR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - ERIVLT"]
    #[inline(always)]
    pub fn erivlt(&self) -> ErivltR {
        ErivltR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - EIVLS"]
    #[inline(always)]
    pub fn eivls(&self) -> EivlsR {
        EivlsR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - EIVLRXS"]
    #[inline(always)]
    pub fn eivlrxs(&self) -> EivlrxsR {
        EivlrxsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - VL"]
    #[inline(always)]
    pub fn vl(&mut self) -> VlW<MacvtrSpec> {
        VlW::new(self, 0)
    }
    #[doc = "Bit 16 - ETV"]
    #[inline(always)]
    pub fn etv(&mut self) -> EtvW<MacvtrSpec> {
        EtvW::new(self, 16)
    }
    #[doc = "Bit 17 - VTIM"]
    #[inline(always)]
    pub fn vtim(&mut self) -> VtimW<MacvtrSpec> {
        VtimW::new(self, 17)
    }
    #[doc = "Bit 18 - ESVL"]
    #[inline(always)]
    pub fn esvl(&mut self) -> EsvlW<MacvtrSpec> {
        EsvlW::new(self, 18)
    }
    #[doc = "Bit 19 - ERSVLM"]
    #[inline(always)]
    pub fn ersvlm(&mut self) -> ErsvlmW<MacvtrSpec> {
        ErsvlmW::new(self, 19)
    }
    #[doc = "Bit 20 - DOVLTC"]
    #[inline(always)]
    pub fn dovltc(&mut self) -> DovltcW<MacvtrSpec> {
        DovltcW::new(self, 20)
    }
    #[doc = "Bits 21:22 - EVLS"]
    #[inline(always)]
    pub fn evls(&mut self) -> EvlsW<MacvtrSpec> {
        EvlsW::new(self, 21)
    }
    #[doc = "Bit 24 - EVLRXS"]
    #[inline(always)]
    pub fn evlrxs(&mut self) -> EvlrxsW<MacvtrSpec> {
        EvlrxsW::new(self, 24)
    }
    #[doc = "Bit 25 - VTHM"]
    #[inline(always)]
    pub fn vthm(&mut self) -> VthmW<MacvtrSpec> {
        VthmW::new(self, 25)
    }
    #[doc = "Bit 26 - EDVLP"]
    #[inline(always)]
    pub fn edvlp(&mut self) -> EdvlpW<MacvtrSpec> {
        EdvlpW::new(self, 26)
    }
    #[doc = "Bit 27 - ERIVLT"]
    #[inline(always)]
    pub fn erivlt(&mut self) -> ErivltW<MacvtrSpec> {
        ErivltW::new(self, 27)
    }
    #[doc = "Bits 28:29 - EIVLS"]
    #[inline(always)]
    pub fn eivls(&mut self) -> EivlsW<MacvtrSpec> {
        EivlsW::new(self, 28)
    }
    #[doc = "Bit 31 - EIVLRXS"]
    #[inline(always)]
    pub fn eivlrxs(&mut self) -> EivlrxsW<MacvtrSpec> {
        EivlrxsW::new(self, 31)
    }
}
#[doc = "VLAN tag register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacvtrSpec;
impl crate::RegisterSpec for MacvtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvtr::R`](R) reader structure"]
impl crate::Readable for MacvtrSpec {}
#[doc = "`write(|w| ..)` method takes [`macvtr::W`](W) writer structure"]
impl crate::Writable for MacvtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACVTR to value 0"]
impl crate::Resettable for MacvtrSpec {}

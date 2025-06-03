#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `SEL` reader - Select the phase for the Output clock"]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - Select the phase for the Output clock"]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `UNIT` reader - Delay Defines the delay of a Unit delay cell"]
pub type UnitR = crate::FieldReader;
#[doc = "Field `UNIT` writer - Delay Defines the delay of a Unit delay cell"]
pub type UnitW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LNG` reader - Delay line length value"]
pub type LngR = crate::FieldReader<u16>;
#[doc = "Field `LNG` writer - Delay line length value"]
pub type LngW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `LNGF` reader - Length valid flag"]
pub type LngfR = crate::BitReader;
#[doc = "Field `LNGF` writer - Length valid flag"]
pub type LngfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Select the phase for the Output clock"]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Delay Defines the delay of a Unit delay cell"]
    #[inline(always)]
    pub fn unit(&self) -> UnitR {
        UnitR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:27 - Delay line length value"]
    #[inline(always)]
    pub fn lng(&self) -> LngR {
        LngR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bit 31 - Length valid flag"]
    #[inline(always)]
    pub fn lngf(&self) -> LngfR {
        LngfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Select the phase for the Output clock"]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<CfgrSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Delay Defines the delay of a Unit delay cell"]
    #[inline(always)]
    pub fn unit(&mut self) -> UnitW<CfgrSpec> {
        UnitW::new(self, 8)
    }
    #[doc = "Bits 16:27 - Delay line length value"]
    #[inline(always)]
    pub fn lng(&mut self) -> LngW<CfgrSpec> {
        LngW::new(self, 16)
    }
    #[doc = "Bit 31 - Length valid flag"]
    #[inline(always)]
    pub fn lngf(&mut self) -> LngfW<CfgrSpec> {
        LngfW::new(self, 31)
    }
}
#[doc = "DLYB configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
